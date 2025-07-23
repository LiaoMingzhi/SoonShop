use crate::{db::Database, error::OrderError, models::*};
use anyhow::Result;
use uuid::Uuid;

#[derive(Clone)]
pub struct OrderService {
    db: Database,
}

impl OrderService {
    pub fn new(db: &Database) -> Self {
        Self { db: db.clone() }
    }
    
    pub async fn create_order(&self, _req: CreateOrderRequest) -> Result<Order, OrderError> {
        Err(OrderError::InternalError("Not implemented".to_string()))
    }
    
    pub async fn get_order(&self, _order_id: Uuid) -> Result<Order, OrderError> {
        Err(OrderError::NotFound("Order not found".to_string()))
    }
} 