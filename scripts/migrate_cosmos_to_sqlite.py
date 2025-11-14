import argparse
import json
import logging
import os
import sqlite3
import sys
from typing import Any, Dict, Iterable, List
from dataclasses import dataclass

from azure.cosmos import CosmosClient, DatabaseProxy

logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")
logger = logging.getLogger('azure')

# Set the desired logging level
logger.setLevel(logging.WARN)

@dataclass
class Question: 
    kind: str
    question: str
    id: str
    number: int
    question_cluster_size: int

@dataclass
class Challenge:
    id: str
    name: str
    status: str
    target_media: str
    questions: list[Question]

@dataclass
class LibraryItem:
    kind: str
    id: str
    user_id: str
    added_at: str
    activated_challenge_ids: list[str]
    favorite: bool
    title: str
    author: str
    translator: str

@dataclass
class Answer:
    kind: str
    id: str
    question_id: str
    answered: bool
    answer: str
    item_id: str
    user_id: str
    challenge_id: str


def fetch_container_items(db_client: DatabaseProxy, container_name: str) -> List[Dict[str, Any]]:
    logging.info("Fetching items from container '%s'", container_name)
    try:
        container = db_client.get_container_client(container_name)
        # get all items
        items = list(container.query_items(query="SELECT * FROM c", enable_cross_partition_query=True))
        logging.info("Fetched %d items from %s", len(items), container_name)
        return items
    except Exception as e:
        logging.warning("Could not read container %s: %s", container_name, e)
        return []

def parse_cosmos_question(input: Dict[str, Any]) -> Question:
    kind = input.get("kind")
    question = input.get("question")
    id = input.get("id")
    number = input.get("number")
    question_cluster_size = input.get("questionClusterSize")

    if kind is None or question is None \
        or id is None or number is None \
        or question_cluster_size is None:
        raise KeyError("One of challenge keys not found")
    return Question(kind, question, id, int(number), int(question_cluster_size))

def parse_cosmos_challenge(input: Dict[str, Any]) -> Challenge:
    id = input.get("id")
    name = input.get("name")
    status = input.get("status")
    target_media = input.get("targetMedia")
    questions = input.get("questions")
    if(id is None or name is None or status is None\
        or target_media is None or questions is None):
        raise KeyError("One of challenge keys not found")
    
    questions = [parse_cosmos_question(x) for x in questions]
    return Challenge(id, name, status, target_media, questions)

def get_challenges(database_client: DatabaseProxy) -> list[Challenge]:
    challenges = fetch_container_items(database_client, "challenges")
    return [parse_cosmos_challenge(x) for x in challenges]

def parse_library_item(input: Dict[str, Any]) -> LibraryItem:
    kind = input.get("kind")
    id = input.get("id")
    user_id = input.get("userId")
    added_at = input.get("addDate")
    activated_challenge_ids = input.get("activatedChallengeIds")
    favorite = input.get("favorite")

    if kind is None or id is None or user_id is None \
        or added_at is None or activated_challenge_ids is None \
        or favorite is None:
        raise KeyError("Missing library item info")
    book = input.get("book")
    game = input.get("game")
    title = ""
    author = ""
    translator = ""

    if(book is not None):
        book_title = book.get("title")
        if book_title is None :
            raise KeyError("Book should have a title")
        title = book_title
        book_author = book.get("author")
        if book_author is None:
            raise KeyError("Book should have a translator")
        author = book_author
        book_translator = book.get("translator")
        if book_translator is not None:
            translator = book_translator
    elif game is not None:
        game_title = game.get("title")
        if game_title is None:
            raise KeyError("Game should have a title")
        title = game_title
        game_author = game.get("creator")
        if game_author is None:
            raise KeyError(f"Game should have a author {id}")
        author = game_author
    else:
        raise KeyError("Unknown type")
    
    return LibraryItem(
        kind, id, user_id, added_at, 
        activated_challenge_ids, favorite, title, author, translator)



def get_library_items(db) -> list[LibraryItem]:
    items = fetch_container_items(db, "library")
    return [parse_library_item(x) for x in items]


def parse_answers(input: Dict[str, Any]) -> list[Answer]:
    user_id = input.get("userId")
    challenge_id = input.get("challengeId")
    answers = input.get("answers")

    if user_id is None or challenge_id is None or answers is None:
        raise KeyError("Could not convert answer")
    
    result = []
    for entry in answers:
        kind = entry.get("kind")
        id = entry.get("id")
        question_id = entry.get("questionId")
        answered = entry.get("answered")
        answer = entry.get("answer")
        item_id = entry.get("itemId")

        if kind is None or id is None or question_id is None \
            or answered is None or answer is None or item_id is None:
            raise KeyError(f"Not found all fields for answer {kind} {id}")
        
        answer = Answer(kind, id, question_id, answered, answer, item_id, user_id, challenge_id)
        result.append(answer)

    return result

