use actix_web::{web, App, HttpServer, HttpResponse, Result, middleware::Logger};
use actix_cors::Cors;
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🎓 Education Service starting on 0.0.0.0:8014");
    
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
                web::scope("/api/v1/education")
                    .route("/health", web::get().to(health_check))
                    .route("/courses/search", web::get().to(search_courses))
                    .route("/enrollments", web::post().to(create_enrollment))
                    .route("/enrollments/{id}", web::get().to(get_enrollment))
                    .route("/courses/{id}/capacity", web::get().to(check_course_capacity))
            )
    })
    .bind("0.0.0.0:8014")?
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "education-service",
        "timestamp": chrono::Utc::now()
    })))
}

async fn search_courses() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "courses": [],
            "total_count": 0
        }
    })))
}

async fn create_enrollment() -> Result<HttpResponse> {
    Ok(HttpResponse::Created().json(json!({
        "success": true,
        "message": "课程报名已创建",
        "data": {
            "enrollment_id": uuid::Uuid::new_v4(),
            "status": "pending"
        }
    })))
}

async fn get_enrollment() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "enrollment": {
                "id": uuid::Uuid::new_v4(),
                "status": "confirmed",
                "course": "示例课程"
            }
        }
    })))
}

async fn check_course_capacity() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "total_capacity": 30,
            "enrolled_count": 15,
            "available_slots": 15
        }
    })))
} 