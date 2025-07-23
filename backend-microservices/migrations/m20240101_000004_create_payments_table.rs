use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建支付表
        manager
            .create_table(
                Table::create()
                    .table(Payment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Payment::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Payment::PaymentNumber).string().unique_key().not_null())
                    .col(ColumnDef::new(Payment::OrderId).uuid().not_null())
                    .col(ColumnDef::new(Payment::UserId).uuid().not_null())
                    .col(ColumnDef::new(Payment::Amount).decimal().not_null())
                    .col(ColumnDef::new(Payment::Currency).string().default("USD"))
                    .col(ColumnDef::new(Payment::PaymentMethod).string().not_null())
                    .col(ColumnDef::new(Payment::PaymentProvider).string().not_null())
                    .col(ColumnDef::new(Payment::Status).string().default("pending"))
                    .col(ColumnDef::new(Payment::ProviderTransactionId).string())
                    .col(ColumnDef::new(Payment::ProviderPaymentId).string())
                    .col(ColumnDef::new(Payment::WalletAddress).string())
                    .col(ColumnDef::new(Payment::TransactionHash).string())
                    .col(ColumnDef::new(Payment::BlockNumber).big_integer())
                    .col(ColumnDef::new(Payment::ConfirmationCount).integer().default(0))
                    .col(ColumnDef::new(Payment::PaymentIntentId).string())
                    .col(ColumnDef::new(Payment::FailureReason).text())
                    .col(ColumnDef::new(Payment::ProcessingFee).decimal().default(0))
                    .col(ColumnDef::new(Payment::NetAmount).decimal())
                    .col(ColumnDef::new(Payment::Metadata).json())
                    .col(ColumnDef::new(Payment::AuthorizedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Payment::CapturedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Payment::FailedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Payment::RefundedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Payment::ExpiresAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Payment::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Payment::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建支付事件日志表
        manager
            .create_table(
                Table::create()
                    .table(PaymentEvent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaymentEvent::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PaymentEvent::PaymentId).uuid().not_null())
                    .col(ColumnDef::new(PaymentEvent::EventType).string().not_null())
                    .col(ColumnDef::new(PaymentEvent::EventData).json())
                    .col(ColumnDef::new(PaymentEvent::ProviderEventId).string())
                    .col(ColumnDef::new(PaymentEvent::IsProcessed).boolean().default(false))
                    .col(ColumnDef::new(PaymentEvent::ProcessedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(PaymentEvent::ErrorMessage).text())
                    .col(
                        ColumnDef::new(PaymentEvent::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_event_payment")
                            .from(PaymentEvent::Table, PaymentEvent::PaymentId)
                            .to(Payment::Table, Payment::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建支付方式表
        manager
            .create_table(
                Table::create()
                    .table(PaymentMethod::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaymentMethod::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PaymentMethod::UserId).uuid().not_null())
                    .col(ColumnDef::new(PaymentMethod::Type).string().not_null())
                    .col(ColumnDef::new(PaymentMethod::Provider).string().not_null())
                    .col(ColumnDef::new(PaymentMethod::ProviderMethodId).string())
                    .col(ColumnDef::new(PaymentMethod::WalletAddress).string())
                    .col(ColumnDef::new(PaymentMethod::CardLastFour).string())
                    .col(ColumnDef::new(PaymentMethod::CardBrand).string())
                    .col(ColumnDef::new(PaymentMethod::CardExpiryMonth).integer())
                    .col(ColumnDef::new(PaymentMethod::CardExpiryYear).integer())
                    .col(ColumnDef::new(PaymentMethod::IsDefault).boolean().default(false))
                    .col(ColumnDef::new(PaymentMethod::IsActive).boolean().default(true))
                    .col(ColumnDef::new(PaymentMethod::Metadata).json())
                    .col(
                        ColumnDef::new(PaymentMethod::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PaymentMethod::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建钱包余额表
        manager
            .create_table(
                Table::create()
                    .table(WalletBalance::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WalletBalance::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WalletBalance::UserId).uuid().not_null())
                    .col(ColumnDef::new(WalletBalance::Currency).string().not_null())
                    .col(ColumnDef::new(WalletBalance::AvailableBalance).decimal().default(0))
                    .col(ColumnDef::new(WalletBalance::PendingBalance).decimal().default(0))
                    .col(ColumnDef::new(WalletBalance::FrozenBalance).decimal().default(0))
                    .col(ColumnDef::new(WalletBalance::TotalBalance).decimal().default(0))
                    .col(
                        ColumnDef::new(WalletBalance::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(WalletBalance::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建钱包交易表
        manager
            .create_table(
                Table::create()
                    .table(WalletTransaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WalletTransaction::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WalletTransaction::WalletBalanceId).uuid().not_null())
                    .col(ColumnDef::new(WalletTransaction::PaymentId).uuid())
                    .col(ColumnDef::new(WalletTransaction::Type).string().not_null())
                    .col(ColumnDef::new(WalletTransaction::Amount).decimal().not_null())
                    .col(ColumnDef::new(WalletTransaction::PreviousBalance).decimal().not_null())
                    .col(ColumnDef::new(WalletTransaction::NewBalance).decimal().not_null())
                    .col(ColumnDef::new(WalletTransaction::Description).text())
                    .col(ColumnDef::new(WalletTransaction::ReferenceId).string())
                    .col(ColumnDef::new(WalletTransaction::Metadata).json())
                    .col(
                        ColumnDef::new(WalletTransaction::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_wallet_transaction_balance")
                            .from(WalletTransaction::Table, WalletTransaction::WalletBalanceId)
                            .to(WalletBalance::Table, WalletBalance::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_wallet_transaction_payment")
                            .from(WalletTransaction::Table, WalletTransaction::PaymentId)
                            .to(Payment::Table, Payment::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_payment_number")
                    .table(Payment::Table)
                    .col(Payment::PaymentNumber)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_payment_order_id")
                    .table(Payment::Table)
                    .col(Payment::OrderId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_payment_user_id")
                    .table(Payment::Table)
                    .col(Payment::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_payment_status")
                    .table(Payment::Table)
                    .col(Payment::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_payment_provider_transaction_id")
                    .table(Payment::Table)
                    .col(Payment::ProviderTransactionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_payment_transaction_hash")
                    .table(Payment::Table)
                    .col(Payment::TransactionHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_payment_method_user_id")
                    .table(PaymentMethod::Table)
                    .col(PaymentMethod::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_wallet_balance_user_currency")
                    .table(WalletBalance::Table)
                    .col(WalletBalance::UserId)
                    .col(WalletBalance::Currency)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_wallet_transaction_balance_id")
                    .table(WalletTransaction::Table)
                    .col(WalletTransaction::WalletBalanceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WalletTransaction::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(WalletBalance::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PaymentMethod::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PaymentEvent::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Payment::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Payment {
    Table,
    Id,
    PaymentNumber,
    OrderId,
    UserId,
    Amount,
    Currency,
    PaymentMethod,
    PaymentProvider,
    Status,
    ProviderTransactionId,
    ProviderPaymentId,
    WalletAddress,
    TransactionHash,
    BlockNumber,
    ConfirmationCount,
    PaymentIntentId,
    FailureReason,
    ProcessingFee,
    NetAmount,
    Metadata,
    AuthorizedAt,
    CapturedAt,
    FailedAt,
    RefundedAt,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum PaymentEvent {
    Table,
    Id,
    PaymentId,
    EventType,
    EventData,
    ProviderEventId,
    IsProcessed,
    ProcessedAt,
    ErrorMessage,
    CreatedAt,
}

#[derive(Iden)]
enum PaymentMethod {
    Table,
    Id,
    UserId,
    Type,
    Provider,
    ProviderMethodId,
    WalletAddress,
    CardLastFour,
    CardBrand,
    CardExpiryMonth,
    CardExpiryYear,
    IsDefault,
    IsActive,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum WalletBalance {
    Table,
    Id,
    UserId,
    Currency,
    AvailableBalance,
    PendingBalance,
    FrozenBalance,
    TotalBalance,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum WalletTransaction {
    Table,
    Id,
    WalletBalanceId,
    PaymentId,
    Type,
    Amount,
    PreviousBalance,
    NewBalance,
    Description,
    ReferenceId,
    Metadata,
    CreatedAt,
} 