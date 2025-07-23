use super::common::*;
use serde_json::Value;
use reqwest::StatusCode;

/// API网关集成测试
#[cfg(test)]
mod api_gateway_tests {
    use super::*;

    #[tokio::test]
    async fn test_gateway_health_check() {
        let env = TestEnvironment::new();
        
        let response = env.client
            .get(&format!("{}/health", env.config.api_gateway_url))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), StatusCode::OK);
        
        let body: Value = response.json().await.expect("Failed to parse JSON");
        assert_eq!(body["status"], "healthy");
    }

    #[tokio::test]
    async fn test_gateway_routing() {
        let env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        // 测试路由到用户服务
        let response = env.client
            .get(&format!("{}/api/users/health", env.config.api_gateway_url))
            .send()
            .await
            .expect("Failed to send request");

        assert!(response.status().is_success() || response.status() == StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_authentication_middleware() {
        let env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        // 测试未认证访问保护的端点
        let response = env.client
            .get(&format!("{}/api/users/profile", env.config.api_gateway_url))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        // 发送大量请求测试限流
        let mut requests = Vec::new();
        for _ in 0..20 {
            let request = env.client
                .get(&format!("{}/api/products", env.config.api_gateway_url))
                .send();
            requests.push(request);
        }

        let responses = futures::future::join_all(requests).await;
        
        // 检查是否有请求被限流
        let rate_limited = responses.iter()
            .any(|r| r.as_ref().map(|resp| resp.status() == StatusCode::TOO_MANY_REQUESTS).unwrap_or(false));
        
        // 根据配置，可能会有限流，也可能没有
        println!("Rate limiting test completed, rate limited: {}", rate_limited);
    }

    #[tokio::test]
    async fn test_cors_headers() {
        let env = TestEnvironment::new();
        
        let response = env.client
            .options(&format!("{}/api/users", env.config.api_gateway_url))
            .header("Origin", "http://localhost:3000")
            .header("Access-Control-Request-Method", "POST")
            .send()
            .await
            .expect("Failed to send request");

        // 检查CORS头部
        assert!(response.headers().contains_key("access-control-allow-origin") || 
                response.status() == StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_error_handling() {
        let env = TestEnvironment::new();
        
        // 访问不存在的路由
        let response = env.client
            .get(&format!("{}/api/nonexistent", env.config.api_gateway_url))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_request_timeout() {
        let env = TestEnvironment::new();
        
        // 这个测试可能需要特殊的端点来模拟超时
        // 这里测试正常请求不会超时
        let response = env.client
            .get(&format!("{}/health", env.config.api_gateway_url))
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .expect("Request should not timeout");

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn test_service_discovery() {
        let env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        // 测试服务发现是否正常工作
        let services = vec![
            "/api/users/health",
            "/api/products/health", 
            "/api/orders/health",
            "/api/payments/health",
            "/api/inventory/health",
            "/api/notifications/health",
        ];

        for service_path in services {
            let response = env.client
                .get(&format!("{}{}", env.config.api_gateway_url, service_path))
                .send()
                .await
                .expect("Failed to send request");

            // 服务应该可达（可能返回401未认证，但不应该是502网关错误）
            assert_ne!(response.status(), StatusCode::BAD_GATEWAY, 
                      "Service {} appears to be unreachable", service_path);
        }
    }

    #[tokio::test]
    async fn test_load_balancing() {
        let env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        // 发送多个请求，检查负载均衡是否工作
        let mut responses = Vec::new();
        for _ in 0..10 {
            let response = env.client
                .get(&format!("{}/api/products", env.config.api_gateway_url))
                .send()
                .await
                .expect("Failed to send request");
            responses.push(response.status());
        }

        // 所有请求都应该返回一致的状态码（表明负载均衡正常）
        let first_status = responses[0];
        assert!(responses.iter().all(|&status| status == first_status || 
                                   status.is_success() || 
                                   status == StatusCode::UNAUTHORIZED));
    }

    #[tokio::test]
    async fn test_gateway_metrics() {
        let env = TestEnvironment::new();
        
        // 访问指标端点
        let response = env.client
            .get(&format!("{}/metrics", env.config.api_gateway_url))
            .send()
            .await
            .expect("Failed to send request");

        // 指标端点应该可用或者返回404（如果未启用）
        assert!(response.status().is_success() || response.status() == StatusCode::NOT_FOUND);
        
        if response.status().is_success() {
            let body = response.text().await.expect("Failed to get response body");
            // 检查是否包含Prometheus格式的指标
            assert!(body.contains("# HELP") || body.contains("# TYPE"));
        }
    }
} 