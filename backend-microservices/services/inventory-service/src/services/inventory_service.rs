use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::inventory::{
    InventoryItem, InventoryReservation, InventoryMovement, 
    CreateInventoryItemRequest, UpdateInventoryItemRequest, AdjustInventoryRequest,
    ReserveInventoryRequest, ReleaseReservationRequest, ReservationStatus,
    MovementType, AdjustmentType
};
use crate::db::Database;

pub struct InventoryService<'a> {
    db: &'a Database,
}

impl<'a> InventoryService<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }
    
    pub async fn create_inventory_item(
        &self,
        req: &CreateInventoryItemRequest,
        created_by: Uuid,
    ) -> Result<InventoryItem> {
        let item = InventoryItem {
            id: Uuid::new_v4(),
            product_id: req.product_id,
            sku: req.sku.clone(),
            warehouse_id: req.warehouse_id,
            location: req.location.clone(),
            quantity: req.quantity,
            reserved_quantity: 0,
            available_quantity: req.quantity,
            minimum_quantity: req.minimum_quantity,
            maximum_quantity: req.maximum_quantity,
            reorder_point: req.reorder_point,
            reorder_quantity: req.reorder_quantity,
            cost_price: req.cost_price,
            weight: req.weight,
            dimensions: req.dimensions.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_checked_at: None,
            is_active: true,
        };
        
        self.db.create_inventory_item(&item).await?;
        
        // 记录库存变动
        self.record_movement(
            &item,
            MovementType::StockIn,
            req.quantity,
            0,
            req.quantity,
            None,
            None,
            "Initial stock".to_string(),
            created_by,
            None,
        ).await?;
        
        Ok(item)
    }
    
    pub async fn update_inventory_item(
        &self,
        item_id: Uuid,
        req: &UpdateInventoryItemRequest,
    ) -> Result<InventoryItem> {
        let mut item = self.db.get_inventory_item_by_id(item_id).await?
            .ok_or_else(|| anyhow::anyhow!("Inventory item not found"))?;
        
        if let Some(location) = &req.location {
            item.location = location.clone();
        }
        if let Some(min_qty) = req.minimum_quantity {
            item.minimum_quantity = min_qty;
        }
        if let Some(max_qty) = req.maximum_quantity {
            item.maximum_quantity = max_qty;
        }
        if let Some(reorder_point) = req.reorder_point {
            item.reorder_point = reorder_point;
        }
        if let Some(reorder_qty) = req.reorder_quantity {
            item.reorder_quantity = reorder_qty;
        }
        if let Some(cost_price) = req.cost_price {
            item.cost_price = cost_price;
        }
        if let Some(weight) = req.weight {
            item.weight = Some(weight);
        }
        if let Some(dimensions) = &req.dimensions {
            item.dimensions = Some(dimensions.clone());
        }
        if let Some(is_active) = req.is_active {
            item.is_active = is_active;
        }
        
        item.updated_at = Utc::now();
        
        self.db.update_inventory_item(&item).await?;
        
        Ok(item)
    }
    
    pub async fn adjust_inventory(
        &self,
        item_id: Uuid,
        req: &AdjustInventoryRequest,
        adjusted_by: Uuid,
    ) -> Result<InventoryItem> {
        let mut item = self.db.get_inventory_item_by_id(item_id).await?
            .ok_or_else(|| anyhow::anyhow!("Inventory item not found"))?;
        
        let previous_quantity = item.quantity;
        let new_quantity = match req.adjustment_type {
            AdjustmentType::Increase => item.quantity + req.quantity,
            AdjustmentType::Decrease => (item.quantity - req.quantity).max(0),
            AdjustmentType::SetTo => req.quantity,
        };
        
        let quantity_change = new_quantity - previous_quantity;
        
        item.quantity = new_quantity;
        item.available_quantity = item.quantity - item.reserved_quantity;
        item.updated_at = Utc::now();
        
        self.db.update_inventory_item(&item).await?;
        
        // 记录库存变动
        let movement_type = if quantity_change > 0 {
            MovementType::StockIn
        } else {
            MovementType::StockOut
        };
        
        self.record_movement(
            &item,
            movement_type,
            quantity_change.abs(),
            previous_quantity,
            new_quantity,
            req.reference_id,
            req.reference_type.clone(),
            req.reason.clone(),
            adjusted_by,
            req.notes.clone(),
        ).await?;
        
        Ok(item)
    }
    
    pub async fn reserve_inventory(
        &self,
        req: &ReserveInventoryRequest,
        user_id: Uuid,
    ) -> Result<InventoryReservation> {
        // 查找最优的库存项
        let item = self.find_best_inventory_item(
            req.product_id,
            req.warehouse_id,
            req.quantity,
        ).await?;
        
        if item.available_quantity < req.quantity {
            return Err(anyhow::anyhow!("Insufficient inventory"));
        }
        
        let reservation_duration = req.reservation_duration_minutes.unwrap_or(15);
        let expires_at = Utc::now() + chrono::Duration::minutes(reservation_duration as i64);
        
        let reservation = InventoryReservation {
            id: Uuid::new_v4(),
            inventory_item_id: item.id,
            order_id: req.order_id,
            user_id,
            quantity: req.quantity,
            status: ReservationStatus::Active,
            expires_at,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            released_at: None,
            metadata: None,
        };
        
        // 创建预约
        self.db.create_reservation(&reservation).await?;
        
        // 更新库存项
        let mut updated_item = item;
        updated_item.reserved_quantity += req.quantity;
        updated_item.available_quantity -= req.quantity;
        updated_item.updated_at = Utc::now();
        
        self.db.update_inventory_item(&updated_item).await?;
        
        // 记录库存变动
        self.record_movement(
            &updated_item,
            MovementType::Reserved,
            req.quantity,
            updated_item.available_quantity + req.quantity,
            updated_item.available_quantity,
            Some(req.order_id),
            Some("reservation".to_string()),
            format!("Reserved for order {}", req.order_id),
            user_id,
            None,
        ).await?;
        
        Ok(reservation)
    }
    
    pub async fn release_reservation(
        &self,
        req: &ReleaseReservationRequest,
        user_id: Uuid,
    ) -> Result<InventoryReservation> {
        let mut reservation = self.db.get_reservation_by_id(req.reservation_id).await?
            .ok_or_else(|| anyhow::anyhow!("Reservation not found"))?;
        
        if reservation.status != ReservationStatus::Active {
            return Err(anyhow::anyhow!("Reservation is not active"));
        }
        
        let release_quantity = req.quantity.unwrap_or(reservation.quantity);
        if release_quantity > reservation.quantity {
            return Err(anyhow::anyhow!("Release quantity exceeds reserved quantity"));
        }
        
        // 更新预约状态
        reservation.quantity -= release_quantity;
        reservation.status = if reservation.quantity == 0 {
            ReservationStatus::Released
        } else {
            ReservationStatus::Active
        };
        reservation.updated_at = Utc::now();
        
        if reservation.quantity == 0 {
            reservation.released_at = Some(Utc::now());
        }
        
        self.db.update_reservation(&reservation).await?;
        
        // 更新库存项
        let mut item = self.db.get_inventory_item_by_id(reservation.inventory_item_id).await?
            .ok_or_else(|| anyhow::anyhow!("Inventory item not found"))?;
        
        item.reserved_quantity -= release_quantity;
        item.available_quantity += release_quantity;
        item.updated_at = Utc::now();
        
        self.db.update_inventory_item(&item).await?;
        
        // 记录库存变动
        self.record_movement(
            &item,
            MovementType::Released,
            release_quantity,
            item.available_quantity - release_quantity,
            item.available_quantity,
            Some(reservation.order_id),
            Some("release".to_string()),
            req.reason.clone().unwrap_or_else(|| "Reservation released".to_string()),
            user_id,
            None,
        ).await?;
        
        Ok(reservation)
    }
    
    pub async fn fulfill_reservation(
        &self,
        reservation_id: Uuid,
        user_id: Uuid,
    ) -> Result<InventoryReservation> {
        let mut reservation = self.db.get_reservation_by_id(reservation_id).await?
            .ok_or_else(|| anyhow::anyhow!("Reservation not found"))?;
        
        if reservation.status != ReservationStatus::Active {
            return Err(anyhow::anyhow!("Reservation is not active"));
        }
        
        // 更新预约状态
        reservation.status = ReservationStatus::Fulfilled;
        reservation.updated_at = Utc::now();
        
        self.db.update_reservation(&reservation).await?;
        
        // 更新库存项
        let mut item = self.db.get_inventory_item_by_id(reservation.inventory_item_id).await?
            .ok_or_else(|| anyhow::anyhow!("Inventory item not found"))?;
        
        item.quantity -= reservation.quantity;
        item.reserved_quantity -= reservation.quantity;
        item.available_quantity = item.quantity - item.reserved_quantity;
        item.updated_at = Utc::now();
        
        self.db.update_inventory_item(&item).await?;
        
        // 记录库存变动
        self.record_movement(
            &item,
            MovementType::Sale,
            reservation.quantity,
            item.quantity + reservation.quantity,
            item.quantity,
            Some(reservation.order_id),
            Some("fulfillment".to_string()),
            format!("Fulfilled reservation for order {}", reservation.order_id),
            user_id,
            None,
        ).await?;
        
        Ok(reservation)
    }
    
    pub async fn get_inventory_item_by_id(&self, item_id: Uuid) -> Result<Option<InventoryItem>> {
        self.db.get_inventory_item_by_id(item_id).await
    }
    
    pub async fn get_inventory_items_by_product(
        &self,
        product_id: Uuid,
        warehouse_id: Option<Uuid>,
    ) -> Result<Vec<InventoryItem>> {
        self.db.get_inventory_items_by_product(product_id, warehouse_id).await
    }
    
    pub async fn get_low_stock_items(&self, warehouse_id: Option<Uuid>) -> Result<Vec<InventoryItem>> {
        self.db.get_low_stock_items(warehouse_id).await
    }
    
    pub async fn get_inventory_movements(
        &self,
        item_id: Uuid,
        limit: Option<u32>,
    ) -> Result<Vec<InventoryMovement>> {
        self.db.get_inventory_movements(item_id, limit).await
    }
    
    pub async fn expire_reservations(&self) -> Result<u32> {
        let expired_reservations = self.db.get_expired_reservations().await?;
        let mut count = 0;
        
        for mut reservation in expired_reservations {
            reservation.status = ReservationStatus::Expired;
            reservation.updated_at = Utc::now();
            
            self.db.update_reservation(&reservation).await?;
            
            // 释放库存
            if let Ok(Some(mut item)) = self.db.get_inventory_item_by_id(reservation.inventory_item_id).await {
                item.reserved_quantity -= reservation.quantity;
                item.available_quantity += reservation.quantity;
                item.updated_at = Utc::now();
                
                self.db.update_inventory_item(&item).await?;
            }
            
            count += 1;
        }
        
        Ok(count)
    }
    
    pub async fn list_inventory_items(
        &self,
        query: &crate::handlers::inventory_handler::InventoryQuery,
    ) -> Result<Vec<InventoryItem>> {
        self.db.list_inventory_items(query).await
    }
    
    pub async fn delete_inventory_item(&self, item_id: Uuid) -> Result<bool> {
        self.db.delete_inventory_item(item_id).await
    }
    
    pub async fn get_reservation_by_id(&self, reservation_id: Uuid) -> Result<Option<InventoryReservation>> {
        self.db.get_reservation_by_id(reservation_id).await
    }
    
    // 私有方法
    async fn find_best_inventory_item(
        &self,
        product_id: Uuid,
        warehouse_id: Option<Uuid>,
        quantity: i32,
    ) -> Result<InventoryItem> {
        let items = self.get_inventory_items_by_product(product_id, warehouse_id).await?;
        
        // 查找有足够库存的项
        for item in items {
            if item.available_quantity >= quantity && item.is_active {
                return Ok(item);
            }
        }
        
        Err(anyhow::anyhow!("No suitable inventory item found"))
    }
    
    async fn record_movement(
        &self,
        item: &InventoryItem,
        movement_type: MovementType,
        quantity: i32,
        previous_quantity: i32,
        new_quantity: i32,
        reference_id: Option<Uuid>,
        reference_type: Option<String>,
        reason: String,
        created_by: Uuid,
        notes: Option<String>,
    ) -> Result<()> {
        let movement = InventoryMovement {
            id: Uuid::new_v4(),
            inventory_item_id: item.id,
            movement_type,
            quantity,
            previous_quantity,
            new_quantity,
            reference_id,
            reference_type,
            reason,
            created_at: Utc::now(),
            created_by,
            notes,
            metadata: None,
        };
        
        self.db.create_inventory_movement(&movement).await
    }
} 