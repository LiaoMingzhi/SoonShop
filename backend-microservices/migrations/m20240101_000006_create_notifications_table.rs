use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建通知模板表
        manager
            .create_table(
                Table::create()
                    .table(NotificationTemplate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NotificationTemplate::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NotificationTemplate::Name).string().not_null())
                    .col(ColumnDef::new(NotificationTemplate::Type).string().not_null())
                    .col(ColumnDef::new(NotificationTemplate::Subject).string())
                    .col(ColumnDef::new(NotificationTemplate::Content).text().not_null())
                    .col(ColumnDef::new(NotificationTemplate::Language).string().default("zh-CN"))
                    .col(ColumnDef::new(NotificationTemplate::Variables).json())
                    .col(ColumnDef::new(NotificationTemplate::IsActive).boolean().default(true))
                    .col(ColumnDef::new(NotificationTemplate::Version).integer().default(1))
                    .col(ColumnDef::new(NotificationTemplate::Metadata).json())
                    .col(
                        ColumnDef::new(NotificationTemplate::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(NotificationTemplate::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建通知表
        manager
            .create_table(
                Table::create()
                    .table(Notification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notification::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notification::UserId).uuid())
                    .col(ColumnDef::new(Notification::TemplateId).uuid())
                    .col(ColumnDef::new(Notification::Type).string().not_null())
                    .col(ColumnDef::new(Notification::Channel).string().not_null())
                    .col(ColumnDef::new(Notification::Recipient).string().not_null())
                    .col(ColumnDef::new(Notification::Subject).string())
                    .col(ColumnDef::new(Notification::Content).text().not_null())
                    .col(ColumnDef::new(Notification::Status).string().default("pending"))
                    .col(ColumnDef::new(Notification::Priority).string().default("normal"))
                    .col(ColumnDef::new(Notification::IsRead).boolean().default(false))
                    .col(ColumnDef::new(Notification::Variables).json())
                    .col(ColumnDef::new(Notification::RetryCount).integer().default(0))
                    .col(ColumnDef::new(Notification::MaxRetries).integer().default(3))
                    .col(ColumnDef::new(Notification::NextRetryAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Notification::SentAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Notification::DeliveredAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Notification::ReadAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Notification::FailedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Notification::ErrorMessage).text())
                    .col(ColumnDef::new(Notification::ExternalId).string())
                    .col(ColumnDef::new(Notification::ExternalStatus).string())
                    .col(ColumnDef::new(Notification::Metadata).json())
                    .col(
                        ColumnDef::new(Notification::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Notification::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_notification_template")
                            .from(Notification::Table, Notification::TemplateId)
                            .to(NotificationTemplate::Table, NotificationTemplate::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建用户通知偏好表
        manager
            .create_table(
                Table::create()
                    .table(UserNotificationPreference::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserNotificationPreference::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserNotificationPreference::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserNotificationPreference::NotificationType).string().not_null())
                    .col(ColumnDef::new(UserNotificationPreference::EmailEnabled).boolean().default(true))
                    .col(ColumnDef::new(UserNotificationPreference::SmsEnabled).boolean().default(false))
                    .col(ColumnDef::new(UserNotificationPreference::PushEnabled).boolean().default(true))
                    .col(ColumnDef::new(UserNotificationPreference::InAppEnabled).boolean().default(true))
                    .col(ColumnDef::new(UserNotificationPreference::Frequency).string().default("immediate"))
                    .col(ColumnDef::new(UserNotificationPreference::QuietHoursStart).time())
                    .col(ColumnDef::new(UserNotificationPreference::QuietHoursEnd).time())
                    .col(ColumnDef::new(UserNotificationPreference::Timezone).string())
                    .col(ColumnDef::new(UserNotificationPreference::Language).string().default("zh-CN"))
                    .col(
                        ColumnDef::new(UserNotificationPreference::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserNotificationPreference::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建通知订阅表
        manager
            .create_table(
                Table::create()
                    .table(NotificationSubscription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NotificationSubscription::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NotificationSubscription::UserId).uuid().not_null())
                    .col(ColumnDef::new(NotificationSubscription::Topic).string().not_null())
                    .col(ColumnDef::new(NotificationSubscription::Channel).string().not_null())
                    .col(ColumnDef::new(NotificationSubscription::Endpoint).string().not_null())
                    .col(ColumnDef::new(NotificationSubscription::AuthKey).string())
                    .col(ColumnDef::new(NotificationSubscription::P256dhKey).string())
                    .col(ColumnDef::new(NotificationSubscription::IsActive).boolean().default(true))
                    .col(ColumnDef::new(NotificationSubscription::DeviceInfo).json())
                    .col(ColumnDef::new(NotificationSubscription::LastUsedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(NotificationSubscription::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(NotificationSubscription::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建通知统计表
        manager
            .create_table(
                Table::create()
                    .table(NotificationStats::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NotificationStats::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NotificationStats::Date).date().not_null())
                    .col(ColumnDef::new(NotificationStats::NotificationType).string().not_null())
                    .col(ColumnDef::new(NotificationStats::Channel).string().not_null())
                    .col(ColumnDef::new(NotificationStats::TotalSent).integer().default(0))
                    .col(ColumnDef::new(NotificationStats::TotalDelivered).integer().default(0))
                    .col(ColumnDef::new(NotificationStats::TotalRead).integer().default(0))
                    .col(ColumnDef::new(NotificationStats::TotalFailed).integer().default(0))
                    .col(ColumnDef::new(NotificationStats::DeliveryRate).decimal())
                    .col(ColumnDef::new(NotificationStats::ReadRate).decimal())
                    .col(ColumnDef::new(NotificationStats::FailureRate).decimal())
                    .col(
                        ColumnDef::new(NotificationStats::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(NotificationStats::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_notification_template_type")
                    .table(NotificationTemplate::Table)
                    .col(NotificationTemplate::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_user_id")
                    .table(Notification::Table)
                    .col(Notification::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_type_status")
                    .table(Notification::Table)
                    .col(Notification::Type)
                    .col(Notification::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_channel")
                    .table(Notification::Table)
                    .col(Notification::Channel)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_recipient")
                    .table(Notification::Table)
                    .col(Notification::Recipient)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_status")
                    .table(Notification::Table)
                    .col(Notification::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_priority")
                    .table(Notification::Table)
                    .col(Notification::Priority)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_created_at")
                    .table(Notification::Table)
                    .col(Notification::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_notification_preference_user_type")
                    .table(UserNotificationPreference::Table)
                    .col(UserNotificationPreference::UserId)
                    .col(UserNotificationPreference::NotificationType)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_subscription_user_topic")
                    .table(NotificationSubscription::Table)
                    .col(NotificationSubscription::UserId)
                    .col(NotificationSubscription::Topic)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_stats_date_type")
                    .table(NotificationStats::Table)
                    .col(NotificationStats::Date)
                    .col(NotificationStats::NotificationType)
                    .col(NotificationStats::Channel)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NotificationStats::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(NotificationSubscription::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserNotificationPreference::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Notification::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(NotificationTemplate::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum NotificationTemplate {
    Table,
    Id,
    Name,
    Type,
    Subject,
    Content,
    Language,
    Variables,
    IsActive,
    Version,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Notification {
    Table,
    Id,
    UserId,
    TemplateId,
    Type,
    Channel,
    Recipient,
    Subject,
    Content,
    Status,
    Priority,
    IsRead,
    Variables,
    RetryCount,
    MaxRetries,
    NextRetryAt,
    SentAt,
    DeliveredAt,
    ReadAt,
    FailedAt,
    ErrorMessage,
    ExternalId,
    ExternalStatus,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum UserNotificationPreference {
    Table,
    Id,
    UserId,
    NotificationType,
    EmailEnabled,
    SmsEnabled,
    PushEnabled,
    InAppEnabled,
    Frequency,
    QuietHoursStart,
    QuietHoursEnd,
    Timezone,
    Language,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum NotificationSubscription {
    Table,
    Id,
    UserId,
    Topic,
    Channel,
    Endpoint,
    AuthKey,
    P256dhKey,
    IsActive,
    DeviceInfo,
    LastUsedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum NotificationStats {
    Table,
    Id,
    Date,
    NotificationType,
    Channel,
    TotalSent,
    TotalDelivered,
    TotalRead,
    TotalFailed,
    DeliveryRate,
    ReadRate,
    FailureRate,
    CreatedAt,
    UpdatedAt,
} 