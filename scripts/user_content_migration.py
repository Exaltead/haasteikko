# User Content Migration Tool for User ID Change

"""
This script migrates all content for a single user when their user ID changes in Cosmos DB.
- Recreates all library items for the user with the new user ID.
- Recreates all related answers, updating references to the new user ID and the new library item IDs.
- ld user ID, and new user ID are read from command line.
"""

import argparse
import logging
import sys
from typing import Dict
from azure.cosmos import CosmosClient, PartitionKey
import uuid

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s %(levelname)s %(message)s')
logging.getLogger('azure.cosmos').setLevel(logging.WARNING)

def parse_args():
    parser = argparse.ArgumentParser(description="Migrate user content for user ID change.")
    parser.add_argument('--db', required=True, help='Cosmos DB database name')
    parser.add_argument('--old_user', required=True, help='Old user ID')
    parser.add_argument('--new_user', required=True, help='New user ID')
    return parser.parse_args()

def get_container(client, db_name, container_name):
    db = client.get_database_client(db_name)
    return db.get_container_client(container_name)

def migrate_library_items(library_container, old_user_id, new_user_id) -> Dict[str, str]:
    logging.info(f"Fetching library items for user {old_user_id}")
    query = f"SELECT * FROM c WHERE c.userId = @userId"
    items = list(library_container.query_items(
        query=query,
        parameters=[{"name": "@userId", "value": old_user_id}],
        enable_cross_partition_query=False
    ))
    id_map = {}
    for item in items:
        old_id = item['id']
        new_id = str(uuid.uuid4())
        new_item = item.copy()
        new_item['id'] = new_id
        new_item['userId'] = new_user_id
        try:
            library_container.create_item(body=new_item)
            id_map[old_id] = new_id
            logging.info(f"Migrated library item {old_id} -> {new_id}")
        except Exception as e:
            logging.error(f"Failed to migrate library item {old_id}: {e}")
    return id_map

def migrate_answers(answers_container, old_user_id, new_user_id, id_map):
    logging.info(f"Fetching answers for user {old_user_id}")
    query = f"SELECT * FROM c WHERE c.userId = @userId"
    answers = list(answers_container.query_items(
        query=query,
        parameters=[{"name": "@userId", "value": old_user_id}],
        enable_cross_partition_query=False
    ))
    for ans in answers:
        new_ans = ans.copy()
        new_ans['userId'] = new_user_id
        # Update libraryItemId reference if present
        if 'itemId' in new_ans and new_ans['itemId'] in id_map:
            new_ans['itemId'] = id_map[new_ans['itemId']]
        else:
            logging.warning(f"Answer {ans['id']} references a library item not found in the migration map.")
        new_ans['id'] = str(uuid.uuid4())
        try:
            answers_container.create_item(body=new_ans)
            logging.info(f"Migrated answer {ans['id']} -> {new_ans['id']}")
        except Exception as e:
            logging.error(f"Failed to migrate answer {ans['id']}: {e}")

def main():
    args = parse_args()
    try:
        connection_string = "SETME"
        client = CosmosClient.from_connection_string(conn_str=connection_string)
        library_container = get_container(client, args.db, 'library')
        answers_container = get_container(client, args.db, 'answers')
        id_map = migrate_library_items(library_container, args.old_user, args.new_user)
        migrate_answers(answers_container, args.old_user, args.new_user, id_map)
        logging.info("Migration completed successfully.")
    except Exception as e:
        logging.error(f"Migration failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
