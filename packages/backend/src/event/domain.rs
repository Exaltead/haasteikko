use crate::{
    AppState,
    auth::User,
    challenge::{ChallengeFilter, ChallengeRepository},
    database::{Database, Repository},
    event::{Event, EventFilter, EventRepository, NewEvent},
    library::LibraryRepository,
};

pub fn get_events(
    user: &User,
    state: &AppState,
    item_id: Option<&str>,
) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = EventRepository::new(db);
    let filter = EventFilter {
        user_id: user.id.clone(),
        item_id: item_id.map(|s| s.to_string()),
    };

    let events = repo.search(filter)?;
    Ok(events)
}

pub fn get_event_by_id(
    user: &User,
    state: &AppState,
    id: &str,
) -> Result<Option<Event>, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = EventRepository::new(db);
    if let Some(event) = repo.read_by_id(id)? {
        if event.user_id == user.id {
            Ok(Some(event))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub fn create_event(
    user: &User,
    state: &AppState,
    new_event: &NewEvent,
) -> Result<String, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;

    let mut library_repo = LibraryRepository::new(Database::new(&state.database_path)?);
    let item = library_repo
        .read_by_id(&new_event.item_id)?
        .ok_or("Item not found")?;

    if item.user_id != user.id {
        return Err("Not authorized".into());
    }

    // Should maybe reuse the same connection
    let mut challenge_repo = ChallengeRepository::new(Database::new(&state.database_path)?);
    let challenges = challenge_repo.search(ChallengeFilter {
        status: Some("active".to_string()),
        media_type: Some(item.kind.clone()),
    })?;

    let activated_challenge_ids = challenges.iter().map(|c| c.id.clone()).collect();

    let mut repo = EventRepository::new(db);

    let event = Event {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: user.id.clone(),
        item_id: new_event.item_id.clone(),
        completed_at: new_event.completed_at.clone(),
        activated_challenge_ids,
    };

    let id = repo.create(&event)?;
    Ok(id)
}

pub fn update_event(
    user: &User,
    state: &AppState,
    id: &str,
    new_event: &NewEvent,
) -> Result<bool, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = EventRepository::new(db);

    if let Some(existing_event) = repo.read_by_id(id)? {
        if existing_event.user_id != user.id {
            return Ok(false);
        }
    } else {
        return Ok(false);
    }

    let event = Event {
        id: id.to_string(),
        user_id: user.id.clone(),
        item_id: new_event.item_id.clone(),
        completed_at: new_event.completed_at.clone(),
        activated_challenge_ids: new_event.activated_challenge_ids.clone(),
    };

    let updated = repo.update(id, &event)?;
    Ok(updated)
}

pub fn delete_event(
    user: &User,
    state: &AppState,
    id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = EventRepository::new(db);

    if let Some(existing_event) = repo.read_by_id(id)? {
        if existing_event.user_id != user.id {
            return Ok(false);
        }
    } else {
        return Ok(false);
    }

    let deleted = repo.delete(id)?;
    Ok(deleted)
}
