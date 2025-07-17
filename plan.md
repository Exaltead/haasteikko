# Plan: User Content Migration Tool for User ID Change

## Description

Create a tool to migrate all content for a single user when their user ID changes. The tool will:

- Recreate all library items for the user with the new user ID.
- Recreate all related answers, updating references to the new user ID and the new library item IDs.
- Solutions will not be migrated.
- Connection string to Cosmos DB will be read from command line.
- Old and new UserId will be read from command line.

## Steps to Implement

1. Identify all library items for the old user ID.
2. For each library item:
   - Create a new item with the new user ID and a new item ID.
   - Map old item IDs to new item IDs.
3. Identify all answers related to the old user ID and their library items.
4. For each answer:
   - Recreate the answer with the new user ID and update the reference to the new library item ID.
5. Leave solutions unchanged.
6. Provide a script or function to execute the migration, accepting old and new user IDs as input.
7. Add logging and error handling for traceability.

## Dependencies & Considerations

- Access to Cosmos DB collections: `library`, `answers`.

## Files to Create/Modify

- **scripts/user_content_migration.py** (new): Migration script/tool implementation.
