use crate::{
    AppState,
    auth::User,
    challenge::{ChallengeFilter, ChallengeRepository},
    database::{Database, Repository},
    library::{LibraryFilter, LibraryItem, LibraryRepository, NewLibraryItem},
};

pub fn get_library_items(
    user: &User,
    state: &AppState,
) -> Result<Vec<LibraryItem>, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = LibraryRepository::new(db);
    let filter = LibraryFilter {
        user_id: user.id.clone(),
        item_id: None,
    };

    let items = repo.search(filter)?;
    Ok(items)
}

pub fn get_library_item_by_id(
    user: &User,
    state: &AppState,
    id: &str,
) -> Result<Option<LibraryItem>, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = LibraryRepository::new(db);
    if let Some(item) = repo.read_by_id(id)? {
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
    let db = Database::new(&state.database_path)?;
    // Should maybe reuse the same connection
    let mut challenge_repo = ChallengeRepository::new(Database::new(&state.database_path)?);
    let challenges = challenge_repo.search(ChallengeFilter {
        status: Some("active".to_string()),
        media_type: Some(item.kind.to_string()),
    })?;

    let activated_challenges = challenges.iter().map(|f| f.id.clone()).collect();

    let mut repo = LibraryRepository::new(db);

    let item = LibraryItem {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: user.id.clone(),
        kind: item.kind.clone(),
        title: item.title.clone(),
        author: item.author.clone(),
        added_at: chrono::Utc::now().to_rfc3339(),
        completed_at: item.completed_at.clone(),
        favorite: item.favorite,
        activated_challenge_ids: activated_challenges,
        translator: item.translator.clone(),
    };
    let id = repo.create(&item)?;
    Ok(id)
}

pub fn update_library_item(
    user: &User,
    state: &AppState,
    id: &str,
    item: &NewLibraryItem,
) -> Result<bool, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = LibraryRepository::new(db);
    if let Some(existing_item) = repo.read_by_id(id)? {
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

    let updated = repo.update(id, &item)?;
    Ok(updated)
}

pub fn delete_library_item(
    user: &User,
    state: &AppState,
    id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = LibraryRepository::new(db);

    if let Some(existing_item) = repo.read_by_id(id)? {
        if existing_item.user_id != user.id {
            return Ok(false);
        }
    } else {
        return Ok(false);
    }

    let deleted = repo.delete(id)?;
    Ok(deleted)
}
