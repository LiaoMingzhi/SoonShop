use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// OpenAPI文档构建器
pub struct OpenApiDocBuilder {
    spec: OpenApiSpec,
}

impl OpenApiDocBuilder {
    /// 创建新的API文档构建器
    pub fn new(title: &str, version: &str, description: &str) -> Self {
        Self {
            spec: OpenApiSpec {
                openapi: "3.0.3".to_string(),
                info: ApiInfo {
                    title: title.to_string(),
                    version: version.to_string(),
                    description: Some(description.to_string()),
                    contact: None,
                    license: None,
                },
                servers: vec![],
                paths: HashMap::new(),
                components: Some(Components {
                    schemas: HashMap::new(),
                    responses: HashMap::new(),
                    parameters: HashMap::new(),
                    examples: HashMap::new(),
                    request_bodies: HashMap::new(),
                    headers: HashMap::new(),
                    security_schemes: HashMap::new(),
                    links: HashMap::new(),
                    callbacks: HashMap::new(),
                }),
                security: None,
                tags: vec![],
                external_docs: None,
            },
        }
    }
    
    /// 添加服务器信息
    pub fn add_server(mut self, url: &str, description: &str) -> Self {
        self.spec.servers.push(ServerInfo {
            url: url.to_string(),
            description: Some(description.to_string()),
            variables: None,
        });
        self
    }
    
    /// 添加标签
    pub fn add_tag(mut self, name: &str, description: &str) -> Self {
        self.spec.tags.push(Tag {
            name: name.to_string(),
            description: Some(description.to_string()),
            external_docs: None,
        });
        self
    }
    
    /// 添加联系信息
    pub fn add_contact(mut self, name: &str, email: &str, url: Option<&str>) -> Self {
        self.spec.info.contact = Some(Contact {
            name: Some(name.to_string()),
            email: Some(email.to_string()),
            url: url.map(|s| s.to_string()),
        });
        self
    }
    
    /// 添加许可证信息
    pub fn add_license(mut self, name: &str, url: Option<&str>) -> Self {
        self.spec.info.license = Some(License {
            name: name.to_string(),
            url: url.map(|s| s.to_string()),
        });
        self
    }
    
    /// 添加路径
    pub fn add_path(mut self, path: &str, path_item: PathItem) -> Self {
        self.spec.paths.insert(path.to_string(), path_item);
        self
    }
    
    /// 添加组件schema
    pub fn add_schema(mut self, name: &str, schema: Schema) -> Self {
        if let Some(components) = &mut self.spec.components {
            components.schemas.insert(name.to_string(), schema);
        }
        self
    }
    
    /// 添加安全方案
    pub fn add_security_scheme(mut self, name: &str, scheme: SecurityScheme) -> Self {
        if let Some(components) = &mut self.spec.components {
            components.security_schemes.insert(name.to_string(), scheme);
        }
        self
    }
    
    /// 构建OpenAPI规范
    pub fn build(self) -> OpenApiSpec {
        self.spec
    }
    
    /// 生成JSON格式的文档
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.spec)
    }
    
    /// 生成YAML格式的文档
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&self.spec)
    }
}

