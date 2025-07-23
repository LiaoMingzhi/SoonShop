use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use tracing::{info, error, instrument};
use base64::{Engine, engine::general_purpose};

use crate::config::PayPalConfig;
use crate::error::PaymentError;

/// PayPal支付提供商
#[derive(Clone)]
pub struct PayPalProvider {
    client: Client,
    config: PayPalConfig,
    base_url: String,
    access_token: Option<String>,
}

/// PayPal订单创建请求
#[derive(Serialize, Debug)]
pub struct PayPalCreateOrderRequest {
    pub intent: String,
    pub purchase_units: Vec<PayPalPurchaseUnit>,
    pub payment_source: Option<PayPalPaymentSource>,
}

#[derive(Serialize, Debug)]
pub struct PayPalPurchaseUnit {
    pub amount: PayPalAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayPalAmount {
    pub currency_code: String,
    pub value: String,
}

#[derive(Serialize, Debug)]
pub struct PayPalPaymentSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PayPalPaymentSourceDetails>,
}

#[derive(Serialize, Debug)]
pub struct PayPalPaymentSourceDetails {
    pub experience_context: PayPalExperienceContext,
}

#[derive(Serialize, Debug)]
pub struct PayPalExperienceContext {
    pub return_url: String,
    pub cancel_url: String,
}

/// PayPal订单响应
#[derive(Deserialize, Debug)]
pub struct PayPalOrder {
    pub id: String,
    pub status: String,
    pub links: Option<Vec<PayPalLink>>,
    pub purchase_units: Option<Vec<PayPalPurchaseUnitResponse>>,
}

#[derive(Deserialize, Debug)]
pub struct PayPalLink {
    pub href: String,
    pub rel: String,
    pub method: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PayPalPurchaseUnitResponse {
    pub amount: PayPalAmount,
    pub payments: Option<PayPalPayments>,
}

#[derive(Deserialize, Debug)]
pub struct PayPalPayments {
    pub captures: Option<Vec<PayPalCapture>>,
}

#[derive(Deserialize, Debug)]
pub struct PayPalCapture {
    pub id: String,
    pub status: String,
    pub amount: PayPalAmount,
}

/// PayPal访问令牌响应
#[derive(Deserialize, Debug)]
struct PayPalTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl PayPalProvider {
    /// 创建新的PayPal支付提供商实例
    pub async fn new(config: &PayPalConfig) -> Result<Self> {
        let client = Client::new();
        
        let base_url = if config.sandbox {
            "https://api-m.sandbox.paypal.com".to_string()
        } else {
            "https://api-m.paypal.com".to_string()
        };

        let mut provider = PayPalProvider {
            client,
            config: config.clone(),
            base_url,
            access_token: None,
        };

        // 获取访问令牌
        provider.get_access_token().await?;

        Ok(provider)
    }

    /// 获取访问令牌
    #[instrument(skip(self))]
    async fn get_access_token(&mut self) -> Result<String> {
        let url = format!("{}/v1/oauth2/token", self.base_url);
        
        let auth = general_purpose::STANDARD.encode(format!("{}:{}", self.config.client_id, self.config.client_secret));
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Basic {}", auth))
            .header("Accept", "application/json")
            .header("Accept-Language", "en_US")
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to get access token: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("PayPal token error: {}", error_text);
            return Err(PaymentError::ProviderError(format!("PayPal token error: {}", error_text)).into());
        }

        let token_response: PayPalTokenResponse = response
            .json()
            .await
            .map_err(|e| PaymentError::InvalidResponse(format!("Failed to parse token response: {}", e)))?;

        self.access_token = Some(token_response.access_token.clone());
        
        info!("Successfully obtained PayPal access token");
        Ok(token_response.access_token)
    }

    /// 创建订单
    #[instrument(skip(self))]
    pub async fn create_order(
        &self,
        amount: f64,
        currency: &str,
        description: Option<&str>,
    ) -> Result<PayPalOrder> {
        let url = format!("{}/v2/checkout/orders", self.base_url);
        
        let request = PayPalCreateOrderRequest {
            intent: "CAPTURE".to_string(),
            purchase_units: vec![PayPalPurchaseUnit {
                amount: PayPalAmount {
                    currency_code: currency.to_uppercase(),
                    value: format!("{:.2}", amount),
                },
                description: description.map(|s| s.to_string()),
            }],
            payment_source: Some(PayPalPaymentSource {
                paypal: Some(PayPalPaymentSourceDetails {
                    experience_context: PayPalExperienceContext {
                        return_url: self.config.return_url.clone(),
                        cancel_url: self.config.cancel_url.clone(),
                    },
                }),
            }),
        };

        let access_token = self.access_token.as_ref()
            .ok_or(PaymentError::ConfigurationError("No access token available".to_string()))?;

        info!("Creating PayPal order for amount: {} {}", amount, currency);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("PayPal-Request-Id", uuid::Uuid::new_v4().to_string())
            .json(&request)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to create order: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("PayPal order creation error: {}", error_text);
            return Err(PaymentError::ProviderError(format!("PayPal order error: {}", error_text)).into());
        }

        let order: PayPalOrder = response
            .json()
            .await
            .map_err(|e| PaymentError::InvalidResponse(format!("Failed to parse order response: {}", e)))?;

        info!("Successfully created PayPal order: {}", order.id);
        Ok(order)
    }

    /// 捕获订单
    #[instrument(skip(self))]
    pub async fn capture_order(&self, order_id: &str) -> Result<PayPalOrder> {
        let url = format!("{}/v2/checkout/orders/{}/capture", self.base_url, order_id);
        
        let access_token = self.access_token.as_ref()
            .ok_or(PaymentError::ConfigurationError("No access token available".to_string()))?;

        info!("Capturing PayPal order: {}", order_id);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("PayPal-Request-Id", uuid::Uuid::new_v4().to_string())
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to capture order: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("PayPal capture error: {}", error_text);
            return Err(PaymentError::ProviderError(format!("PayPal capture error: {}", error_text)).into());
        }

        let order: PayPalOrder = response
            .json()
            .await
            .map_err(|e| PaymentError::InvalidResponse(format!("Failed to parse capture response: {}", e)))?;

        info!("Successfully captured PayPal order: {}", order.id);
        Ok(order)
    }

    /// 获取订单详情
    #[instrument(skip(self))]
    pub async fn get_order(&self, order_id: &str) -> Result<PayPalOrder> {
        let url = format!("{}/v2/checkout/orders/{}", self.base_url, order_id);
        
        let access_token = self.access_token.as_ref()
            .ok_or(PaymentError::ConfigurationError("No access token available".to_string()))?;

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to get order: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("PayPal get order error: {}", error_text);
            return Err(PaymentError::ProviderError(format!("PayPal get order error: {}", error_text)).into());
        }

        let order: PayPalOrder = response
            .json()
            .await
            .map_err(|e| PaymentError::InvalidResponse(format!("Failed to parse order response: {}", e)))?;

        Ok(order)
    }

    /// 健康检查
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> Result<()> {
        // 尝试获取新的访问令牌作为健康检查
        let url = format!("{}/v1/oauth2/token", self.base_url);
        
        let auth = general_purpose::STANDARD.encode(format!("{}:{}", self.config.client_id, self.config.client_secret));
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Basic {}", auth))
            .header("Accept", "application/json")
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Health check failed: {}", e)))?;

        if response.status().is_success() {
            info!("PayPal health check passed");
            Ok(())
        } else {
            error!("PayPal health check failed with status: {}", response.status());
            Err(PaymentError::ProviderError("PayPal health check failed".to_string()).into())
        }
    }
} 