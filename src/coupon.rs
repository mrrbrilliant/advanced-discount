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
