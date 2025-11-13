use crate::{
    AppState,
    auth::User,
    challenge::{
        NewSharedChallenge, SharedChallenge,
        repository::{ChallengeFilter, ChallengeRepository},
    },
    database::{Database, Repository},
};

pub fn get_challenges(
    state: &AppState,
) -> Result<Vec<SharedChallenge>, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = ChallengeRepository::new(db);
    Ok(repo.search(ChallengeFilter::new())?)
}

pub fn get_challenge_by_id(
    state: &AppState,
    id: &str,
) -> Result<Option<SharedChallenge>, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = ChallengeRepository::new(db);
    Ok(repo.read_by_id(id)?)
}

pub fn create_challenge(
    _user: &User, // Not used as shared challenges don't track ownership
    state: &AppState,
    challenge: &NewSharedChallenge,
) -> Result<String, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = ChallengeRepository::new(db);

    let challenge = SharedChallenge {
        id: uuid::Uuid::new_v4().to_string(),
        name: challenge.name.clone(),
        status: challenge.status.clone(),
        target_media: challenge.target_media.clone(),
        questions: challenge.questions.clone(),
        kind: "shared".to_string(),
    };

    Ok(repo.create(&challenge)?)
}

pub fn update_challenge(
    _user: &User, // Not used as anyone can update shared challenges
    state: &AppState,
    id: &str,
    challenge: &NewSharedChallenge,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = ChallengeRepository::new(db);

    let challenge = SharedChallenge {
        id: id.to_string(),
        name: challenge.name.clone(),
        status: challenge.status.clone(),
        target_media: challenge.target_media.clone(),
        questions: challenge.questions.clone(),
        kind: "shared".to_string(),
    };

    if repo.update(id, &challenge)? {
        Ok(())
    } else {
        Err("Challenge not found".into())
    }
}

pub fn delete_challenge(
    _user: &User, // Not used as anyone can delete shared challenges
    state: &AppState,
    id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = ChallengeRepository::new(db);

    if repo.delete(id)? {
        Ok(())
    } else {
        Err("Challenge not found".into())
    }
}
