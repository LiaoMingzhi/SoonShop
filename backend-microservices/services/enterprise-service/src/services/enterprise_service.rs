use crate::models::{Enterprise, EnterpriseCreateRequest, EnterpriseUpdateRequest, EnterpriseStats};
use crate::repositories::enterprise_repository::EnterpriseRepository;
use anyhow::Result;
use chrono::Utc;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

#[derive(Clone)]
pub struct EnterpriseService {
    repository: EnterpriseRepository,
}

impl EnterpriseService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repository: EnterpriseRepository::new(db),
        }
    }

    pub async fn create_enterprise(&self, request: EnterpriseCreateRequest) -> Result<Enterprise> {
        let now = Utc::now();
        let enterprise = Enterprise {
            id: Uuid::new_v4(),
            owner_id: request.owner_id,
            name: request.name,
            description: request.description,
            current_multiplier: 2.0, // Default multiplier
            total_evaluations: 0,
            average_score: 0.0,
            last_evaluation_at: None,
            total_rewards_earned: 0,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.repository.create(enterprise).await
    }

    pub async fn get_enterprise(&self, id: Uuid) -> Result<Option<Enterprise>> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_enterprises_by_owner(&self, owner_id: Uuid) -> Result<Vec<Enterprise>> {
        self.repository.find_by_owner_id(owner_id).await
    }

    pub async fn update_enterprise(&self, id: Uuid, request: EnterpriseUpdateRequest) -> Result<Option<Enterprise>> {
        let mut enterprise = match self.repository.find_by_id(id).await? {
            Some(enterprise) => enterprise,
            None => return Ok(None),
        };

        if let Some(name) = request.name {
            enterprise.name = name;
        }

        if let Some(description) = request.description {
            enterprise.description = Some(description);
        }

        enterprise.updated_at = Utc::now();

        self.repository.update(enterprise).await.map(Some)
    }

    pub async fn update_multiplier(&self, id: Uuid, multiplier: f64) -> Result<Option<Enterprise>> {
        let mut enterprise = match self.repository.find_by_id(id).await? {
            Some(enterprise) => enterprise,
            None => return Ok(None),
        };

        enterprise.current_multiplier = multiplier.max(1.0).min(5.0); // Clamp between 1.0 and 5.0
        enterprise.updated_at = Utc::now();

        self.repository.update(enterprise).await.map(Some)
    }

    pub async fn calculate_rewards(&self, id: Uuid, base_amount: u64) -> Result<u64> {
        let enterprise = match self.repository.find_by_id(id).await? {
            Some(enterprise) => enterprise,
            None => return Ok(base_amount),
        };

        let rewards = (base_amount as f64 * enterprise.current_multiplier) as u64;
        Ok(rewards)
    }

    pub async fn add_rewards(&self, id: Uuid, amount: u64) -> Result<Option<Enterprise>> {
        let mut enterprise = match self.repository.find_by_id(id).await? {
            Some(enterprise) => enterprise,
            None => return Ok(None),
        };

        enterprise.total_rewards_earned += amount;
        enterprise.updated_at = Utc::now();

        self.repository.update(enterprise).await.map(Some)
    }

    pub async fn get_enterprise_stats(&self) -> Result<EnterpriseStats> {
        let stats = self.repository.get_stats().await?;
        Ok(stats)
    }

    pub async fn deactivate_enterprise(&self, id: Uuid) -> Result<Option<Enterprise>> {
        let mut enterprise = match self.repository.find_by_id(id).await? {
            Some(enterprise) => enterprise,
            None => return Ok(None),
        };

        enterprise.is_active = false;
        enterprise.updated_at = Utc::now();

        self.repository.update(enterprise).await.map(Some)
    }

    pub async fn activate_enterprise(&self, id: Uuid) -> Result<Option<Enterprise>> {
        let mut enterprise = match self.repository.find_by_id(id).await? {
            Some(enterprise) => enterprise,
            None => return Ok(None),
        };

        enterprise.is_active = true;
        enterprise.updated_at = Utc::now();

        self.repository.update(enterprise).await.map(Some)
    }
} 