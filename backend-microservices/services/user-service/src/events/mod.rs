use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdatedEvent {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeletedEvent {
    pub user_id: Uuid,
    pub deleted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedInEvent {
    pub user_id: Uuid,
    pub username: String,
    pub login_at: DateTime<Utc>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum UserEvent {
    UserCreated(UserCreatedEvent),
    UserUpdated(UserUpdatedEvent),
    UserDeleted(UserDeletedEvent),
    UserLoggedIn(UserLoggedInEvent),
}

impl UserEvent {
    pub fn user_id(&self) -> Uuid {
        match self {
            UserEvent::UserCreated(event) => event.user_id,
            UserEvent::UserUpdated(event) => event.user_id,
            UserEvent::UserDeleted(event) => event.user_id,
            UserEvent::UserLoggedIn(event) => event.user_id,
        }
    }

    pub fn event_type(&self) -> &'static str {
        match self {
            UserEvent::UserCreated(_) => "user.created",
            UserEvent::UserUpdated(_) => "user.updated",
            UserEvent::UserDeleted(_) => "user.deleted",
            UserEvent::UserLoggedIn(_) => "user.logged_in",
        }
    }
} 