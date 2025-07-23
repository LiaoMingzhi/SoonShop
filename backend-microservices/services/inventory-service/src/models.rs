use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub sku: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub sku: String,
    pub quantity: i32,
    pub reserved_quantity: i32,
    pub available_quantity: i32,
    pub location: Option<String>,
    pub reorder_level: Option<i32>,
    pub max_level: Option<i32>,
    pub cost_per_unit: Option<f64>,
    pub last_count_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReservation {
    pub id: Uuid,
    pub inventory_item_id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: i32,
    pub status: ReservationStatus,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReservationStatus {
    Active,
    Fulfilled,
    Released,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryMovement {
    pub id: Uuid,
    pub inventory_item_id: Uuid,
    pub movement_type: MovementType,
    pub quantity: i32,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub reason: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MovementType {
    In,
    Out,
    Adjustment,
    Transfer,
}

// Request models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInventoryItemRequest {
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub sku: String,
    pub quantity: i32,
    pub location: Option<String>,
    pub reorder_level: Option<i32>,
    pub max_level: Option<i32>,
    pub cost_per_unit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInventoryItemRequest {
    pub location: Option<String>,
    pub reorder_level: Option<i32>,
    pub max_level: Option<i32>,
    pub cost_per_unit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustInventoryRequest {
    pub quantity_change: i32,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReserveInventoryRequest {
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub order_id: Uuid,
    pub quantity: i32,
    pub expires_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseReservationRequest {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub sku: String,
    pub price: f64,
    pub initial_stock: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStockRequest {
    pub quantity: i32,
    pub reason: String,
} 