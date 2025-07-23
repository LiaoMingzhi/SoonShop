use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::db::Database;
use crate::services::notification_service::NotificationService;
use crate::models::notification::{
    Notification, NotificationTemplate, UserNotificationPreference, NotificationSubscription,
    NotificationType, DeliveryStatus, DeliveryChannel, NotificationFrequency,
};
use crate::models::{NotificationChannel, NotificationStatus};
use crate::error::NotificationError;

// 查询参数
#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub notification_type: Option<NotificationType>,
    pub delivery_status: Option<DeliveryStatus>,
    pub delivery_channel: Option<DeliveryChannel>,
    pub read_status: Option<String>, // "read", "unread", "all"
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct TemplateQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub notification_type: Option<NotificationType>,
    pub is_active: Option<bool>,
}

// 请求结构
#[derive(Debug, Deserialize)]
pub struct CreateNotificationRequest {
    pub user_id: Uuid,
    pub template_id: Option<Uuid>,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
    pub delivery_channel: DeliveryChannel,
    pub scheduled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNotificationRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub data: Option<serde_json::Value>,
    pub scheduled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub notification_type: NotificationType,
    pub title_template: String,
    pub body_template: String,
    pub template_variables: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub title_template: Option<String>,
    pub body_template: Option<String>,
    pub template_variables: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PreviewTemplateRequest {
    pub variables: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub preferences: Vec<PreferenceUpdate>,
}

#[derive(Debug, Deserialize)]
pub struct PreferenceUpdate {
    pub notification_type: NotificationType,
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub in_app_enabled: Option<bool>,
    pub frequency: Option<NotificationFrequency>,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub endpoint: String,
    pub p256dh_key: String,
    pub auth_key: String,
    pub user_agent: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BulkNotificationRequest {
    pub user_ids: Vec<Uuid>,
    pub template_id: Option<Uuid>,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
    pub delivery_channel: DeliveryChannel,
    pub scheduled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct BulkMarkReadRequest {
    pub notification_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct BulkDeleteRequest {
    pub notification_ids: Vec<Uuid>,
}

// 响应结构
#[derive(Debug, Serialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<Notification>,
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    pub unread_count: u64,
}

#[derive(Debug, Serialize)]
pub struct TemplateListResponse {
    pub templates: Vec<NotificationTemplate>,
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[derive(Debug, Serialize)]
pub struct PreviewResponse {
    pub title: String,
    pub body: String,
    pub variables_used: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct StatisticsResponse {
    pub total_notifications: u64,
    pub delivery_stats: HashMap<DeliveryChannel, DeliveryStats>,
    pub type_stats: HashMap<NotificationType, TypeStats>,
    pub recent_activity: Vec<ActivityStat>,
}

#[derive(Debug, Serialize)]
pub struct DeliveryStats {
    pub total_sent: u64,
    pub total_delivered: u64,
    pub total_failed: u64,
    pub delivery_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct TypeStats {
    pub total_sent: u64,
    pub total_opened: u64,
    pub total_clicked: u64,
    pub open_rate: f64,
    pub click_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct ActivityStat {
    pub date: chrono::NaiveDate,
    pub notifications_sent: u64,
    pub notifications_delivered: u64,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

// 通知操作
pub async fn create_notification(
    db: web::Data<Database>,
    req: web::Json<CreateNotificationRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    
    let notification_req = CreateNotificationRequest {
        user_id: req.user_id,
        template_id: req.template_id,
        notification_type: req.notification_type.clone(),
        title: req.title.clone(),
        body: req.body.clone(),
        data: req.data.clone(),
        delivery_channel: req.delivery_channel.clone(),
        scheduled_at: req.scheduled_at,
    };
    
    match service.create_notification(&notification_req).await {
        Ok(notification) => Ok(HttpResponse::Created().json(notification)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Failed to create notification",
            "message": e.to_string()
        })))
    }
}

pub async fn list_notifications(
    db: web::Data<Database>,
    query: web::Query<NotificationQuery>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    
    let query_params = NotificationQuery {
        page: query.page,
        per_page: query.per_page,
        notification_type: query.notification_type.clone(),
        delivery_status: query.delivery_status.clone(),
        delivery_channel: query.delivery_channel.clone(),
        read_status: query.read_status.clone(),
        date_from: query.date_from,
        date_to: query.date_to,
        user_id: query.user_id,
    };
    
    match service.list_notifications(&query_params).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to list notifications",
            "message": e.to_string()
        })))
    }
}

