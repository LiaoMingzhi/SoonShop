use actix_web::{web, App, HttpServer, HttpResponse, Result, middleware::Logger};
use actix_cors::Cors;
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🏠 Housing Service starting on 0.0.0.0:8013");
    
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
                web::scope("/api/v1/housing")
                    .route("/health", web::get().to(health_check))
                    .route("/properties/search", web::get().to(search_properties))
                    .route("/rentals", web::post().to(create_rental))
                    .route("/rentals/{id}", web::get().to(get_rental))
                    .route("/rentals/{id}/contract", web::get().to(generate_contract))
            )
    })
    .bind("0.0.0.0:8013")?
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "housing-service",
        "timestamp": chrono::Utc::now()
    })))
}

async fn search_properties() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "properties": [],
            "total_count": 0
        }
    })))
}

async fn create_rental() -> Result<HttpResponse> {
    Ok(HttpResponse::Created().json(json!({
        "success": true,
        "message": "住房租赁申请已创建",
        "data": {
            "rental_id": uuid::Uuid::new_v4(),
            "status": "pending"
        }
    })))
}

async fn get_rental() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "rental": {
                "id": uuid::Uuid::new_v4(),
                "status": "active",
                "property": "示例房产"
            }
        }
    })))
}

async fn generate_contract() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "租赁合约已生成",
        "data": {
            "contract_id": uuid::Uuid::new_v4(),
            "contract_url": "https://example.com/contract.pdf"
        }
    })))
} 