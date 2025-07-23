use sea_orm::{entity::prelude::*, Set, ActiveValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub wallet_address: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub wallet_address: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(phone)]
    pub phone: Option<String>,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 1))]
    pub first_name: Option<String>,
    #[validate(length(min = 1))]
    pub last_name: Option<String>,
    #[validate(phone)]
    pub phone: Option<String>,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

impl From<Model> for UserResponse {
    fn from(user: Model) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            wallet_address: user.wallet_address,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<CreateUserRequest> for ActiveModel {
    fn from(req: CreateUserRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Set(Uuid::new_v4()),
            email: Set(req.email),
            password_hash: Set(String::new()), // 将在服务层设置
            first_name: Set(req.first_name),
            last_name: Set(req.last_name),
            phone: Set(req.phone),
            wallet_address: Set(req.wallet_address),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
        }
    }
}

impl Entity {
    pub fn find_by_email(email: &str) -> Select<Self> {
        Self::find().filter(Column::Email.eq(email))
    }
    
    pub fn find_by_wallet_address(wallet_address: &str) -> Select<Self> {
        Self::find().filter(Column::WalletAddress.eq(wallet_address))
    }
    
    pub fn find_active_users() -> Select<Self> {
        Self::find().filter(Column::IsActive.eq(true))
    }
} 