def fetch_answers(db) -> list[Answer]:
    answer_sets = fetch_container_items(db, "answers")
    result = []
    for answer_set in answer_sets:
        for answer in parse_answers(answer_set):
            result.append(answer)
    return result


def write_challenge(cur:sqlite3.Cursor, challenge: Challenge):
    sql = "INSERT OR REPLACE INTO challenge(id, name, status, target_media, kind) VALUES(?,?,?,?,?)"
    cur.execute(sql, (challenge.id, challenge.name, challenge.status, challenge.target_media, 'shared'))

    for question in challenge.questions:
        question_sql = (
        "INSERT OR REPLACE INTO question(id, challenge_id, kind, question, question_cluster_size, number) "
        "VALUES(?,?,?,?,?,?)"
        )
        cur.execute(question_sql, 
            (question.id, challenge.id, question.kind, question.question, 
            question.question_cluster_size, question.number))
        
def write_item(cur: sqlite3.Cursor, item: LibraryItem):
    sql = (
        "INSERT OR REPLACE INTO library(id, user_id, kind, title, author, added_at, completed_at, favorite, translator)"
        " VALUES(?,?,?,?,?,?,?,?,?)"
    )
    cur.execute(sql, 
        (item.id, item.user_id, item.kind, item.title, 
        item.author, item.added_at, item.added_at, item.favorite, item.translator))
    
    for challenge_id in item.activated_challenge_ids:
        cur.execute(
            "INSERT OR REPLACE INTO activated_item_challenge(item_id, challenge_id) VALUES(?,?)",
            (item.id, challenge_id))

def filter_old_challenges(all_challenge_ids: list[str], answers: list[Answer]):
    print(all_challenge_ids)
    return [x for x in answers if x.challenge_id in all_challenge_ids]

def write_answer(cur: sqlite3.Cursor, answer: Answer):
    sql = (
        "INSERT OR REPLACE INTO answer(id, question_id, challenge_id, user_id, kind, answer, answered, item_id) "
        "VALUES(?,?,?,?,?,?,?,?)"
    )
    try:

        cur.execute(sql, 
            (answer.id, 
            answer.question_id, answer.challenge_id, 
            answer.user_id, answer.kind, 
            answer.answer, answer.answered, answer.item_id))
    except:
        print(answer)
        raise

def find_replacing_item_id(item_id: str, items: list[LibraryItem]) -> str:
    current_item = [x for x in items if x.id == item_id][0]
    target_user = "SETME"

    same_name_for_target = [x for x in items if x.user_id == target_user and x.title == current_item.title]
    if len(same_name_for_target) != 1:
        print(current_item)
        raise KeyError
    return same_name_for_target[0].id

def main():
    valid_user_ids = ['SETME']
    sqlite_path = "database.sqlite"
    conn_string = os.environ.get("AZURE_COSMOS_CONN")
    if conn_string is None :
        logging.error("Failed to read connection string from env")
        return 1
    db_name = "haasteikkoprod-db"
    cosmos_client = CosmosClient.from_connection_string(conn_string)
    database_client = cosmos_client.get_database_client(db_name)
    challenges = get_challenges(database_client)
    items = get_library_items(database_client)
    answers = fetch_answers(database_client)
    filtered_answers = filter_old_challenges([x.id for x in challenges], answers)
    valid_items = [x for x in items if x.user_id in valid_user_ids]
    print(f"Trimmed {len(answers) - len(filtered_answers)} old answers ({len(filtered_answers)}) remaining")

    sql_con = sqlite3.connect(sqlite_path,)
    sql_con.execute("PRAGMA foreign_keys = 1")
    cur = sql_con.cursor()

    for challenge in challenges:
        write_challenge(cur, challenge)
    
    for item in valid_items:
        write_item(cur, item)

    invalid_item_ids = []
    for answer in filtered_answers:
        if answer.user_id not in valid_user_ids:
            continue
        if answer.item_id not in [x.id for x in valid_items]:
            answer_real_item_id = find_replacing_item_id(answer.item_id, items)
            answer.item_id = answer_real_item_id
        
        write_answer(cur, answer)

    for item_id in invalid_item_ids:
        item = [x for x in items if x.id == item_id][0]
        same_name_found = [x for x in items if x.title == item.title and x.id != item.id and x.user_id in valid_user_ids]
        print(f"Invalid item: {item.id}, {item.title}, same found: {len(same_name_found)}")


    sql_con.commit()

main()