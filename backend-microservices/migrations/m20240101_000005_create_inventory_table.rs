use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建仓库表
        manager
            .create_table(
                Table::create()
                    .table(Warehouse::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Warehouse::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Warehouse::Name).string().not_null())
                    .col(ColumnDef::new(Warehouse::Code).string().unique_key().not_null())
                    .col(ColumnDef::new(Warehouse::Description).text())
                    .col(ColumnDef::new(Warehouse::Address).json())
                    .col(ColumnDef::new(Warehouse::ContactInfo).json())
                    .col(ColumnDef::new(Warehouse::IsActive).boolean().default(true))
                    .col(ColumnDef::new(Warehouse::Priority).integer().default(0))
                    .col(ColumnDef::new(Warehouse::Metadata).json())
                    .col(
                        ColumnDef::new(Warehouse::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Warehouse::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建库存表
        manager
            .create_table(
                Table::create()
                    .table(Inventory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Inventory::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Inventory::ProductId).uuid().not_null())
                    .col(ColumnDef::new(Inventory::ProductVariantId).uuid())
                    .col(ColumnDef::new(Inventory::WarehouseId).uuid().not_null())
                    .col(ColumnDef::new(Inventory::Sku).string().not_null())
                    .col(ColumnDef::new(Inventory::QuantityAvailable).integer().default(0))
                    .col(ColumnDef::new(Inventory::QuantityReserved).integer().default(0))
                    .col(ColumnDef::new(Inventory::QuantityOnOrder).integer().default(0))
                    .col(ColumnDef::new(Inventory::ReorderLevel).integer().default(10))
                    .col(ColumnDef::new(Inventory::MaxStockLevel).integer())
                    .col(ColumnDef::new(Inventory::Location).string())
                    .col(ColumnDef::new(Inventory::Cost).decimal())
                    .col(ColumnDef::new(Inventory::LastStockTake).timestamp_with_time_zone())
                    .col(ColumnDef::new(Inventory::Metadata).json())
                    .col(
                        ColumnDef::new(Inventory::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Inventory::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_warehouse")
                            .from(Inventory::Table, Inventory::WarehouseId)
                            .to(Warehouse::Table, Warehouse::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建库存预留表
        manager
            .create_table(
                Table::create()
                    .table(InventoryReservation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InventoryReservation::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(InventoryReservation::InventoryId).uuid().not_null())
                    .col(ColumnDef::new(InventoryReservation::OrderId).uuid())
                    .col(ColumnDef::new(InventoryReservation::ReservationNumber).string().unique_key().not_null())
                    .col(ColumnDef::new(InventoryReservation::Quantity).integer().not_null())
                    .col(ColumnDef::new(InventoryReservation::Status).string().default("reserved"))
                    .col(ColumnDef::new(InventoryReservation::ReservedBy).uuid())
                    .col(ColumnDef::new(InventoryReservation::ReservedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(InventoryReservation::ExpiresAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(InventoryReservation::ReleasedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(InventoryReservation::FulfilledAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(InventoryReservation::Metadata).json())
                    .col(
                        ColumnDef::new(InventoryReservation::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(InventoryReservation::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_reservation_inventory")
                            .from(InventoryReservation::Table, InventoryReservation::InventoryId)
                            .to(Inventory::Table, Inventory::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建库存变动历史表
        manager
            .create_table(
                Table::create()
                    .table(InventoryMovement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InventoryMovement::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(InventoryMovement::InventoryId).uuid().not_null())
                    .col(ColumnDef::new(InventoryMovement::MovementType).string().not_null())
                    .col(ColumnDef::new(InventoryMovement::Quantity).integer().not_null())
                    .col(ColumnDef::new(InventoryMovement::PreviousQuantity).integer().not_null())
                    .col(ColumnDef::new(InventoryMovement::NewQuantity).integer().not_null())
                    .col(ColumnDef::new(InventoryMovement::Reference).string())
                    .col(ColumnDef::new(InventoryMovement::ReferenceId).uuid())
                    .col(ColumnDef::new(InventoryMovement::Reason).text())
                    .col(ColumnDef::new(InventoryMovement::PerformedBy).uuid())
                    .col(ColumnDef::new(InventoryMovement::Cost).decimal())
                    .col(ColumnDef::new(InventoryMovement::Metadata).json())
                    .col(
                        ColumnDef::new(InventoryMovement::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_movement_inventory")
                            .from(InventoryMovement::Table, InventoryMovement::InventoryId)
                            .to(Inventory::Table, Inventory::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建供应商表
        manager
            .create_table(
                Table::create()
                    .table(Supplier::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Supplier::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Supplier::Name).string().not_null())
                    .col(ColumnDef::new(Supplier::Code).string().unique_key().not_null())
                    .col(ColumnDef::new(Supplier::ContactInfo).json())
                    .col(ColumnDef::new(Supplier::Address).json())
                    .col(ColumnDef::new(Supplier::PaymentTerms).string())
                    .col(ColumnDef::new(Supplier::DeliveryTerms).string())
                    .col(ColumnDef::new(Supplier::Rating).decimal())
                    .col(ColumnDef::new(Supplier::IsActive).boolean().default(true))
                    .col(ColumnDef::new(Supplier::Metadata).json())
                    .col(
                        ColumnDef::new(Supplier::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Supplier::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建采购订单表
        manager
            .create_table(
                Table::create()
                    .table(PurchaseOrder::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PurchaseOrder::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PurchaseOrder::OrderNumber).string().unique_key().not_null())
                    .col(ColumnDef::new(PurchaseOrder::SupplierId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrder::WarehouseId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrder::Status).string().default("draft"))
                    .col(ColumnDef::new(PurchaseOrder::TotalAmount).decimal().not_null())
                    .col(ColumnDef::new(PurchaseOrder::Currency).string().default("USD"))
                    .col(ColumnDef::new(PurchaseOrder::ExpectedDelivery).timestamp_with_time_zone())
                    .col(ColumnDef::new(PurchaseOrder::OrderedBy).uuid())
                    .col(ColumnDef::new(PurchaseOrder::ApprovedBy).uuid())
                    .col(ColumnDef::new(PurchaseOrder::ReceivedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(PurchaseOrder::Notes).text())
                    .col(ColumnDef::new(PurchaseOrder::Metadata).json())
                    .col(
                        ColumnDef::new(PurchaseOrder::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrder::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_purchase_order_supplier")
                            .from(PurchaseOrder::Table, PurchaseOrder::SupplierId)
                            .to(Supplier::Table, Supplier::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_purchase_order_warehouse")
                            .from(PurchaseOrder::Table, PurchaseOrder::WarehouseId)
                            .to(Warehouse::Table, Warehouse::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建采购订单项表
        manager
            .create_table(
                Table::create()
                    .table(PurchaseOrderItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PurchaseOrderItem::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PurchaseOrderItem::PurchaseOrderId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::ProductId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::ProductVariantId).uuid())
                    .col(ColumnDef::new(PurchaseOrderItem::Sku).string().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::Quantity).integer().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::UnitCost).decimal().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::TotalCost).decimal().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::ReceivedQuantity).integer().default(0))
                    .col(ColumnDef::new(PurchaseOrderItem::DamagedQuantity).integer().default(0))
                    .col(ColumnDef::new(PurchaseOrderItem::Metadata).json())
                    .col(
                        ColumnDef::new(PurchaseOrderItem::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrderItem::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_purchase_order_item_purchase_order")
                            .from(PurchaseOrderItem::Table, PurchaseOrderItem::PurchaseOrderId)
                            .to(PurchaseOrder::Table, PurchaseOrder::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_warehouse_code")
                    .table(Warehouse::Table)
                    .col(Warehouse::Code)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_inventory_product_warehouse")
                    .table(Inventory::Table)
                    .col(Inventory::ProductId)
                    .col(Inventory::WarehouseId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_inventory_sku")
                    .table(Inventory::Table)
                    .col(Inventory::Sku)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_inventory_reorder_level")
                    .table(Inventory::Table)
                    .col(Inventory::ReorderLevel)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_inventory_reservation_status")
                    .table(InventoryReservation::Table)
                    .col(InventoryReservation::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_inventory_movement_type")
                    .table(InventoryMovement::Table)
                    .col(InventoryMovement::MovementType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_supplier_code")
                    .table(Supplier::Table)
                    .col(Supplier::Code)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_purchase_order_number")
                    .table(PurchaseOrder::Table)
                    .col(PurchaseOrder::OrderNumber)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_purchase_order_status")
                    .table(PurchaseOrder::Table)
                    .col(PurchaseOrder::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PurchaseOrderItem::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PurchaseOrder::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Supplier::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(InventoryMovement::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(InventoryReservation::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Inventory::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Warehouse::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Warehouse {
    Table,
    Id,
    Name,
    Code,
    Description,
    Address,
    ContactInfo,
    IsActive,
    Priority,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Inventory {
    Table,
    Id,
    ProductId,
    ProductVariantId,
    WarehouseId,
    Sku,
    QuantityAvailable,
    QuantityReserved,
    QuantityOnOrder,
    ReorderLevel,
    MaxStockLevel,
    Location,
    Cost,
    LastStockTake,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum InventoryReservation {
    Table,
    Id,
    InventoryId,
    OrderId,
    ReservationNumber,
    Quantity,
    Status,
    ReservedBy,
    ReservedAt,
    ExpiresAt,
    ReleasedAt,
    FulfilledAt,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum InventoryMovement {
    Table,
    Id,
    InventoryId,
    MovementType,
    Quantity,
    PreviousQuantity,
    NewQuantity,
    Reference,
    ReferenceId,
    Reason,
    PerformedBy,
    Cost,
    Metadata,
    CreatedAt,
}

#[derive(Iden)]
enum Supplier {
    Table,
    Id,
    Name,
    Code,
    ContactInfo,
    Address,
    PaymentTerms,
    DeliveryTerms,
    Rating,
    IsActive,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum PurchaseOrder {
    Table,
    Id,
    OrderNumber,
    SupplierId,
    WarehouseId,
    Status,
    TotalAmount,
    Currency,
    ExpectedDelivery,
    OrderedBy,
    ApprovedBy,
    ReceivedAt,
    Notes,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum PurchaseOrderItem {
    Table,
    Id,
    PurchaseOrderId,
    ProductId,
    ProductVariantId,
    Sku,
    Quantity,
    UnitCost,
    TotalCost,
    ReceivedQuantity,
    DamagedQuantity,
    Metadata,
    CreatedAt,
    UpdatedAt,
} 