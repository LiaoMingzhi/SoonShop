use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::notification::{
    Notification, NotificationPreference, NotificationRecipient, NotificationTemplate,
    SendNotificationRequest, SendBulkNotificationRequest, UpdateNotificationPreferenceRequest,
    UpdateRecipientRequest, NotificationStatus, NotificationChannel, NotificationType,
    NotificationPriority, NotificationStatsResponse, ChannelStats, TypeStats
};
use crate::db::Database;
use crate::providers::NotificationProviders;

pub struct NotificationService<'a> {
    db: &'a Database,
    providers: &'a NotificationProviders,
}

impl<'a> NotificationService<'a> {
    pub fn new(db: &'a Database, providers: &'a NotificationProviders) -> Self {
        Self { db, providers }
    }
    
    pub async fn send_notification(
        &self,
        req: &SendNotificationRequest,
    ) -> Result<Notification> {
        // 检查用户的通知偏好
        let preferences = self.db.get_user_preferences(req.recipient_id).await?;
        if !self.should_send_notification(&preferences, &req.notification_type, &req.channel) {
            return Err(anyhow::anyhow!("User has disabled this notification type"));
        }
        
        // 获取收件人信息
        let recipient = self.db.get_recipient(req.recipient_id).await?
            .ok_or_else(|| anyhow::anyhow!("Recipient not found"))?;
        
        // 处理模板
        let (title, content) = if let Some(template_id) = &req.template_id {
            self.render_template(template_id, &req.template_data, &req.channel).await?
        } else {
            (req.title.clone(), req.content.clone())
        };
        
        // 创建通知记录
        let notification = Notification {
            id: Uuid::new_v4(),
            recipient_id: req.recipient_id,
            notification_type: req.notification_type.clone(),
            channel: req.channel.clone(),
            priority: req.priority.clone(),
            title,
            content,
            template_id: req.template_id.clone(),
            template_data: req.template_data.clone(),
            status: if req.scheduled_at.is_some() {
                NotificationStatus::Scheduled
            } else {
                NotificationStatus::Pending
            },
            scheduled_at: req.scheduled_at,
            sent_at: None,
            delivered_at: None,
            read_at: None,
            error_message: None,
            retry_count: 0,
            max_retries: 3,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: req.metadata.clone(),
        };
        
        self.db.create_notification(&notification).await?;
        
        // 如果不是定时发送，立即发送
        if req.scheduled_at.is_none() {
            self.send_notification_now(&notification, &recipient).await?;
        }
        
        Ok(notification)
    }
    
    pub async fn send_bulk_notification(
        &self,
        req: &SendBulkNotificationRequest,
    ) -> Result<Vec<Notification>> {
        let mut notifications = Vec::new();
        
        for recipient_id in &req.recipient_ids {
            let notification_req = SendNotificationRequest {
                recipient_id: *recipient_id,
                notification_type: req.notification_type.clone(),
                channel: req.channel.clone(),
                priority: req.priority.clone(),
                title: req.title.clone(),
                content: req.content.clone(),
                template_id: req.template_id.clone(),
                template_data: req.template_data.clone(),
                scheduled_at: req.scheduled_at,
                metadata: req.metadata.clone(),
            };
            
            match self.send_notification(&notification_req).await {
                Ok(notification) => notifications.push(notification),
                Err(e) => {
                    log::error!("Failed to send notification to {}: {}", recipient_id, e);
                    continue;
                }
            }
        }
        
        Ok(notifications)
    }
    
    pub async fn get_notification_by_id(
        &self,
        notification_id: Uuid,
    ) -> Result<Option<Notification>> {
        self.db.get_notification_by_id(notification_id).await
    }
    