pub async fn get_notification(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let notification_id = path.into_inner();
    
    match service.get_notification_by_id(notification_id).await {
        Ok(Some(notification)) => Ok(HttpResponse::Ok().json(notification)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Notification not found"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get notification",
            "message": e.to_string()
        })))
    }
}

pub async fn update_notification(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateNotificationRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let notification_id = path.into_inner();
    
    let update_req = UpdateNotificationRequest {
        title: req.title.clone(),
        body: req.body.clone(),
        data: req.data.clone(),
        scheduled_at: req.scheduled_at,
    };
    
    match service.update_notification(notification_id, &update_req).await {
        Ok(notification) => Ok(HttpResponse::Ok().json(notification)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Failed to update notification",
            "message": e.to_string()
        })))
    }
}

pub async fn delete_notification(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let notification_id = path.into_inner();
    
    match service.delete_notification(notification_id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to delete notification",
            "message": e.to_string()
        })))
    }
}

pub async fn mark_as_read(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let notification_id = path.into_inner();
    
    match service.mark_as_read(notification_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Notification marked as read"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to mark notification as read",
            "message": e.to_string()
        })))
    }
}

pub async fn mark_as_unread(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let notification_id = path.into_inner();
    
    match service.mark_as_unread(notification_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Notification marked as unread"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to mark notification as unread",
            "message": e.to_string()
        })))
    }
}

pub async fn bulk_mark_as_read(
    db: web::Data<Database>,
    req: web::Json<BulkMarkReadRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    
    match service.bulk_mark_as_read(&req.notification_ids).await {
        Ok(_) => {
            let count = req.notification_ids.len();
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": format!("Marked {} notifications as read", count)
            })))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to mark notifications as read",
            "message": e.to_string()
        })))
    }
}

pub async fn bulk_delete(
    db: web::Data<Database>,
    req: web::Json<BulkDeleteRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    
    match service.bulk_delete(&req.notification_ids).await {
        Ok(_) => {
            let count = req.notification_ids.len();
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": format!("Deleted {} notifications", count)
            })))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to delete notifications",
            "message": e.to_string()
        })))
    }
}

pub async fn send_bulk_notification(
    db: web::Data<Database>,
    req: web::Json<BulkNotificationRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    
    let bulk_req = BulkNotificationRequest {
        user_ids: req.user_ids.clone(),
        template_id: req.template_id,
        notification_type: req.notification_type.clone(),
        title: req.title.clone(),
        body: req.body.clone(),
        data: req.data.clone(),
        delivery_channel: req.delivery_channel.clone(),
        scheduled_at: req.scheduled_at,
    };
    
    match service.send_bulk_notification(&bulk_req).await {
        Ok(notifications) => Ok(HttpResponse::Created().json(serde_json::json!({
            "message": "Bulk notifications sent successfully",
            "count": notifications.len(),
            "notifications": notifications
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Failed to send bulk notifications",
            "message": e.to_string()
        })))
    }
}

// 模板操作
pub async fn create_template(
    db: web::Data<Database>,
    req: web::Json<CreateTemplateRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    
    let template_req = CreateTemplateRequest {
        name: req.name.clone(),
        notification_type: req.notification_type.clone(),
        title_template: req.title_template.clone(),
        body_template: req.body_template.clone(),
        template_variables: req.template_variables.clone(),
    };
    
    match service.create_template(&template_req).await {
        Ok(template) => Ok(HttpResponse::Created().json(template)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Failed to create template",
            "message": e.to_string()
        })))
    }
}

pub async fn list_templates(
    db: web::Data<Database>,
    query: web::Query<TemplateQuery>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    
    let query_params = TemplateQuery {
        page: query.page,
        per_page: query.per_page,
        notification_type: query.notification_type.clone(),
        is_active: query.is_active,
    };
    
    match service.list_templates(&query_params).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to list templates",
            "message": e.to_string()
        })))
    }
}

pub async fn get_template(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let template_id = path.into_inner();
    
    match service.get_template_by_id(template_id).await {
        Ok(Some(template)) => Ok(HttpResponse::Ok().json(template)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Template not found"
        }))),
        Err(e) => {
            log::error!("Failed to get template: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get template",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn update_template(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateTemplateRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let template_id = path.into_inner();
    
    match service.update_template(template_id, &req).await {
        Ok(template) => Ok(HttpResponse::Ok().json(template)),
        Err(e) => {
            log::error!("Failed to update template: {}", e);
            if e.to_string().contains("not found") {
                Ok(HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Template not found"
                })))
            } else {
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to update template",
                    "message": e.to_string()
                })))
            }
        }
    }
}

