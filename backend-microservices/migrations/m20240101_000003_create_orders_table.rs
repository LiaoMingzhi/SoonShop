use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建订单表
        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Order::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Order::OrderNumber).string().unique_key().not_null())
                    .col(ColumnDef::new(Order::UserId).uuid().not_null())
                    .col(ColumnDef::new(Order::Status).string().default("pending"))
                    .col(ColumnDef::new(Order::TotalAmount).decimal().not_null())
                    .col(ColumnDef::new(Order::Currency).string().default("USD"))
                    .col(ColumnDef::new(Order::ShippingAddress).json())
                    .col(ColumnDef::new(Order::BillingAddress).json())
                    .col(ColumnDef::new(Order::PaymentMethod).string())
                    .col(ColumnDef::new(Order::PaymentStatus).string().default("pending"))
                    .col(ColumnDef::new(Order::ShippingMethod).string())
                    .col(ColumnDef::new(Order::ShippingFee).decimal().default(0))
                    .col(ColumnDef::new(Order::TaxAmount).decimal().default(0))
                    .col(ColumnDef::new(Order::DiscountAmount).decimal().default(0))
                    .col(ColumnDef::new(Order::Notes).text())
                    .col(ColumnDef::new(Order::Metadata).json())
                    .col(ColumnDef::new(Order::OrderedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Order::ShippedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Order::DeliveredAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Order::CancelledAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Order::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Order::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建订单项表
        manager
            .create_table(
                Table::create()
                    .table(OrderItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderItem::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrderItem::OrderId).uuid().not_null())
                    .col(ColumnDef::new(OrderItem::ProductId).uuid().not_null())
                    .col(ColumnDef::new(OrderItem::ProductVariantId).uuid())
                    .col(ColumnDef::new(OrderItem::ProductName).string().not_null())
                    .col(ColumnDef::new(OrderItem::ProductSku).string())
                    .col(ColumnDef::new(OrderItem::UnitPrice).decimal().not_null())
                    .col(ColumnDef::new(OrderItem::Quantity).integer().not_null())
                    .col(ColumnDef::new(OrderItem::TotalPrice).decimal().not_null())
                    .col(ColumnDef::new(OrderItem::ProductSnapshot).json())
                    .col(
                        ColumnDef::new(OrderItem::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OrderItem::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_item_order")
                            .from(OrderItem::Table, OrderItem::OrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建订单状态历史表
        manager
            .create_table(
                Table::create()
                    .table(OrderStatusHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderStatusHistory::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrderStatusHistory::OrderId).uuid().not_null())
                    .col(ColumnDef::new(OrderStatusHistory::FromStatus).string())
                    .col(ColumnDef::new(OrderStatusHistory::ToStatus).string().not_null())
                    .col(ColumnDef::new(OrderStatusHistory::ChangedBy).uuid())
                    .col(ColumnDef::new(OrderStatusHistory::Reason).text())
                    .col(ColumnDef::new(OrderStatusHistory::Metadata).json())
                    .col(
                        ColumnDef::new(OrderStatusHistory::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_status_history_order")
                            .from(OrderStatusHistory::Table, OrderStatusHistory::OrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建退款表
        manager
            .create_table(
                Table::create()
                    .table(Refund::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Refund::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Refund::OrderId).uuid().not_null())
                    .col(ColumnDef::new(Refund::RefundNumber).string().unique_key().not_null())
                    .col(ColumnDef::new(Refund::Amount).decimal().not_null())
                    .col(ColumnDef::new(Refund::Currency).string().default("USD"))
                    .col(ColumnDef::new(Refund::Reason).text())
                    .col(ColumnDef::new(Refund::Status).string().default("pending"))
                    .col(ColumnDef::new(Refund::ProcessedBy).uuid())
                    .col(ColumnDef::new(Refund::ProcessedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Refund::Metadata).json())
                    .col(
                        ColumnDef::new(Refund::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Refund::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_refund_order")
                            .from(Refund::Table, Refund::OrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_order_number")
                    .table(Order::Table)
                    .col(Order::OrderNumber)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_order_user_id")
                    .table(Order::Table)
                    .col(Order::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_order_status")
                    .table(Order::Table)
                    .col(Order::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_order_payment_status")
                    .table(Order::Table)
                    .col(Order::PaymentStatus)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_order_created_at")
                    .table(Order::Table)
                    .col(Order::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_order_item_order_id")
                    .table(OrderItem::Table)
                    .col(OrderItem::OrderId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_order_item_product_id")
                    .table(OrderItem::Table)
                    .col(OrderItem::ProductId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_refund_number")
                    .table(Refund::Table)
                    .col(Refund::RefundNumber)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Refund::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(OrderStatusHistory::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(OrderItem::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Order {
    Table,
    Id,
    OrderNumber,
    UserId,
    Status,
    TotalAmount,
    Currency,
    ShippingAddress,
    BillingAddress,
    PaymentMethod,
    PaymentStatus,
    ShippingMethod,
    ShippingFee,
    TaxAmount,
    DiscountAmount,
    Notes,
    Metadata,
    OrderedAt,
    ShippedAt,
    DeliveredAt,
    CancelledAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum OrderItem {
    Table,
    Id,
    OrderId,
    ProductId,
    ProductVariantId,
    ProductName,
    ProductSku,
    UnitPrice,
    Quantity,
    TotalPrice,
    ProductSnapshot,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum OrderStatusHistory {
    Table,
    Id,
    OrderId,
    FromStatus,
    ToStatus,
    ChangedBy,
    Reason,
    Metadata,
    CreatedAt,
}

#[derive(Iden)]
enum Refund {
    Table,
    Id,
    OrderId,
    RefundNumber,
    Amount,
    Currency,
    Reason,
    Status,
    ProcessedBy,
    ProcessedAt,
    Metadata,
    CreatedAt,
    UpdatedAt,
} 