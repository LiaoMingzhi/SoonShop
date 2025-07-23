// 库存相关模式定义 - 完整实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

// 仓库实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub warehouse_type: WarehouseType,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub capacity: Option<i64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 库存实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub id: Uuid,
    pub warehouse_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub sku: String,
    pub quantity_available: i64,
    pub quantity_reserved: i64,
    pub quantity_incoming: i64,
    pub quantity_damage: i64,
    pub reorder_point: i64,
    pub reorder_quantity: i64,
    pub cost_per_unit: Decimal,
    pub last_stock_check: Option<DateTime<Utc>>,
    pub location: Option<String>, // 仓库内位置
    pub batch_number: Option<String>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 库存预留
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReservation {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub order_id: Option<Uuid>,
    pub reserved_quantity: i64,
    pub reservation_type: ReservationType,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: ReservationStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 库存移动记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryMovement {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub movement_type: MovementType,
    pub quantity: i64,
    pub quantity_before: i64,
    pub quantity_after: i64,
    pub cost_per_unit: Option<Decimal>,
    pub total_cost: Option<Decimal>,
    pub reference_id: Option<Uuid>, // 关联的订单、采购单等ID
    pub reference_type: Option<String>,
    pub reason: Option<String>,
    pub batch_number: Option<String>,
    pub user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// 供应商实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
    pub tax_id: Option<String>,
    pub payment_terms: Option<String>,
    pub lead_time_days: Option<i32>,
    pub rating: Option<Decimal>, // 1-5 评分
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 采购订单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrder {
    pub id: Uuid,
    pub po_number: String,
    pub supplier_id: Uuid,
    pub warehouse_id: Uuid,
    pub status: PurchaseOrderStatus,
    pub order_date: DateTime<Utc>,
    pub expected_delivery_date: Option<DateTime<Utc>>,
    pub actual_delivery_date: Option<DateTime<Utc>>,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub shipping_cost: Decimal,
    pub total_amount: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 采购订单项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderItem {
    pub id: Uuid,
    pub purchase_order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub sku: String,
    pub quantity_ordered: i64,
    pub quantity_received: i64,
    pub unit_cost: Decimal,
    pub total_cost: Decimal,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 枚举定义
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WarehouseType {
    #[serde(rename = "main")]
    Main,
    #[serde(rename = "distribution")]
    Distribution,
    #[serde(rename = "retail")]
    Retail,
    #[serde(rename = "returns")]
    Returns,
    #[serde(rename = "quarantine")]
    Quarantine,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReservationType {
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "allocation")]
    Allocation,
    #[serde(rename = "transfer")]
    Transfer,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReservationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "fulfilled")]
    Fulfilled,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MovementType {
    #[serde(rename = "inbound")]
    Inbound,     // 入库
    #[serde(rename = "outbound")]
    Outbound,    // 出库
    #[serde(rename = "adjustment")]
    Adjustment,  // 库存调整
    #[serde(rename = "transfer")]
    Transfer,    // 仓库间转移
    #[serde(rename = "damage")]
    Damage,      // 损坏
    #[serde(rename = "return")]
    Return,      // 退货
    #[serde(rename = "cycle_count")]
    CycleCount,  // 周期盘点
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PurchaseOrderStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "pending_approval")]
    PendingApproval,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "partially_received")]
    PartiallyReceived,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

