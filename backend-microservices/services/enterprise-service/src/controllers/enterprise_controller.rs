use crate::models::{EnterpriseCreateRequest, EnterpriseUpdateRequest, EnterpriseResponse};
use crate::services::enterprise_service::EnterpriseService;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use validator::Validate;

pub async fn create_enterprise(
    service: web::Data<EnterpriseService>,
    request: web::Json<EnterpriseCreateRequest>,
) -> Result<HttpResponse> {
    // Validate request
    if let Err(errors) = request.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors
        })));
    }

    match service.create_enterprise(request.into_inner()).await {
        Ok(enterprise) => {
            let response: EnterpriseResponse = enterprise.into();
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => {
            tracing::error!("Failed to create enterprise: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create enterprise"
            })))
        }
    }
}

pub async fn get_enterprise(
    service: web::Data<EnterpriseService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let enterprise_id = path.into_inner();

    match service.get_enterprise(enterprise_id).await {
        Ok(Some(enterprise)) => {
            let response: EnterpriseResponse = enterprise.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Enterprise not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to get enterprise: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get enterprise"
            })))
        }
    }
}

pub async fn get_enterprises_by_owner(
    service: web::Data<EnterpriseService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let owner_id = path.into_inner();

    match service.get_enterprises_by_owner(owner_id).await {
        Ok(enterprises) => {
            let response: Vec<EnterpriseResponse> = enterprises.into_iter().map(|e| e.into()).collect();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            tracing::error!("Failed to get enterprises by owner: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get enterprises"
            })))
        }
    }
}

pub async fn update_enterprise(
    service: web::Data<EnterpriseService>,
    path: web::Path<Uuid>,
    request: web::Json<EnterpriseUpdateRequest>,
) -> Result<HttpResponse> {
    let enterprise_id = path.into_inner();

    // Validate request
    if let Err(errors) = request.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors
        })));
    }

    match service.update_enterprise(enterprise_id, request.into_inner()).await {
        Ok(Some(enterprise)) => {
            let response: EnterpriseResponse = enterprise.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Enterprise not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to update enterprise: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update enterprise"
            })))
        }
    }
}

pub async fn update_multiplier(
    service: web::Data<EnterpriseService>,
    path: web::Path<Uuid>,
    multiplier_data: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    let enterprise_id = path.into_inner();

    let multiplier = match multiplier_data.get("multiplier").and_then(|v| v.as_f64()) {
        Some(m) => m,
        None => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid multiplier value"
            })));
        }
    };

    match service.update_multiplier(enterprise_id, multiplier).await {
        Ok(Some(enterprise)) => {
            let response: EnterpriseResponse = enterprise.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Enterprise not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to update multiplier: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update multiplier"
            })))
        }
    }
}

pub async fn get_enterprise_stats(
    service: web::Data<EnterpriseService>,
) -> Result<HttpResponse> {
    match service.get_enterprise_stats().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(stats)),
        Err(e) => {
            tracing::error!("Failed to get enterprise stats: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get enterprise stats"
            })))
        }
    }
}

pub async fn deactivate_enterprise(
    service: web::Data<EnterpriseService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let enterprise_id = path.into_inner();

    match service.deactivate_enterprise(enterprise_id).await {
        Ok(Some(enterprise)) => {
            let response: EnterpriseResponse = enterprise.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Enterprise not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to deactivate enterprise: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to deactivate enterprise"
            })))
        }
    }
}

pub async fn activate_enterprise(
    service: web::Data<EnterpriseService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let enterprise_id = path.into_inner();

    match service.activate_enterprise(enterprise_id).await {
        Ok(Some(enterprise)) => {
            let response: EnterpriseResponse = enterprise.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Enterprise not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to activate enterprise: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to activate enterprise"
            })))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/enterprises")
            .route("", web::post().to(create_enterprise))
            .route("/stats", web::get().to(get_enterprise_stats))
            .route("/{id}", web::get().to(get_enterprise))
            .route("/{id}", web::put().to(update_enterprise))
            .route("/{id}/multiplier", web::put().to(update_multiplier))
            .route("/{id}/deactivate", web::post().to(deactivate_enterprise))
            .route("/{id}/activate", web::post().to(activate_enterprise))
            .route("/owner/{owner_id}", web::get().to(get_enterprises_by_owner))
    );
} 