/// OpenAPI规范结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: ApiInfo,
    pub servers: Vec<ServerInfo>,
    pub paths: HashMap<String, PathItem>,
    pub components: Option<Components>,
    pub security: Option<Vec<SecurityRequirement>>,
    pub tags: Vec<Tag>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInfo {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub url: String,
    pub description: Option<String>,
    pub variables: Option<HashMap<String, ServerVariable>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariable {
    pub default: String,
    pub description: Option<String>,
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
    pub trace: Option<Operation>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub parameters: Option<Vec<Parameter>>,
    pub servers: Option<Vec<ServerInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub parameters: Option<Vec<Parameter>>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBody>,
    pub responses: HashMap<String, Response>,
    pub callbacks: Option<HashMap<String, Callback>>,
    pub deprecated: Option<bool>,
    pub security: Option<Vec<SecurityRequirement>>,
    pub servers: Option<Vec<ServerInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String, // "query", "header", "path", "cookie"
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    #[serde(rename = "allowEmptyValue")]
    pub allow_empty_value: Option<bool>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    #[serde(rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
    pub schema: Option<Schema>,
    pub example: Option<Value>,
    pub examples: Option<HashMap<String, Example>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: HashMap<String, MediaType>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: Option<Schema>,
    pub example: Option<Value>,
    pub examples: Option<HashMap<String, Example>>,
    pub encoding: Option<HashMap<String, Encoding>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encoding {
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub headers: Option<HashMap<String, Header>>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    #[serde(rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    pub headers: Option<HashMap<String, Header>>,
    pub content: Option<HashMap<String, MediaType>>,
    pub links: Option<HashMap<String, Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    #[serde(rename = "allowEmptyValue")]
    pub allow_empty_value: Option<bool>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    #[serde(rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
    pub schema: Option<Schema>,
    pub example: Option<Value>,
    pub examples: Option<HashMap<String, Example>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "operationRef")]
    pub operation_ref: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub parameters: Option<HashMap<String, Value>>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<Value>,
    pub description: Option<String>,
    pub server: Option<ServerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub value: Option<Value>,
    #[serde(rename = "externalValue")]
    pub external_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub format: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub example: Option<Value>,
    pub examples: Option<Vec<Value>>,
    pub properties: Option<HashMap<String, Schema>>,
    pub required: Option<Vec<String>>,
    pub items: Option<Box<Schema>>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<Box<Schema>>,
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<Value>>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<usize>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    #[serde(rename = "minItems")]
    pub min_items: Option<usize>,
    #[serde(rename = "maxItems")]
    pub max_items: Option<usize>,
    #[serde(rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<Schema>>,
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<Schema>>,
    #[serde(rename = "anyOf")]
    pub any_of: Option<Vec<Schema>>,
    pub not: Option<Box<Schema>>,
    pub nullable: Option<bool>,
    pub discriminator: Option<Discriminator>,
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "writeOnly")]
    pub write_only: Option<bool>,
    pub deprecated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discriminator {
    #[serde(rename = "propertyName")]
    pub property_name: String,
    pub mapping: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    pub schemas: HashMap<String, Schema>,
    pub responses: HashMap<String, Response>,
    pub parameters: HashMap<String, Parameter>,
    pub examples: HashMap<String, Example>,
    #[serde(rename = "requestBodies")]
    pub request_bodies: HashMap<String, RequestBody>,
    pub headers: HashMap<String, Header>,
    #[serde(rename = "securitySchemes")]
    pub security_schemes: HashMap<String, SecurityScheme>,
    pub links: HashMap<String, Link>,
    pub callbacks: HashMap<String, Callback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScheme {
    #[serde(rename = "type")]
    pub scheme_type: String, // "apiKey", "http", "oauth2", "openIdConnect"
    pub description: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "in")]
    pub location: Option<String>, // "query", "header", "cookie"
    pub scheme: Option<String>,
    #[serde(rename = "bearerFormat")]
    pub bearer_format: Option<String>,
    pub flows: Option<OAuthFlows>,
    #[serde(rename = "openIdConnectUrl")]
    pub open_id_connect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlows {
    pub implicit: Option<OAuthFlow>,
    pub password: Option<OAuthFlow>,
    #[serde(rename = "clientCredentials")]
    pub client_credentials: Option<OAuthFlow>,
    #[serde(rename = "authorizationCode")]
    pub authorization_code: Option<OAuthFlow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlow {
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: Option<String>,
    #[serde(rename = "tokenUrl")]
    pub token_url: Option<String>,
    #[serde(rename = "refreshUrl")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentation {
    pub description: Option<String>,
    pub url: String,
}

pub type SecurityRequirement = HashMap<String, Vec<String>>;
pub type Callback = HashMap<String, PathItem>;

/// API文档生成器
pub struct ApiDocGenerator {
    services: Vec<ServiceDoc>,
}

impl ApiDocGenerator {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }
    
    /// 添加服务文档
    pub fn add_service(mut self, service: ServiceDoc) -> Self {
        self.services.push(service);
        self
    }
    
    /// 生成聚合文档
    pub fn generate_aggregated_doc(&self) -> OpenApiSpec {
        let mut builder = OpenApiDocBuilder::new(
            "SoonShop API",
            "1.0.0",
            "SoonShop微服务系统API文档"
        );
        
        // 添加服务器信息
        builder = builder.add_server("http://localhost:8000", "开发环境");
        builder = builder.add_server("https://api.soonshop.com", "生产环境");
        
        // 添加通用标签
        builder = builder.add_tag("用户管理", "用户相关接口");
        builder = builder.add_tag("商品管理", "商品相关接口");
        builder = builder.add_tag("订单管理", "订单相关接口");
        builder = builder.add_tag("支付管理", "支付相关接口");
        builder = builder.add_tag("企业管理", "企业相关接口");
        
        // 添加联系信息
        builder = builder.add_contact("SoonShop开发团队", "dev@soonshop.com", Some("https://soonshop.com"));
        
        // 添加许可证
        builder = builder.add_license("MIT", Some("https://opensource.org/licenses/MIT"));
        
        // 添加安全方案
        builder = builder.add_security_scheme("BearerAuth", SecurityScheme {
            scheme_type: "http".to_string(),
            description: Some("JWT Bearer Token认证".to_string()),
            name: None,
            location: None,
            scheme: Some("bearer".to_string()),
            bearer_format: Some("JWT".to_string()),
            flows: None,
            open_id_connect_url: None,
        });
        
        // 合并各服务的路径和组件
        for service in &self.services {
            for (path, path_item) in &service.paths {
                builder = builder.add_path(path, path_item.clone());
            }
            for (name, schema) in &service.schemas {
                builder = builder.add_schema(name, schema.clone());
            }
        }
        
        builder.build()
    }
    
    /// 生成Swagger UI HTML
    pub fn generate_swagger_ui(&self, spec_url: &str) -> String {
        format!(
            r#"
<!DOCTYPE html>
<html lang="zh">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SoonShop API 文档</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui.css" />
    <style>
        html {{
            box-sizing: border-box;
            overflow: -moz-scrollbars-vertical;
            overflow-y: scroll;
        }}
        *, *:before, *:after {{
            box-sizing: inherit;
        }}
        body {{
            margin:0;
            background: #fafafa;
        }}
        .swagger-ui .topbar {{
            display: none;
        }}
        .swagger-ui .info .title {{
            color: #2c3e50;
        }}
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {{
            SwaggerUIBundle({{
                url: '{spec_url}',
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout",
                tryItOutEnabled: true,
                supportedSubmitMethods: ['get', 'post', 'put', 'delete', 'patch'],
                defaultModelsExpandDepth: 2,
                defaultModelExpandDepth: 2,
                docExpansion: 'none',
                filter: true,
                showExtensions: true,
                showCommonExtensions: true,
                validatorUrl: null
            }});
        }};
    </script>
</body>
</html>
            "#,
            spec_url = spec_url
        )
    }
}