// 请求结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWarehouseRequest {
    pub name: String,
    pub code: String,
    pub warehouse_type: WarehouseType,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub capacity: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWarehouseRequest {
    pub name: Option<String>,
    pub warehouse_type: Option<WarehouseType>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub capacity: Option<i64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInventoryRequest {
    pub warehouse_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub sku: String,
    pub quantity_available: i64,
    pub reorder_point: i64,
    pub reorder_quantity: i64,
    pub cost_per_unit: Decimal,
    pub location: Option<String>,
    pub batch_number: Option<String>,
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInventoryRequest {
    pub quantity_available: Option<i64>,
    pub reorder_point: Option<i64>,
    pub reorder_quantity: Option<i64>,
    pub cost_per_unit: Option<Decimal>,
    pub location: Option<String>,
    pub batch_number: Option<String>,
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryAdjustmentRequest {
    pub inventory_id: Uuid,
    pub adjustment_quantity: i64, // 正数为增加，负数为减少
    pub reason: String,
    pub cost_per_unit: Option<Decimal>,
    pub batch_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReservationRequest {
    pub inventory_id: Uuid,
    pub order_id: Option<Uuid>,
    pub reserved_quantity: i64,
    pub reservation_type: ReservationType,
    pub expires_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReservationRequest {
    pub reserved_quantity: Option<i64>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: Option<ReservationStatus>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSupplierRequest {
    pub name: String,
    pub code: String,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
    pub tax_id: Option<String>,
    pub payment_terms: Option<String>,
    pub lead_time_days: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePurchaseOrderRequest {
    pub supplier_id: Uuid,
    pub warehouse_id: Uuid,
    pub expected_delivery_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub items: Vec<PurchaseOrderItemRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderItemRequest {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub sku: String,
    pub quantity_ordered: i64,
    pub unit_cost: Decimal,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiveInventoryRequest {
    pub purchase_order_id: Uuid,
    pub items: Vec<ReceiveInventoryItemRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiveInventoryItemRequest {
    pub purchase_order_item_id: Uuid,
    pub quantity_received: i64,
    pub batch_number: Option<String>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

// 响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub warehouse_type: WarehouseType,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub capacity: Option<i64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryResponse {
    pub id: Uuid,
    pub warehouse: WarehouseResponse,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub sku: String,
    pub quantity_available: i64,
    pub quantity_reserved: i64,
    pub quantity_incoming: i64,
    pub quantity_damage: i64,
    pub total_quantity: i64,
    pub reorder_point: i64,
    pub reorder_quantity: i64,
    pub cost_per_unit: Decimal,
    pub total_value: Decimal,
    pub last_stock_check: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub batch_number: Option<String>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub needs_reorder: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryListResponse {
    pub inventories: Vec<InventoryResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub low_stock_count: i64,
    pub out_of_stock_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationResponse {
    pub id: Uuid,
    pub inventory: InventoryResponse,
    pub order_id: Option<Uuid>,
    pub reserved_quantity: i64,
    pub reservation_type: ReservationType,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: ReservationStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovementResponse {
    pub id: Uuid,
    pub inventory: InventoryResponse,
    pub movement_type: MovementType,
    pub quantity: i64,
    pub quantity_before: i64,
    pub quantity_after: i64,
    pub cost_per_unit: Option<Decimal>,
    pub total_cost: Option<Decimal>,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub reason: Option<String>,
    pub batch_number: Option<String>,
    pub user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplierResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
    pub tax_id: Option<String>,
    pub payment_terms: Option<String>,
    pub lead_time_days: Option<i32>,
    pub rating: Option<Decimal>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderResponse {
    pub id: Uuid,
    pub po_number: String,
    pub supplier: SupplierResponse,
    pub warehouse: WarehouseResponse,
    pub status: PurchaseOrderStatus,
    pub order_date: DateTime<Utc>,
    pub expected_delivery_date: Option<DateTime<Utc>>,
    pub actual_delivery_date: Option<DateTime<Utc>>,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub shipping_cost: Decimal,
    pub total_amount: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub items: Vec<PurchaseOrderItemResponse>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub sku: String,
    pub quantity_ordered: i64,
    pub quantity_received: i64,
    pub unit_cost: Decimal,
    pub total_cost: Decimal,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 统计和报告
#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryStatistics {
    pub total_products: i64,
    pub total_value: Decimal,
    pub low_stock_items: i64,
    pub out_of_stock_items: i64,
    pub total_reservations: i64,
    pub warehouse_stats: HashMap<Uuid, WarehouseStats>,
    pub movement_stats: HashMap<MovementType, MovementStats>,
    pub top_moving_products: Vec<ProductMovementStat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseStats {
    pub warehouse_id: Uuid,
    pub warehouse_name: String,
    pub total_products: i64,
    pub total_value: Decimal,
    pub utilization_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovementStats {
    pub movement_type: MovementType,
    pub total_movements: i64,
    pub total_quantity: i64,
    pub total_value: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductMovementStat {
    pub product_id: Uuid,
    pub sku: String,
    pub total_movements: i64,
    pub total_quantity: i64,
    pub movement_velocity: f64, // 移动速度
}

// 查询参数
#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub warehouse_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub sku: Option<String>,
    pub low_stock: Option<bool>,
    pub out_of_stock: Option<bool>,
    pub needs_reorder: Option<bool>,
    pub location: Option<String>,
    pub batch_number: Option<String>,
    pub expiry_date_from: Option<DateTime<Utc>>,
    pub expiry_date_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovementQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub inventory_id: Option<Uuid>,
    pub movement_type: Option<MovementType>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub reference_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

// 批量操作
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkInventoryAdjustmentRequest {
    pub adjustments: Vec<InventoryAdjustmentRequest>,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkTransferRequest {
    pub from_warehouse_id: Uuid,
    pub to_warehouse_id: Uuid,
    pub items: Vec<TransferItem>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferItem {
    pub inventory_id: Uuid,
    pub quantity: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkOperationResponse {
    pub success_count: i32,
    pub failed_count: i32,
    pub errors: Vec<String>,
} 