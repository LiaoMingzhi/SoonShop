use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: String,
    pub warehouse_id: Uuid,
    pub location: String,
    pub quantity: i32,
    pub reserved_quantity: i32,
    pub available_quantity: i32,
    pub minimum_quantity: i32,
    pub maximum_quantity: i32,
    pub reorder_point: i32,
    pub reorder_quantity: i32,
    pub cost_price: rust_decimal::Decimal,
    pub weight: Option<rust_decimal::Decimal>,
    pub dimensions: Option<Dimensions>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_checked_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub length: rust_decimal::Decimal,
    pub width: rust_decimal::Decimal,
    pub height: rust_decimal::Decimal,
    pub unit: String, // cm, inch, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReservation {
    pub id: Uuid,
    pub inventory_item_id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub quantity: i32,
    pub status: ReservationStatus,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReservationStatus {
    Active,
    Expired,
    Released,
    Fulfilled,
}

impl std::fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReservationStatus::Active => write!(f, "active"),
            ReservationStatus::Expired => write!(f, "expired"),
            ReservationStatus::Released => write!(f, "released"),
            ReservationStatus::Fulfilled => write!(f, "fulfilled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryMovement {
    pub id: Uuid,
    pub inventory_item_id: Uuid,
    pub movement_type: MovementType,
    pub quantity: i32,
    pub previous_quantity: i32,
    pub new_quantity: i32,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>, // order, adjustment, transfer, etc.
    pub reason: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MovementType {
    StockIn,
    StockOut,
    Transfer,
    Adjustment,
    Damaged,
    Lost,
    Found,
    Return,
    Sale,
    Purchase,
    Reserved,
    Released,
}

impl std::fmt::Display for MovementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MovementType::StockIn => write!(f, "stock_in"),
            MovementType::StockOut => write!(f, "stock_out"),
            MovementType::Transfer => write!(f, "transfer"),
            MovementType::Adjustment => write!(f, "adjustment"),
            MovementType::Damaged => write!(f, "damaged"),
            MovementType::Lost => write!(f, "lost"),
            MovementType::Found => write!(f, "found"),
            MovementType::Return => write!(f, "return"),
            MovementType::Sale => write!(f, "sale"),
            MovementType::Purchase => write!(f, "purchase"),
            MovementType::Reserved => write!(f, "reserved"),
            MovementType::Released => write!(f, "released"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub address: Address,
    pub manager_id: Option<Uuid>,
    pub contact_info: ContactInfo,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
    pub coordinates: Option<Coordinates>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub manager_name: Option<String>,
}

// 请求和响应模型
#[derive(Debug, Deserialize, Validate)]
pub struct CreateInventoryItemRequest {
    pub product_id: Uuid,
    pub sku: String,
    pub warehouse_id: Uuid,
    pub location: String,
    #[validate(range(min = 0))]
    pub quantity: i32,
    #[validate(range(min = 0))]
    pub minimum_quantity: i32,
    #[validate(range(min = 0))]
    pub maximum_quantity: i32,
    #[validate(range(min = 0))]
    pub reorder_point: i32,
    #[validate(range(min = 0))]
    pub reorder_quantity: i32,
    pub cost_price: rust_decimal::Decimal,
    pub weight: Option<rust_decimal::Decimal>,
    pub dimensions: Option<Dimensions>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateInventoryItemRequest {
    pub location: Option<String>,
    #[validate(range(min = 0))]
    pub minimum_quantity: Option<i32>,
    #[validate(range(min = 0))]
    pub maximum_quantity: Option<i32>,
    #[validate(range(min = 0))]
    pub reorder_point: Option<i32>,
    #[validate(range(min = 0))]
    pub reorder_quantity: Option<i32>,
    pub cost_price: Option<rust_decimal::Decimal>,
    pub weight: Option<rust_decimal::Decimal>,
    pub dimensions: Option<Dimensions>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AdjustInventoryRequest {
    pub adjustment_type: AdjustmentType,
    pub quantity: i32,
    pub reason: String,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReserveInventoryRequest {
    pub product_id: Uuid,
    pub order_id: Uuid,
    #[validate(range(min = 1))]
    pub quantity: i32,
    pub warehouse_id: Option<Uuid>,
    pub reservation_duration_minutes: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReleaseReservationRequest {
    pub reservation_id: Uuid,
    pub quantity: Option<i32>, // 部分释放
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdjustmentType {
    Increase,
    Decrease,
    SetTo,
}

#[derive(Debug, Serialize)]
pub struct InventoryItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: String,
    pub warehouse_id: Uuid,
    pub location: String,
    pub quantity: i32,
    pub reserved_quantity: i32,
    pub available_quantity: i32,
    pub minimum_quantity: i32,
    pub maximum_quantity: i32,
    pub reorder_point: i32,
    pub reorder_quantity: i32,
    pub cost_price: rust_decimal::Decimal,
    pub weight: Option<rust_decimal::Decimal>,
    pub dimensions: Option<Dimensions>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_checked_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub is_low_stock: bool,
    pub needs_reorder: bool,
}

#[derive(Debug, Serialize)]
pub struct ReservationResponse {
    pub id: Uuid,
    pub inventory_item_id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub quantity: i32,
    pub status: ReservationStatus,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub time_remaining_minutes: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct InventoryMovementResponse {
    pub id: Uuid,
    pub inventory_item_id: Uuid,
    pub movement_type: MovementType,
    pub quantity: i32,
    pub previous_quantity: i32,
    pub new_quantity: i32,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub reason: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub notes: Option<String>,
}

impl From<InventoryItem> for InventoryItemResponse {
    fn from(item: InventoryItem) -> Self {
        let is_low_stock = item.available_quantity <= item.minimum_quantity;
        let needs_reorder = item.available_quantity <= item.reorder_point;
        
        InventoryItemResponse {
            id: item.id,
            product_id: item.product_id,
            sku: item.sku,
            warehouse_id: item.warehouse_id,
            location: item.location,
            quantity: item.quantity,
            reserved_quantity: item.reserved_quantity,
            available_quantity: item.available_quantity,
            minimum_quantity: item.minimum_quantity,
            maximum_quantity: item.maximum_quantity,
            reorder_point: item.reorder_point,
            reorder_quantity: item.reorder_quantity,
            cost_price: item.cost_price,
            weight: item.weight,
            dimensions: item.dimensions,
            created_at: item.created_at,
            updated_at: item.updated_at,
            last_checked_at: item.last_checked_at,
            is_active: item.is_active,
            is_low_stock,
            needs_reorder,
        }
    }
}

impl From<InventoryReservation> for ReservationResponse {
    fn from(reservation: InventoryReservation) -> Self {
        let time_remaining = if reservation.status == ReservationStatus::Active {
            let now = Utc::now();
            if reservation.expires_at > now {
                Some((reservation.expires_at - now).num_minutes())
            } else {
                Some(0)
            }
        } else {
            None
        };
        
        ReservationResponse {
            id: reservation.id,
            inventory_item_id: reservation.inventory_item_id,
            order_id: reservation.order_id,
            user_id: reservation.user_id,
            quantity: reservation.quantity,
            status: reservation.status,
            expires_at: reservation.expires_at,
            created_at: reservation.created_at,
            updated_at: reservation.updated_at,
            time_remaining_minutes: time_remaining,
        }
    }
}

impl From<InventoryMovement> for InventoryMovementResponse {
    fn from(movement: InventoryMovement) -> Self {
        InventoryMovementResponse {
            id: movement.id,
            inventory_item_id: movement.inventory_item_id,
            movement_type: movement.movement_type,
            quantity: movement.quantity,
            previous_quantity: movement.previous_quantity,
            new_quantity: movement.new_quantity,
            reference_id: movement.reference_id,
            reference_type: movement.reference_type,
            reason: movement.reason,
            created_at: movement.created_at,
            created_by: movement.created_by,
            notes: movement.notes,
        }
    }
} 