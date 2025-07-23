use actix_web::{web, App, HttpServer, HttpResponse, Result, middleware::Logger};
use actix_cors::Cors;
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🏥 Healthcare Service starting on 0.0.0.0:8012");
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1/healthcare")
                    .route("/health", web::get().to(health_check))
                    .route("/appointments", web::post().to(create_appointment))
                    .route("/appointments/{id}", web::get().to(get_appointment))
                    .route("/appointments/{id}/cancel", web::post().to(cancel_appointment))
                    .route("/doctors/{id}/schedule", web::get().to(get_doctor_schedule))
            )
    })
    .bind("0.0.0.0:8012")?
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "healthcare-service",
        "timestamp": chrono::Utc::now()
    })))
}

async fn create_appointment() -> Result<HttpResponse> {
    Ok(HttpResponse::Created().json(json!({
        "success": true,
        "message": "医疗预约已创建",
        "data": {
            "appointment_id": uuid::Uuid::new_v4(),
            "status": "pending"
        }
    })))
}

async fn get_appointment() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "appointment": {
                "id": uuid::Uuid::new_v4(),
                "status": "confirmed",
                "doctor": "Dr. Smith",
                "date": chrono::Utc::now()
            }
        }
    })))
}

async fn cancel_appointment() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "医疗预约已取消"
    })))
}

async fn get_doctor_schedule() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "available_slots": []
        }
    })))
} 