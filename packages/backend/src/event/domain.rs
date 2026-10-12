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

fn active_challenge_ids_for_kind(
    state: &AppState,
    kind: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut challenge_repo = ChallengeRepository::new(Database::new(&state.database_path)?);
    let challenges = challenge_repo.search(ChallengeFilter {
        status: Some("active".to_string()),
        media_type: Some(kind.to_string()),
    })?;
    Ok(challenges.iter().map(|c| c.id.clone()).collect())
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

    let activated_challenge_ids = match &new_event.activated_challenge_ids {
        Some(ids) => ids.clone(),
        None => active_challenge_ids_for_kind(state, &item.kind)?,
    };

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

    let existing_event = match repo.read_by_id(id)? {
        Some(e) if e.user_id == user.id => e,
        _ => return Ok(false),
    };

    let activated_challenge_ids = match &new_event.activated_challenge_ids {
        Some(ids) => ids.clone(),
        None => {
            let mut library_repo = LibraryRepository::new(Database::new(&state.database_path)?);
            let item = library_repo
                .read_by_id(&existing_event.item_id)?
                .ok_or("Item not found")?;
            active_challenge_ids_for_kind(state, &item.kind)?
        }
    };

    // An event can't be moved to another item: `new_event.item_id` is ignored, so an update can't
    // attach the event to an item the user doesn't own.
    let event = Event {
        id: id.to_string(),
        user_id: user.id.clone(),
        item_id: existing_event.item_id.clone(),
        completed_at: new_event.completed_at.clone(),
        activated_challenge_ids,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrations::Migrator;
    use std::path::PathBuf;
    use tempfile::{TempDir, tempdir};

    const OWN_ITEM: &str = "00000000-0000-4000-8000-000000000001";
    const FOREIGN_ITEM: &str = "00000000-0000-4000-8000-000000000002";

    fn setup() -> (TempDir, AppState) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let migrations_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");
        Migrator::new(&db_path, &migrations_path)
            .unwrap()
            .run_migrations()
            .unwrap();

        let db = Database::new(db_path.to_str().unwrap()).unwrap();
        db.conn
            .execute_batch(&format!(
                "INSERT INTO library (id, user_id, kind, title, author, added_at)
                    VALUES ('{OWN_ITEM}', 'user-1', 'Book', 'Oma', 'Kirjailija', '2026-01-01');
                 INSERT INTO library (id, user_id, kind, title, author, added_at)
                    VALUES ('{FOREIGN_ITEM}', 'user-2', 'Book', 'Vieras', 'Kirjailija', '2026-01-01');"
            ))
            .unwrap();

        let state = AppState {
            jwks: jsonwebtoken::jwk::JwkSet { keys: vec![] },
            required_audience: String::new(),
            database_path: db_path.to_str().unwrap().to_string(),
        };
        (dir, state)
    }

    fn new_event(item_id: &str, completed_at: &str) -> NewEvent {
        NewEvent {
            item_id: item_id.to_string(),
            completed_at: completed_at.to_string(),
            activated_challenge_ids: Some(vec![]),
        }
    }

    #[test]
    fn updating_an_event_cannot_move_it_to_another_item() {
        let (_dir, state) = setup();
        let user = User::new("user-1".to_string());
        let id = create_event(&user, &state, &new_event(OWN_ITEM, "2026-01-01")).unwrap();

        let updated =
            update_event(&user, &state, &id, &new_event(FOREIGN_ITEM, "2026-02-02")).unwrap();
        assert!(updated);

        let event = get_event_by_id(&user, &state, &id).unwrap().unwrap();
        assert_eq!(event.item_id, OWN_ITEM);
        assert_eq!(event.completed_at, "2026-02-02");
    }
}
