use crate::models::{EvaluationCreateRequest, EvaluationResponse};
use crate::services::evaluation_service::EvaluationService;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use validator::Validate;

pub async fn submit_evaluation(
    service: web::Data<EvaluationService>,
    request: web::Json<EvaluationCreateRequest>,
) -> Result<HttpResponse> {
    if let Err(errors) = request.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors
        })));
    }

    match service.submit_evaluation(request.into_inner()).await {
        Ok(evaluation) => {
            let response: EvaluationResponse = evaluation.into();
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => {
            tracing::error!("Failed to submit evaluation: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to submit evaluation"
            })))
        }
    }
}

pub async fn get_evaluation(
    service: web::Data<EvaluationService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let evaluation_id = path.into_inner();

    match service.get_evaluation(evaluation_id).await {
        Ok(Some(evaluation)) => {
            let response: EvaluationResponse = evaluation.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Evaluation not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to get evaluation: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get evaluation"
            })))
        }
    }
}

pub async fn get_evaluations_by_enterprise(
    service: web::Data<EvaluationService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let enterprise_id = path.into_inner();

    match service.get_evaluations_by_enterprise(enterprise_id).await {
        Ok(evaluations) => {
            let response: Vec<EvaluationResponse> = evaluations.into_iter().map(|e| e.into()).collect();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            tracing::error!("Failed to get evaluations by enterprise: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get evaluations"
            })))
        }
    }
}

pub async fn generate_evaluation_report(
    service: web::Data<EvaluationService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let enterprise_id = path.into_inner();

    match service.generate_evaluation_report(enterprise_id).await {
        Ok(report) => Ok(HttpResponse::Ok().json(report)),
        Err(e) => {
            tracing::error!("Failed to generate evaluation report: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to generate evaluation report"
            })))
        }
    }
}

pub async fn update_enterprise_multiplier(
    service: web::Data<EvaluationService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let enterprise_id = path.into_inner();

    match service.update_enterprise_multiplier(enterprise_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Multiplier updated successfully"
        }))),
        Err(e) => {
            tracing::error!("Failed to update enterprise multiplier: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update multiplier"
            })))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/evaluations")
            .route("", web::post().to(submit_evaluation))
            .route("/{id}", web::get().to(get_evaluation))
            .route("/enterprise/{enterprise_id}", web::get().to(get_evaluations_by_enterprise))
            .route("/enterprise/{enterprise_id}/report", web::get().to(generate_evaluation_report))
            .route("/enterprise/{enterprise_id}/update-multiplier", web::post().to(update_enterprise_multiplier))
    );
} 