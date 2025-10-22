use crate::{
    AppState,
    auth::User,
    database::{Database, Repository},
    library::{LibraryFilter, LibraryItem, NewLibraryItem},
};

pub fn get_library_items(
    user: &User,
    state: &AppState,
) -> Result<Vec<LibraryItem>, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let filter = LibraryFilter {
        user_id: user.id.clone(),
    };

    let items = db.search(filter)?;
    Ok(items)
}

pub fn get_library_item_by_id(
    user: &User,
    state: &AppState,
    id: &str,
) -> Result<Option<LibraryItem>, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    if let Some(item) = db.read_by_id(id)? {
        if item.user_id == user.id {
            Ok(Some(item))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub fn create_library_item(
    user: &User,
    state: &AppState,
    item: &NewLibraryItem,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut db = Database::new(&state.database_path)?;
    let item = LibraryItem {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: user.id.clone(),
        kind: item.kind.clone(),
        title: item.title.clone(),
        author: item.author.clone(),
        added_at: chrono::Utc::now().to_rfc3339(),
        completed_at: item.completed_at.clone(),
        favorite: item.favorite,
        activated_challenge_ids: item.activated_challenge_ids.clone(),
        translator: item.translator.clone(),
    };
    let id = db.create(&item)?;
    Ok(id)
}

pub fn update_library_item(
    user: &User,
    state: &AppState,
    id: &str,
    item: &NewLibraryItem,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut db = Database::new(&state.database_path)?;
    println!("Updating item with id: {} for user {}", id, user.id);
    if let Some(existing_item) = db.read_by_id(id)? {
        if existing_item.user_id != user.id {
            println!("User ID mismatch: {} vs {}", existing_item.user_id, user.id);
            return Ok(false);
        }
    } else {
        return Ok(false);
    }

    let item = LibraryItem {
        id: id.to_string(),
        user_id: user.id.clone(),
        kind: item.kind.clone(),
        title: item.title.clone(),
        author: item.author.clone(),
        added_at: "".to_string(), // Not updated
        completed_at: item.completed_at.clone(),
        favorite: item.favorite,
        activated_challenge_ids: item.activated_challenge_ids.clone(),
        translator: item.translator.clone(),
    };

    let updated = db.update(id, &item)?;
    Ok(updated)
}

pub fn delete_library_item(
    user: &User,
    state: &AppState,
    id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;

    if let Some(existing_item) = db.read_by_id(id)? {
        if existing_item.user_id != user.id {
            return Ok(false);
        }
    } else {
        return Ok(false);
    }

    let deleted = db.delete(id)?;
    Ok(deleted)
}