pub async fn delete_template(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let template_id = path.into_inner();
    
    match service.delete_template(template_id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Template not found"
        }))),
        Err(e) => {
            log::error!("Failed to delete template: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete template",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn preview_template(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
    req: web::Json<PreviewTemplateRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let template_id = path.into_inner();
    
    match service.preview_template(template_id, &req.variables).await {
        Ok((title, body, variables_used)) => {
            let response = PreviewResponse {
                title,
                body,
                variables_used,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to preview template: {}", e);
            if e.to_string().contains("not found") {
                Ok(HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Template not found"
                })))
            } else {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Failed to preview template",
                    "message": e.to_string()
                })))
            }
        }
    }
}

// 用户偏好
pub async fn get_user_preferences(
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let user_id = Uuid::new_v4(); // TODO: 从JWT token获取用户ID
    
    match service.get_user_preferences(user_id).await {
        Ok(preferences) => Ok(HttpResponse::Ok().json(preferences)),
        Err(e) => {
            log::error!("Failed to get user preferences: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get user preferences",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn update_user_preferences(
    db: web::Data<Database>,
    req: web::Json<UpdatePreferencesRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let user_id = Uuid::new_v4(); // TODO: 从JWT token获取用户ID
    
    match service.update_user_preferences(user_id, &req.preferences).await {
        Ok(preferences) => Ok(HttpResponse::Ok().json(preferences)),
        Err(e) => {
            log::error!("Failed to update user preferences: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update user preferences",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn reset_user_preferences(
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let user_id = Uuid::new_v4(); // TODO: 从JWT token获取用户ID
    
    match service.reset_user_preferences(user_id).await {
        Ok(preferences) => Ok(HttpResponse::Ok().json(preferences)),
        Err(e) => {
            log::error!("Failed to reset user preferences: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to reset user preferences",
                "message": e.to_string()
            })))
        }
    }
}

// 订阅管理
pub async fn list_subscriptions(
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let user_id = Uuid::new_v4(); // TODO: 从JWT token获取用户ID
    
    match service.list_user_subscriptions(user_id).await {
        Ok(subscriptions) => Ok(HttpResponse::Ok().json(subscriptions)),
        Err(e) => {
            log::error!("Failed to list subscriptions: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to list subscriptions",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn subscribe(
    db: web::Data<Database>,
    req: web::Json<SubscribeRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let user_id = Uuid::new_v4(); // TODO: 从JWT token获取用户ID
    
    match service.create_subscription(user_id, &req).await {
        Ok(subscription) => Ok(HttpResponse::Created().json(subscription)),
        Err(e) => {
            log::error!("Failed to create subscription: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create subscription",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn unsubscribe(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let subscription_id = path.into_inner();
    
    match service.delete_subscription(subscription_id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Subscription not found"
        }))),
        Err(e) => {
            log::error!("Failed to delete subscription: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete subscription",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn subscribe_push(
    db: web::Data<Database>,
    req: web::Json<SubscribeRequest>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    let user_id = Uuid::new_v4(); // TODO: 从JWT token获取用户ID
    
    match service.create_push_subscription(user_id, &req).await {
        Ok(subscription) => Ok(HttpResponse::Created().json(subscription)),
        Err(e) => {
            log::error!("Failed to create push subscription: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create push subscription",
                "message": e.to_string()
            })))
        }
    }
}

// 统计信息
pub async fn get_statistics(
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    
    match service.get_notification_statistics().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(stats)),
        Err(e) => {
            log::error!("Failed to get statistics: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get statistics",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn get_delivery_statistics(
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    
    match service.get_delivery_statistics().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(stats)),
        Err(e) => {
            log::error!("Failed to get delivery statistics: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get delivery statistics",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn get_engagement_statistics(
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let service = NotificationService::new(&db);
    
    match service.get_engagement_statistics().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(stats)),
        Err(e) => {
            log::error!("Failed to get engagement statistics: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get engagement statistics",
                "message": e.to_string()
            })))
        }
    }
}

// 健康检查
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
        version: "1.0.0".to_string(),
    }))
}

pub async fn send_notification(
    _notification_data: web::Json<SendNotificationRequest>,
) -> Result<HttpResponse, NotificationError> {
    // TODO: 实现发送通知逻辑
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Notification sent successfully"
    })))
}

#[derive(Debug, Serialize)]
pub struct SendNotificationRequest {
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub channel: NotificationChannel,
} 