    pub async fn get_user_notifications(
        &self,
        user_id: Uuid,
        channel: Option<NotificationChannel>,
        status: Option<NotificationStatus>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Notification>> {
        self.db.get_user_notifications(user_id, channel, status, limit, offset).await
    }
    
    pub async fn mark_notification_as_read(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<Notification> {
        let mut notification = self.db.get_notification_by_id(notification_id).await?
            .ok_or_else(|| anyhow::anyhow!("Notification not found"))?;
        
        if notification.recipient_id != user_id {
            return Err(anyhow::anyhow!("Unauthorized"));
        }
        
        notification.status = NotificationStatus::Read;
        notification.read_at = Some(Utc::now());
        notification.updated_at = Utc::now();
        
        self.db.update_notification(&notification).await?;
        
        Ok(notification)
    }
    
    pub async fn cancel_notification(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<Notification> {
        let mut notification = self.db.get_notification_by_id(notification_id).await?
            .ok_or_else(|| anyhow::anyhow!("Notification not found"))?;
        
        if notification.recipient_id != user_id {
            return Err(anyhow::anyhow!("Unauthorized"));
        }
        
        if notification.status != NotificationStatus::Scheduled {
            return Err(anyhow::anyhow!("Only scheduled notifications can be cancelled"));
        }
        
        notification.status = NotificationStatus::Cancelled;
        notification.updated_at = Utc::now();
        
        self.db.update_notification(&notification).await?;
        
        Ok(notification)
    }
    
    pub async fn update_notification_preference(
        &self,
        user_id: Uuid,
        req: &UpdateNotificationPreferenceRequest,
    ) -> Result<NotificationPreference> {
        let preference = NotificationPreference {
            id: Uuid::new_v4(),
            user_id,
            notification_type: req.notification_type.clone(),
            channel: req.channel.clone(),
            enabled: req.enabled,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.db.upsert_notification_preference(&preference).await?;
        
        Ok(preference)
    }
    
    pub async fn get_notification_preferences(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<NotificationPreference>> {
        self.db.get_user_preferences(user_id).await
    }
    
    pub async fn update_recipient(
        &self,
        user_id: Uuid,
        req: &UpdateRecipientRequest,
    ) -> Result<NotificationRecipient> {
        let mut recipient = self.db.get_recipient(user_id).await?
            .unwrap_or_else(|| NotificationRecipient {
                id: Uuid::new_v4(),
                user_id,
                email: None,
                phone: None,
                push_token: None,
                platform: None,
                timezone: None,
                language: None,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        
        if let Some(email) = &req.email {
            recipient.email = Some(email.clone());
        }
        if let Some(phone) = &req.phone {
            recipient.phone = Some(phone.clone());
        }
        if let Some(push_token) = &req.push_token {
            recipient.push_token = Some(push_token.clone());
        }
        if let Some(platform) = &req.platform {
            recipient.platform = Some(platform.clone());
        }
        if let Some(timezone) = &req.timezone {
            recipient.timezone = Some(timezone.clone());
        }
        if let Some(language) = &req.language {
            recipient.language = Some(language.clone());
        }
        
        recipient.updated_at = Utc::now();
        
        self.db.upsert_recipient(&recipient).await?;
        
        Ok(recipient)
    }
    
    pub async fn get_notification_stats(
        &self,
        user_id: Option<Uuid>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<NotificationStatsResponse> {
        let stats = self.db.get_notification_stats(user_id, start_date, end_date).await?;
        
        let delivery_rate = if stats.total_sent > 0 {
            (stats.total_delivered as f64 / stats.total_sent as f64) * 100.0
        } else {
            0.0
        };
        
        let read_rate = if stats.total_delivered > 0 {
            (stats.total_read as f64 / stats.total_delivered as f64) * 100.0
        } else {
            0.0
        };
        
        Ok(NotificationStatsResponse {
            total_sent: stats.total_sent,
            total_delivered: stats.total_delivered,
            total_read: stats.total_read,
            total_failed: stats.total_failed,
            delivery_rate,
            read_rate,
            channel_stats: stats.channel_stats,
            type_stats: stats.type_stats,
        })
    }
    
    pub async fn process_scheduled_notifications(&self) -> Result<u32> {
        let scheduled = self.db.get_scheduled_notifications().await?;
        let mut processed = 0;
        
        for notification in scheduled {
            if let Ok(Some(recipient)) = self.db.get_recipient(notification.recipient_id).await {
                if let Err(e) = self.send_notification_now(&notification, &recipient).await {
                    log::error!("Failed to send scheduled notification {}: {}", notification.id, e);
                }
                processed += 1;
            }
        }
        
        Ok(processed)
    }
    
    pub async fn retry_failed_notifications(&self) -> Result<u32> {
        let failed = self.db.get_failed_notifications().await?;
        let mut retried = 0;
        
        for notification in failed {
            if notification.retry_count < notification.max_retries {
                if let Ok(Some(recipient)) = self.db.get_recipient(notification.recipient_id).await {
                    if let Err(e) = self.send_notification_now(&notification, &recipient).await {
                        log::error!("Failed to retry notification {}: {}", notification.id, e);
                    }
                    retried += 1;
                }
            }
        }
        
        Ok(retried)
    }
    
    // 私有方法
    async fn send_notification_now(
        &self,
        notification: &Notification,
        recipient: &NotificationRecipient,
    ) -> Result<()> {
        let mut updated_notification = notification.clone();
        updated_notification.status = NotificationStatus::Sent;
        updated_notification.sent_at = Some(Utc::now());
        updated_notification.retry_count += 1;
        updated_notification.updated_at = Utc::now();
        
        // 根据渠道发送通知
        let result = match notification.channel {
            NotificationChannel::Email => {
                self.send_email_notification(&notification, recipient).await
            }
            NotificationChannel::SMS => {
                self.send_sms_notification(&notification, recipient).await
            }
            NotificationChannel::Push => {
                self.send_push_notification(&notification, recipient).await
            }
            NotificationChannel::InApp => {
                self.send_in_app_notification(&notification, recipient).await
            }
            NotificationChannel::Webhook => {
                self.send_webhook_notification(&notification, recipient).await
            }
        };
        
        match result {
            Ok(_) => {
                updated_notification.status = NotificationStatus::Delivered;
                updated_notification.delivered_at = Some(Utc::now());
            }
            Err(e) => {
                updated_notification.status = NotificationStatus::Failed;
                updated_notification.error_message = Some(e.to_string());
            }
        }
        
        self.db.update_notification(&updated_notification).await?;
        
        Ok(())
    }
    
    async fn send_email_notification(
        &self,
        notification: &Notification,
        recipient: &NotificationRecipient,
    ) -> Result<()> {
        if let Some(email) = &recipient.email {
            self.providers.email.send_email(
                email,
                &notification.title,
                &notification.content,
            ).await?;
        } else {
            return Err(anyhow::anyhow!("Recipient has no email address"));
        }
        
        Ok(())
    }
    
    async fn send_sms_notification(
        &self,
        notification: &Notification,
        recipient: &NotificationRecipient,
    ) -> Result<()> {
        if let Some(phone) = &recipient.phone {
            self.providers.sms.send_sms(
                phone,
                &notification.content,
            ).await?;
        } else {
            return Err(anyhow::anyhow!("Recipient has no phone number"));
        }
        
        Ok(())
    }
    
    async fn send_push_notification(
        &self,
        notification: &Notification,
        recipient: &NotificationRecipient,
    ) -> Result<()> {
        if let Some(push_token) = &recipient.push_token {
            self.providers.push.send_push(
                push_token,
                &notification.title,
                &notification.content,
                recipient.platform.as_deref(),
            ).await?;
        } else {
            return Err(anyhow::anyhow!("Recipient has no push token"));
        }
        
        Ok(())
    }
    
    async fn send_in_app_notification(
        &self,
        notification: &Notification,
        recipient: &NotificationRecipient,
    ) -> Result<()> {
        // 应用内通知不需要外部发送，只需要标记为已发送
        Ok(())
    }
    
    async fn send_webhook_notification(
        &self,
        notification: &Notification,
        recipient: &NotificationRecipient,
    ) -> Result<()> {
        // 发送webhook通知
        self.providers.webhook.send_webhook(
            &notification.title,
            &notification.content,
            notification.metadata.as_ref(),
        ).await?;
        
        Ok(())
    }
    
    fn should_send_notification(
        &self,
        preferences: &[NotificationPreference],
        notification_type: &NotificationType,
        channel: &NotificationChannel,
    ) -> bool {
        for pref in preferences {
            if pref.notification_type == *notification_type && pref.channel == *channel {
                return pref.enabled;
            }
        }
        
        // 默认允许发送
        true
    }
    
    async fn render_template(
        &self,
        template_id: &str,
        template_data: &Option<serde_json::Value>,
        channel: &NotificationChannel,
    ) -> Result<(String, String)> {
        let template = self.db.get_template(template_id).await?
            .ok_or_else(|| anyhow::anyhow!("Template not found"))?;
        
        if template.channel != *channel {
            return Err(anyhow::anyhow!("Template channel mismatch"));
        }
        
        let data = template_data.as_ref().unwrap_or(&serde_json::json!({}));
        
        // 使用模板引擎渲染
        let title = self.render_template_string(&template.subject_template, data)?;
        let content = self.render_template_string(&template.content_template, data)?;
        
        Ok((title, content))
    }
    
    fn render_template_string(&self, template: &str, data: &serde_json::Value) -> Result<String> {
        // 这里可以使用handlebars或其他模板引擎
        // 简化实现
        Ok(template.to_string())
    }
} 