/// 服务文档
#[derive(Debug, Clone)]
pub struct ServiceDoc {
    pub name: String,
    pub version: String,
    pub description: String,
    pub paths: HashMap<String, PathItem>,
    pub schemas: HashMap<String, Schema>,
}

/// API文档中间件
pub struct ApiDocMiddleware {
    spec: OpenApiSpec,
}

impl ApiDocMiddleware {
    pub fn new(spec: OpenApiSpec) -> Self {
        Self { spec }
    }
    
    /// 配置路由
    pub fn configure_routes(&self, cfg: &mut web::ServiceConfig) {
        let spec_json = serde_json::to_string(&self.spec).unwrap();
        let spec_yaml = serde_yaml::to_string(&self.spec).unwrap();
        
        cfg.service(
            web::scope("/docs")
                .route("/openapi.json", web::get().to(move || {
                    let spec = spec_json.clone();
                    async move {
                        HttpResponse::Ok()
                            .content_type("application/json")
                            .body(spec)
                    }
                }))
                .route("/openapi.yaml", web::get().to(move || {
                    let spec = spec_yaml.clone();
                    async move {
                        HttpResponse::Ok()
                            .content_type("application/yaml")
                            .body(spec)
                    }
                }))
                .route("/", web::get().to(swagger_ui_handler))
                .route("/swagger-ui", web::get().to(swagger_ui_handler))
        );
    }
}

