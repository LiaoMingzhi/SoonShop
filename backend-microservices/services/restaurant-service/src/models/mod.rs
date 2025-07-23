use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveTime};
use uuid::Uuid;
use validator::Validate;

// ============ 餐厅相关模型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restaurant {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub cuisine_type: CuisineType,
    pub address: RestaurantAddress,
    pub contact: RestaurantContact,
    pub operating_hours: Vec<OperatingHours>,
    pub capacity: u32,
    pub rating: f64,
    pub price_range: PriceRange,
    pub features: Vec<RestaurantFeature>,
    pub images: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantContact {
    pub phone: String,
    pub email: String,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingHours {
    pub day_of_week: u32, // 0=Sunday, 1=Monday, ..., 6=Saturday
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
    pub is_closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CuisineType {
    Chinese,
    Italian,
    Japanese,
    American,
    French,
    Indian,
    Thai,
    Mexican,
    Korean,
    Mediterranean,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriceRange {
    Budget,      // $
    Moderate,    // $$
    Expensive,   // $$$
    VeryExpensive, // $$$$
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestaurantFeature {
    Parking,
    WifiAccess,
    Wheelchair,
    OutdoorSeating,
    Takeout,
    Delivery,
    Reservations,
    CreditCard,
    Alcohol,
    LiveMusic,
    PrivateDining,
    Brunch,
    Breakfast,
    Lunch,
    Dinner,
    LateNight,
}

// ============ 预订相关模型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub restaurant_id: Uuid,
    pub reservation_number: String,
    pub guest_count: u32,
    pub reservation_date: DateTime<Utc>,
    pub duration_minutes: u32,
    pub status: ReservationStatus,
    pub table_id: Option<Uuid>,
    pub special_requests: Option<String>,
    pub contact_info: GuestContactInfo,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestContactInfo {
    pub name: String,
    pub phone: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReservationStatus {
    Pending,     // 待确认
    Confirmed,   // 已确认
    Seated,      // 已就座
    Completed,   // 已完成
    Cancelled,   // 已取消
    NoShow,      // 未到店
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateReservationRequest {
    pub user_id: Uuid,
    pub restaurant_id: Uuid,
    #[validate(range(min = 1, max = 20))]
    pub guest_count: u32,
    pub reservation_date: DateTime<Utc>,
    #[validate(range(min = 30, max = 480))]
    pub duration_minutes: u32,
    #[validate(length(max = 500))]
    pub special_requests: Option<String>,
    pub contact_info: GuestContactInfo,
}

// ============ 餐桌相关模型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub table_number: String,
    pub capacity: u32,
    pub location: TableLocation,
    pub features: Vec<TableFeature>,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableLocation {
    Indoor,
    Outdoor,
    Patio,
    PrivateRoom,
    Bar,
    Window,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableFeature {
    Smoking,
    NonSmoking,
    Wheelchair,
    HighTop,
    Booth,
    Round,
    Square,
    Rectangular,
}

// ============ 菜单相关模型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub menu_type: MenuType,
    pub categories: Vec<MenuCategory>,
    pub is_active: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MenuType {
    Breakfast,
    Lunch,
    Dinner,
    Brunch,
    Dessert,
    Beverage,
    Wine,
    Specials,
    Seasonal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub items: Vec<MenuItem>,
    pub sort_order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub currency: String,
    pub allergens: Vec<String>,
    pub dietary_info: Vec<DietaryInfo>,
    pub spice_level: Option<SpiceLevel>,
    pub preparation_time: Option<u32>,
    pub image_url: Option<String>,
    pub is_available: bool,
    pub popularity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DietaryInfo {
    Vegetarian,
    Vegan,
    GlutenFree,
    DairyFree,
    NutFree,
    Halal,
    Kosher,
    Organic,
    LowCarb,
    Keto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpiceLevel {
    Mild,
    Medium,
    Hot,
    VeryHot,
}

// ============ 评价相关模型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: Uuid,
    pub user_id: Uuid,
    pub restaurant_id: Uuid,
    pub reservation_id: Option<Uuid>,
    pub rating: f64,
    pub title: String,
    pub content: String,
    pub photos: Vec<String>,
    pub aspects: ReviewAspects,
    pub is_verified: bool,
    pub helpful_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewAspects {
    pub food_quality: Option<f64>,
    pub service_quality: Option<f64>,
    pub atmosphere: Option<f64>,
    pub value_for_money: Option<f64>,
    pub cleanliness: Option<f64>,
}

// ============ 查询和响应模型 ============

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RestaurantSearchRequest {
    pub location: Option<String>,
    pub cuisine_type: Option<CuisineType>,
    pub price_range: Option<PriceRange>,
    pub rating_min: Option<f64>,
    pub features: Option<Vec<RestaurantFeature>>,
    pub date: Option<DateTime<Utc>>,
    pub guest_count: Option<u32>,
    pub sort_by: Option<RestaurantSortOption>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestaurantSortOption {
    Rating,
    Distance,
    Price,
    Popularity,
    Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantSearchResponse {
    pub restaurants: Vec<RestaurantSearchResult>,
    pub total_count: u64,
    pub page: u32,
    pub limit: u32,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantSearchResult {
    pub id: Uuid,
    pub name: String,
    pub cuisine_type: CuisineType,
    pub rating: f64,
    pub price_range: PriceRange,
    pub distance: Option<f64>,
    pub address: String,
    pub features: Vec<RestaurantFeature>,
    pub image_url: Option<String>,
    pub availability: Option<AvailabilityInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityInfo {
    pub is_available: bool,
    pub next_available_time: Option<DateTime<Utc>>,
    pub available_tables: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationResponse {
    pub reservation: Reservation,
    pub restaurant: RestaurantSearchResult,
    pub confirmation_instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub time: DateTime<Utc>,
    pub available_tables: u32,
    pub is_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityResponse {
    pub date: DateTime<Utc>,
    pub time_slots: Vec<TimeSlot>,
}

// ============ 实现转换 ============

impl From<Reservation> for ReservationResponse {
    fn from(reservation: Reservation) -> Self {
        ReservationResponse {
            reservation,
            restaurant: RestaurantSearchResult {
                id: Uuid::new_v4(),
                name: "示例餐厅".to_string(),
                cuisine_type: CuisineType::Chinese,
                rating: 4.5,
                price_range: PriceRange::Moderate,
                distance: None,
                address: "示例地址".to_string(),
                features: vec![],
                image_url: None,
                availability: None,
            },
            confirmation_instructions: "请于预约时间前15分钟到达餐厅".to_string(),
        }
    }
} 