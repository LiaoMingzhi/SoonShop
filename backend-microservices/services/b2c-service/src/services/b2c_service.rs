use crate::models::*;
use crate::repositories::b2c_repository::B2CRepository;
use anyhow::Result;
use chrono::Utc;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

#[derive(Clone)]
pub struct B2CService {
    repository: B2CRepository,
    // 在实际实现中，这里会包含其他服务的客户端
    // product_service_client: ProductServiceClient,
    // inventory_service_client: InventoryServiceClient,
    // payment_service_client: PaymentServiceClient,
}

impl B2CService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repository: B2CRepository::new(db),
        }
    }

    // ============ 购物车管理 ============

    pub async fn add_to_cart(&self, request: AddToCartRequest) -> Result<ShoppingCart> {
        // 1. 验证商品信息
        let product_info = self.validate_product(&request.product_id).await?;
        
        // 2. 检查库存
        self.check_inventory(&request.product_id, request.quantity).await?;
        
        // 3. 获取或创建购物车
        let mut cart = self.get_or_create_cart(request.user_id).await?;
        
        // 4. 检查商品是否已在购物车中
        if let Some(existing_item) = cart.items.iter_mut()
            .find(|item| item.product_id == request.product_id) {
            // 更新数量
            existing_item.quantity += request.quantity;
            existing_item.total_price = existing_item.unit_price * existing_item.quantity as f64;
        } else {
            // 添加新商品
            let cart_item = CartItem {
                id: Uuid::new_v4(),
                cart_id: cart.id,
                product_id: request.product_id,
                product_name: product_info.name,
                product_image: product_info.image,
                quantity: request.quantity,
                unit_price: product_info.price,
                total_price: product_info.price * request.quantity as f64,
                product_attributes: request.product_attributes,
                added_at: Utc::now(),
            };
            cart.items.push(cart_item);
        }
        
        // 5. 重新计算总价
        self.recalculate_cart_totals(&mut cart);
        
        // 6. 保存购物车
        cart.updated_at = Utc::now();
        self.repository.update_cart(cart.clone()).await?;
        
        Ok(cart)
    }

    pub async fn remove_from_cart(&self, user_id: Uuid, item_id: Uuid) -> Result<ShoppingCart> {
        let mut cart = self.repository.get_cart_by_user(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("购物车不存在"))?;
        
        cart.items.retain(|item| item.id != item_id);
        self.recalculate_cart_totals(&mut cart);
        cart.updated_at = Utc::now();
        
        self.repository.update_cart(cart.clone()).await?;
        Ok(cart)
    }

    pub async fn update_cart_item(&self, user_id: Uuid, item_id: Uuid, request: UpdateCartItemRequest) -> Result<ShoppingCart> {
        let mut cart = self.repository.get_cart_by_user(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("购物车不存在"))?;
        
        if let Some(item) = cart.items.iter_mut().find(|item| item.id == item_id) {
            // 检查库存
            self.check_inventory(&item.product_id, request.quantity).await?;
            
            item.quantity = request.quantity;
            item.total_price = item.unit_price * request.quantity as f64;
        } else {
            return Err(anyhow::anyhow!("购物车商品不存在"));
        }
        
        self.recalculate_cart_totals(&mut cart);
        cart.updated_at = Utc::now();
        
        self.repository.update_cart(cart.clone()).await?;
        Ok(cart)
    }

    pub async fn get_cart(&self, user_id: Uuid) -> Result<ShoppingCartResponse> {
        let cart = self.get_or_create_cart(user_id).await?;
        let recommendations = self.get_cart_recommendations(&cart).await?;
        
        Ok(ShoppingCartResponse {
            cart,
            recommendations,
        })
    }

    pub async fn clear_cart(&self, user_id: Uuid) -> Result<()> {
        let mut cart = self.repository.get_cart_by_user(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("购物车不存在"))?;
        
        cart.items.clear();
        cart.total_amount = 0.0;
        cart.total_quantity = 0;
        cart.updated_at = Utc::now();
        
        self.repository.update_cart(cart).await?;
        Ok(())
    }

    // ============ 订单管理 ============

    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<OrderResponse> {
        // 1. 获取购物车
        let cart = self.repository.get_cart_by_id(request.cart_id).await?
            .ok_or_else(|| anyhow::anyhow!("购物车不存在"))?;
        
        if cart.user_id != request.user_id {
            return Err(anyhow::anyhow!("购物车不属于当前用户"));
        }
        
        if cart.items.is_empty() {
            return Err(anyhow::anyhow!("购物车为空"));
        }
        
        // 2. 再次检查库存
        for item in &cart.items {
            self.check_inventory(&item.product_id, item.quantity).await?;
        }
        
        // 3. 计算订单金额
        let subtotal = cart.total_amount;
        let shipping_fee = self.calculate_shipping_fee(&request.shipping_address, &cart).await?;
        let tax_amount = self.calculate_tax_amount(subtotal, &request.shipping_address).await?;
        let discount_amount = self.calculate_discount(&request.coupon_code, subtotal).await?;
        let total_amount = subtotal + shipping_fee + tax_amount - discount_amount;
        
        // 4. 创建订单
        let order = B2COrder {
            id: Uuid::new_v4(),
            user_id: request.user_id,
            order_number: self.generate_order_number().await?,
            status: OrderStatus::Pending,
            items: cart.items.into_iter().map(|cart_item| OrderItem {
                id: Uuid::new_v4(),
                order_id: Uuid::new_v4(), // 临时ID，创建后会更新
                product_id: cart_item.product_id,
                product_name: cart_item.product_name,
                product_image: cart_item.product_image,
                quantity: cart_item.quantity,
                unit_price: cart_item.unit_price,
                total_price: cart_item.total_price,
                product_attributes: cart_item.product_attributes,
            }).collect(),
            shipping_address: request.shipping_address,
            billing_address: request.billing_address,
            payment_method: request.payment_method,
            subtotal,
            shipping_fee,
            tax_amount,
            discount_amount,
            total_amount,
            currency: cart.currency,
            notes: request.notes,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            confirmed_at: None,
            shipped_at: None,
            delivered_at: None,
        };
        
        // 5. 保存订单
        let saved_order = self.repository.create_order(order).await?;
        
        // 6. 清空购物车
        self.clear_cart(request.user_id).await?;
        
        // 7. 预留库存
        for item in &saved_order.items {
            self.reserve_inventory(&item.product_id, item.quantity).await?;
        }
        
        Ok(saved_order.into())
    }

    pub async fn get_order(&self, user_id: Uuid, order_id: Uuid) -> Result<OrderResponse> {
        let order = self.repository.get_order_by_id(order_id).await?
            .ok_or_else(|| anyhow::anyhow!("订单不存在"))?;
        
        if order.user_id != user_id {
            return Err(anyhow::anyhow!("订单不属于当前用户"));
        }
        
        Ok(order.into())
    }

    pub async fn get_user_orders(&self, user_id: Uuid, page: u32, limit: u32) -> Result<Vec<OrderResponse>> {
        let orders = self.repository.get_orders_by_user(user_id, page, limit).await?;
        Ok(orders.into_iter().map(|order| order.into()).collect())
    }

    pub async fn cancel_order(&self, user_id: Uuid, order_id: Uuid) -> Result<OrderResponse> {
        let mut order = self.repository.get_order_by_id(order_id).await?
            .ok_or_else(|| anyhow::anyhow!("订单不存在"))?;
        
        if order.user_id != user_id {
            return Err(anyhow::anyhow!("订单不属于当前用户"));
        }
        
        match order.status {
            OrderStatus::Pending | OrderStatus::Confirmed => {
                order.status = OrderStatus::Cancelled;
                order.updated_at = Utc::now();
                
                // 释放库存
                for item in &order.items {
                    self.release_inventory(&item.product_id, item.quantity).await?;
                }
                
                let updated_order = self.repository.update_order(order).await?;
                Ok(updated_order.into())
            }
            _ => Err(anyhow::anyhow!("订单状态不允许取消")),
        }
    }

    // ============ 商品搜索和推荐 ============

    pub async fn search_products(&self, request: ProductSearchRequest) -> Result<ProductSearchResponse> {
        // 在实际实现中，这里会调用商品服务的搜索API
        // 现在返回模拟数据
        Ok(ProductSearchResponse {
            products: vec![],
            total_count: 0,
            page: request.page.unwrap_or(1),
            limit: request.limit.unwrap_or(10),
            has_more: false,
        })
    }

    pub async fn get_product_recommendations(&self, user_id: Uuid) -> Result<Vec<ProductRecommendation>> {
        // 基于用户浏览历史和购买记录生成推荐
        let browse_history = self.repository.get_browse_history(user_id, 50).await?;
        let purchase_history = self.repository.get_purchase_history(user_id, 20).await?;
        
        // 在实际实现中，这里会有复杂的推荐算法
        // 现在返回空列表
        Ok(vec![])
    }

    pub async fn record_product_browse(&self, user_id: Uuid, product_id: Uuid, duration: Option<u32>) -> Result<()> {
        // 获取商品信息
        let product_info = self.validate_product(&product_id).await?;
        
        let browse_record = ProductBrowseHistory {
            id: Uuid::new_v4(),
            user_id,
            product_id,
            product_name: product_info.name,
            product_image: product_info.image,
            category_id: product_info.category_id,
            browse_duration: duration,
            browsed_at: Utc::now(),
        };
        
        self.repository.create_browse_history(browse_record).await?;
        Ok(())
    }

    // ============ 私有辅助方法 ============

    async fn get_or_create_cart(&self, user_id: Uuid) -> Result<ShoppingCart> {
        if let Some(cart) = self.repository.get_cart_by_user(user_id).await? {
            Ok(cart)
        } else {
            let new_cart = ShoppingCart {
                id: Uuid::new_v4(),
                user_id,
                items: vec![],
                total_amount: 0.0,
                total_quantity: 0,
                currency: "USD".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            self.repository.create_cart(new_cart.clone()).await?;
            Ok(new_cart)
        }
    }

    fn recalculate_cart_totals(&self, cart: &mut ShoppingCart) {
        cart.total_amount = cart.items.iter().map(|item| item.total_price).sum();
        cart.total_quantity = cart.items.iter().map(|item| item.quantity).sum();
    }

    async fn validate_product(&self, product_id: &Uuid) -> Result<ProductInfo> {
        // 在实际实现中，这里会调用商品服务API
        Ok(ProductInfo {
            name: "示例商品".to_string(),
            price: 99.99,
            image: Some("https://example.com/image.jpg".to_string()),
            category_id: Uuid::new_v4(),
        })
    }

    async fn check_inventory(&self, product_id: &Uuid, quantity: u32) -> Result<()> {
        // 在实际实现中，这里会调用库存服务API
        Ok(())
    }

    async fn reserve_inventory(&self, product_id: &Uuid, quantity: u32) -> Result<()> {
        // 在实际实现中，这里会调用库存服务API
        Ok(())
    }

    async fn release_inventory(&self, product_id: &Uuid, quantity: u32) -> Result<()> {
        // 在实际实现中，这里会调用库存服务API
        Ok(())
    }

    async fn calculate_shipping_fee(&self, address: &ShippingAddress, cart: &ShoppingCart) -> Result<f64> {
        // 根据地址和购物车计算运费
        Ok(10.0) // 模拟运费
    }

    async fn calculate_tax_amount(&self, subtotal: f64, address: &ShippingAddress) -> Result<f64> {
        // 根据地址计算税额
        Ok(subtotal * 0.1) // 模拟10%税率
    }

    async fn calculate_discount(&self, coupon_code: &Option<String>, subtotal: f64) -> Result<f64> {
        // 计算优惠券折扣
        match coupon_code {
            Some(_) => Ok(subtotal * 0.05), // 模拟5%折扣
            None => Ok(0.0),
        }
    }

    async fn generate_order_number(&self) -> Result<String> {
        let timestamp = Utc::now().timestamp();
        let random_suffix: u32 = rand::random::<u32>() % 10000;
        Ok(format!("B2C{}{:04}", timestamp, random_suffix))
    }

    async fn get_cart_recommendations(&self, cart: &ShoppingCart) -> Result<Vec<ProductRecommendation>> {
        // 基于购物车内容生成推荐
        Ok(vec![])
    }
}

// 辅助结构体
#[derive(Debug)]
struct ProductInfo {
    name: String,
    price: f64,
    image: Option<String>,
    category_id: Uuid,
} 