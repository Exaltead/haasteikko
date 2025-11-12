use crate::{
    AppState,
    auth::User,
    challenge::domain::{
        create_challenge, delete_challenge, get_challenge_by_id, get_challenges, update_challenge,
    },
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};

mod api;
mod domain;
mod repository;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub kind: String,
    pub question: String,
    pub id: String,
    pub number: i32,
    pub question_cluster_size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedChallenge {
    pub id: String,
    pub name: String,
    pub status: String,
    pub target_media: String,
    pub questions: Vec<Question>,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSharedChallenge {
    pub name: String,
    pub status: String,
    pub target_media: String,
    pub questions: Vec<Question>,
}

pub use api::routes;
