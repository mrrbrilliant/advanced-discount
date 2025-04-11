use crate::datetime::datetime_serialization;
use chrono::{DateTime, Utc};
use mongodb::bson::{Decimal128, doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub id: String,
    pub shop_id: String,
    pub code: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub discount_type: CouponDiscountType,
    pub discount_value: Decimal128,
    pub is_single_use: bool,
    pub used_count: i32,
    pub max_uses: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "datetime_serialization")]
    pub starts_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "datetime_serialization")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CouponDiscountType {
    Percentage,
    FixedAmount,
    FreeShipping,
}
#[tarpc::service]
pub trait CouponService {
    async fn create_coupon(coupon: Coupon) -> Result<Coupon, String>;
    async fn get_coupon(id: String) -> Result<Coupon, String>;
    async fn update_coupon(coupon: Coupon) -> Result<Coupon, String>;
    async fn delete_coupon(id: String) -> Result<(), String>;
    async fn list_coupons(shop_id: String) -> Result<Vec<Coupon>, String>;
    async fn apply_coupon(
        coupon_code: String,
        cart_total: Decimal128,
    ) -> Result<Decimal128, String>;
    async fn validate_coupon(coupon_code: String) -> Result<bool, String>;
    async fn get_coupon_by_code(coupon_code: String) -> Result<Coupon, String>;
    async fn get_coupon_by_code_and_shop(
        coupon_code: String,
        shop_id: String,
    ) -> Result<Coupon, String>;
}