/// Swagger UI处理器
async fn swagger_ui_handler() -> ActixResult<HttpResponse> {
    let generator = ApiDocGenerator::new();
    let html = generator.generate_swagger_ui("/docs/openapi.json");
    
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

/// 创建常用的Schema
pub fn create_common_schemas() -> HashMap<String, Schema> {
    let mut schemas = HashMap::new();
    
    // 错误响应Schema
    schemas.insert("ErrorResponse".to_string(), Schema {
        schema_type: Some("object".to_string()),
        title: Some("错误响应".to_string()),
        description: Some("API错误响应格式".to_string()),
        properties: Some({
            let mut props = HashMap::new();
            props.insert("error".to_string(), Schema {
                schema_type: Some("string".to_string()),
                description: Some("错误代码".to_string()),
                example: Some(json!("validation_failed")),
                ..Default::default()
            });
            props.insert("message".to_string(), Schema {
                schema_type: Some("string".to_string()),
                description: Some("错误消息".to_string()),
                example: Some(json!("请求参数验证失败")),
                ..Default::default()
            });
            props.insert("details".to_string(), Schema {
                schema_type: Some("object".to_string()),
                description: Some("错误详情".to_string()),
                ..Default::default()
            });
            props
        }),
        required: Some(vec!["error".to_string(), "message".to_string()]),
        ..Default::default()
    });
    
    // 成功响应Schema
    schemas.insert("SuccessResponse".to_string(), Schema {
        schema_type: Some("object".to_string()),
        title: Some("成功响应".to_string()),
        description: Some("API成功响应格式".to_string()),
        properties: Some({
            let mut props = HashMap::new();
            props.insert("success".to_string(), Schema {
                schema_type: Some("boolean".to_string()),
                description: Some("是否成功".to_string()),
                example: Some(json!(true)),
                ..Default::default()
            });
            props.insert("data".to_string(), Schema {
                schema_type: Some("object".to_string()),
                description: Some("响应数据".to_string()),
                ..Default::default()
            });
            props.insert("message".to_string(), Schema {
                schema_type: Some("string".to_string()),
                description: Some("成功消息".to_string()),
                example: Some(json!("操作成功")),
                ..Default::default()
            });
            props
        }),
        required: Some(vec!["success".to_string()]),
        ..Default::default()
    });
    
    // 分页响应Schema
    schemas.insert("PaginationResponse".to_string(), Schema {
        schema_type: Some("object".to_string()),
        title: Some("分页响应".to_string()),
        description: Some("分页查询响应格式".to_string()),
        properties: Some({
            let mut props = HashMap::new();
            props.insert("page".to_string(), Schema {
                schema_type: Some("integer".to_string()),
                description: Some("当前页码".to_string()),
                example: Some(json!(1)),
                ..Default::default()
            });
            props.insert("limit".to_string(), Schema {
                schema_type: Some("integer".to_string()),
                description: Some("每页条数".to_string()),
                example: Some(json!(10)),
                ..Default::default()
            });
            props.insert("total".to_string(), Schema {
                schema_type: Some("integer".to_string()),
                description: Some("总条数".to_string()),
                example: Some(json!(100)),
                ..Default::default()
            });
            props.insert("has_more".to_string(), Schema {
                schema_type: Some("boolean".to_string()),
                description: Some("是否有更多数据".to_string()),
                example: Some(json!(true)),
                ..Default::default()
            });
            props
        }),
        required: Some(vec!["page".to_string(), "limit".to_string(), "total".to_string()]),
        ..Default::default()
    });
    
    schemas
}

impl Default for Schema {
    fn default() -> Self {
        Self {
            schema_type: None,
            format: None,
            title: None,
            description: None,
            default: None,
            example: None,
            examples: None,
            properties: None,
            required: None,
            items: None,
            additional_properties: None,
            enum_values: None,
            minimum: None,
            maximum: None,
            min_length: None,
            max_length: None,
            pattern: None,
            min_items: None,
            max_items: None,
            unique_items: None,
            reference: None,
            all_of: None,
            one_of: None,
            any_of: None,
            not: None,
            nullable: None,
            discriminator: None,
            read_only: None,
            write_only: None,
            deprecated: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_openapi_doc_builder() {
        let builder = OpenApiDocBuilder::new("Test API", "1.0.0", "Test API Documentation");
        let spec = builder.build();
        
        assert_eq!(spec.info.title, "Test API");
        assert_eq!(spec.info.version, "1.0.0");
        assert_eq!(spec.info.description, Some("Test API Documentation".to_string()));
    }
    
    #[test]
    fn test_common_schemas() {
        let schemas = create_common_schemas();
        assert!(schemas.contains_key("ErrorResponse"));
        assert!(schemas.contains_key("SuccessResponse"));
        assert!(schemas.contains_key("PaginationResponse"));
    }
    
    #[test]
    fn test_swagger_ui_generation() {
        let generator = ApiDocGenerator::new();
        let html = generator.generate_swagger_ui("/docs/openapi.json");
        assert!(html.contains("swagger-ui"));
        assert!(html.contains("/docs/openapi.json"));
    }
} 