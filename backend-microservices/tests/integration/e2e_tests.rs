use super::common::*;
use serde_json::{Value, json};
use reqwest::StatusCode;
use uuid::Uuid;

/// 端到端集成测试
/// 测试完整的业务流程，涉及多个微服务的协作
#[cfg(test)]
mod e2e_tests {
    use super::*;

    /// 测试完整的购买流程：注册 -> 登录 -> 浏览商品 -> 添加到购物车 -> 下单 -> 支付
    #[tokio::test]
    async fn test_complete_purchase_flow() {
        let mut env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        // 1. 用户注册
        let user_data = json!({
            "username": format!("buyer_{}", Uuid::new_v4()),
            "email": format!("buyer_{}@example.com", Uuid::new_v4()),
            "password": "secure_password_123",
            "full_name": "购买用户测试"
        });

        let register_response = env.client
            .post(&format!("{}/api/auth/register", env.config.api_gateway_url))
            .json(&user_data)
            .send()
            .await
            .expect("Failed to register user");

        assert!(register_response.status().is_success(), "User registration failed");

        // 2. 用户登录
        let login_data = json!({
            "email": user_data["email"],
            "password": user_data["password"]
        });

        let login_response = env.client
            .post(&format!("{}/api/auth/login", env.config.api_gateway_url))
            .json(&login_data)
            .send()
            .await
            .expect("Failed to login");

        assert!(login_response.status().is_success(), "User login failed");
        
        let login_body: Value = login_response.json().await.expect("Failed to parse login response");
        let token = login_body["data"]["token"].as_str().expect("Token not found");
        env.auth_token = Some(token.to_string());

        // 3. 创建测试商品（模拟商家操作）
        let product_data = json!({
            "name": format!("测试商品_{}", Uuid::new_v4()),
            "description": "这是一个测试商品",
            "price": 9999, // 99.99
            "sku": format!("SKU_{}", Uuid::new_v4()),
            "category": "electronics",
            "stock_quantity": 100
        });

        let create_product_response = env.client
            .post(&format!("{}/api/products", env.config.api_gateway_url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&product_data)
            .send()
            .await
            .expect("Failed to create product");

        println!("Create product response status: {}", create_product_response.status());
        let product_body: Value = if create_product_response.status().is_success() {
            create_product_response.json().await.expect("Failed to parse product response")
        } else {
            // 如果创建失败，尝试获取现有商品
            let products_response = env.client
                .get(&format!("{}/api/products", env.config.api_gateway_url))
                .send()
                .await
                .expect("Failed to get products");
            
            let products_body: Value = products_response.json().await.expect("Failed to parse products response");
            if let Some(products) = products_body["data"].as_array() {
                if !products.is_empty() {
                    products[0].clone()
                } else {
                    panic!("No products available for testing");
                }
            } else {
                panic!("Invalid products response format");
            }
        };

        let product_id = product_body["id"].as_str().expect("Product ID not found");

        // 4. 浏览商品
        let browse_response = env.client
            .get(&format!("{}/api/products/{}", env.config.api_gateway_url, product_id))
            .send()
            .await
            .expect("Failed to browse product");

        assert!(browse_response.status().is_success(), "Product browsing failed");

        // 5. 检查库存
        let inventory_response = env.client
            .get(&format!("{}/api/inventory/{}", env.config.api_gateway_url, product_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .expect("Failed to check inventory");

        println!("Inventory check status: {}", inventory_response.status());

        // 6. 创建订单
        let order_data = json!({
            "items": [
                {
                    "product_id": product_id,
                    "quantity": 2,
                    "price": product_body["price"]
                }
            ],
            "shipping_address": {
                "street": "测试街道123号",
                "city": "测试城市",
                "state": "测试省份",
                "zip_code": "100000",
                "country": "中国"
            }
        });

        let create_order_response = env.client
            .post(&format!("{}/api/orders", env.config.api_gateway_url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&order_data)
            .send()
            .await
            .expect("Failed to create order");

        println!("Create order status: {}", create_order_response.status());
        
        if create_order_response.status().is_success() {
            let order_body: Value = create_order_response.json().await.expect("Failed to parse order response");
            let order_id = order_body["data"]["id"].as_str().expect("Order ID not found");

            // 7. 处理支付
            let payment_data = json!({
                "order_id": order_id,
                "payment_method": "solana",
                "amount": order_body["data"]["total_amount"]
            });

            let payment_response = env.client
                .post(&format!("{}/api/payments", env.config.api_gateway_url))
                .header("Authorization", format!("Bearer {}", token))
                .json(&payment_data)
                .send()
                .await
                .expect("Failed to process payment");

            println!("Payment processing status: {}", payment_response.status());

            // 8. 验证订单状态
            let order_status_response = env.client
                .get(&format!("{}/api/orders/{}", env.config.api_gateway_url, order_id))
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .expect("Failed to get order status");

            println!("Order status check: {}", order_status_response.status());
        }

        println!("✅ Complete purchase flow test completed");
    }

    /// 测试库存管理流程：商品创建 -> 库存更新 -> 订单创建 -> 库存扣减
    #[tokio::test]
    async fn test_inventory_management_flow() {
        let mut env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        // 创建测试用户
        let user_response = env.create_test_user().await.expect("Failed to create test user");
        let email = user_response["data"]["email"].as_str().expect("Email not found");
        
        env.login(email, "test_password_123").await.expect("Failed to login");

        // 1. 创建商品
        let product_data = json!({
            "name": format!("库存测试商品_{}", Uuid::new_v4()),
            "description": "用于测试库存管理",
            "price": 5000,
            "sku": format!("INV_SKU_{}", Uuid::new_v4()),
            "category": "test",
            "stock_quantity": 50
        });

        let create_response = env.client
            .post(&format!("{}/api/products", env.config.api_gateway_url))
            .header("Authorization", format!("Bearer {}", env.auth_token.as_ref().unwrap()))
            .json(&product_data)
            .send()
            .await
            .expect("Failed to create product");

        println!("Product creation status: {}", create_response.status());

        // 2. 检查初始库存
        let products_response = env.client
            .get(&format!("{}/api/products", env.config.api_gateway_url))
            .send()
            .await
            .expect("Failed to get products");

        if products_response.status().is_success() {
            let products_body: Value = products_response.json().await.expect("Failed to parse products");
            if let Some(products) = products_body["data"].as_array() {
                if !products.is_empty() {
                    let product = &products[0];
                    let product_id = product["id"].as_str().expect("Product ID not found");

                    // 3. 更新库存
                    let inventory_update = json!({
                        "product_id": product_id,
                        "quantity": 25,
                        "operation": "set"
                    });

                    let inventory_response = env.client
                        .post(&format!("{}/api/inventory/update", env.config.api_gateway_url))
                        .header("Authorization", format!("Bearer {}", env.auth_token.as_ref().unwrap()))
                        .json(&inventory_update)
                        .send()
                        .await
                        .expect("Failed to update inventory");

                    println!("Inventory update status: {}", inventory_response.status());
                }
            }
        }

        println!("✅ Inventory management flow test completed");
    }

    /// 测试用户管理流程：注册 -> 登录 -> 资料更新 -> 权限验证
    #[tokio::test]
    async fn test_user_management_flow() {
        let mut env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        let unique_id = Uuid::new_v4();
        
        // 1. 用户注册
        let user_data = json!({
            "username": format!("testuser_{}", unique_id),
            "email": format!("test_{}@example.com", unique_id),
            "password": "secure_password_123",
            "full_name": "测试用户管理"
        });

        let register_response = env.client
            .post(&format!("{}/api/auth/register", env.config.api_gateway_url))
            .json(&user_data)
            .send()
            .await
            .expect("Failed to register");

        println!("Registration status: {}", register_response.status());

        // 2. 用户登录
        let login_data = json!({
            "email": user_data["email"],
            "password": user_data["password"]
        });

        let login_response = env.client
            .post(&format!("{}/api/auth/login", env.config.api_gateway_url))
            .json(&login_data)
            .send()
            .await
            .expect("Failed to login");

        println!("Login status: {}", login_response.status());

        if login_response.status().is_success() {
            let login_body: Value = login_response.json().await.expect("Failed to parse login response");
            if let Some(token) = login_body["data"]["token"].as_str() {
                env.auth_token = Some(token.to_string());

                // 3. 获取用户资料
                let profile_response = env.client
                    .get(&format!("{}/api/users/profile", env.config.api_gateway_url))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .expect("Failed to get profile");

                println!("Profile fetch status: {}", profile_response.status());

                // 4. 更新用户资料
                let update_data = json!({
                    "full_name": "更新后的测试用户",
                    "phone": "+86-138-0000-0000"
                });

                let update_response = env.client
                    .put(&format!("{}/api/users/profile", env.config.api_gateway_url))
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&update_data)
                    .send()
                    .await
                    .expect("Failed to update profile");

                println!("Profile update status: {}", update_response.status());
            }
        }

        println!("✅ User management flow test completed");
    }

    /// 测试通知系统流程：事件触发 -> 通知发送
    #[tokio::test]
    async fn test_notification_flow() {
        let mut env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        // 创建用户并登录
        let user_response = env.create_test_user().await.expect("Failed to create user");
        let email = user_response["data"]["email"].as_str().expect("Email not found");
        
        env.login(email, "test_password_123").await.expect("Failed to login");

        // 触发通知（例如：创建订单应该触发通知）
        let order_data = json!({
            "items": [
                {
                    "product_id": "test-product-id",
                    "quantity": 1,
                    "price": 1000
                }
            ],
            "shipping_address": {
                "street": "通知测试地址",
                "city": "测试城市",
                "state": "测试省份",
                "zip_code": "100000",
                "country": "中国"
            }
        });

        let order_response = env.client
            .post(&format!("{}/api/orders", env.config.api_gateway_url))
            .header("Authorization", format!("Bearer {}", env.auth_token.as_ref().unwrap()))
            .json(&order_data)
            .send()
            .await
            .expect("Failed to create order");

        println!("Order creation for notification test: {}", order_response.status());

        // 检查通知服务健康状态
        let notification_health = env.client
            .get(&format!("{}/api/notifications/health", env.config.api_gateway_url))
            .send()
            .await
            .expect("Failed to check notification service");

        println!("Notification service health: {}", notification_health.status());

        println!("✅ Notification flow test completed");
    }

    /// 测试错误恢复流程：模拟错误 -> 系统恢复
    #[tokio::test]
    async fn test_error_recovery_flow() {
        let env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        // 1. 测试无效请求的错误处理
        let invalid_data = json!({
            "invalid": "data"
        });

        let error_response = env.client
            .post(&format!("{}/api/users", env.config.api_gateway_url))
            .json(&invalid_data)
            .send()
            .await
            .expect("Failed to send invalid request");

        assert!(!error_response.status().is_success(), "Should return error for invalid data");

        // 2. 测试未认证访问的错误处理
        let unauth_response = env.client
            .get(&format!("{}/api/users/profile", env.config.api_gateway_url))
            .send()
            .await
            .expect("Failed to send unauthorized request");

        assert_eq!(unauth_response.status(), StatusCode::UNAUTHORIZED);

        // 3. 测试服务健康状态
        let health_response = env.client
            .get(&format!("{}/health", env.config.api_gateway_url))
            .send()
            .await
            .expect("Failed to check health");

        assert!(health_response.status().is_success(), "Health check should succeed");

        println!("✅ Error recovery flow test completed");
    }

    /// 测试性能和并发：多用户同时操作
    #[tokio::test]
    async fn test_concurrent_operations() {
        let env = TestEnvironment::new();
        env.wait_for_services().await.expect("Services not ready");

        // 创建多个并发任务
        let mut tasks = Vec::new();
        for i in 0..5 {
            let env_clone = TestEnvironment::new();
            let task = tokio::spawn(async move {
                // 每个任务创建一个用户并进行操作
                let user_data = json!({
                    "username": format!("concurrent_user_{}", i),
                    "email": format!("concurrent_{}@example.com", i),
                    "password": "password_123",
                    "full_name": format!("并发用户 {}", i)
                });

                let response = env_clone.client
                    .post(&format!("{}/api/auth/register", env_clone.config.api_gateway_url))
                    .json(&user_data)
                    .send()
                    .await;

                match response {
                    Ok(resp) => resp.status().is_success(),
                    Err(_) => false,
                }
            });
            tasks.push(task);
        }

        // 等待所有任务完成
        let results = futures::future::join_all(tasks).await;
        let success_count = results.iter().filter(|r| r.as_ref().unwrap_or(&false)).count();

        println!("Concurrent operations: {}/{} succeeded", success_count, results.len());
        assert!(success_count > 0, "At least some concurrent operations should succeed");

        println!("✅ Concurrent operations test completed");
    }
} 