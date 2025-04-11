use crate::datetime::datetime_serialization;
use chrono::{DateTime, Utc};
use mongodb::bson::Decimal128;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MembershipTarget {
    Customer,
    Reseller,
    Affiliate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MembershipTier {
    pub name: String,
    pub target: MembershipTarget,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl MembershipTier {
    pub fn new(name: impl Into<String>, target: MembershipTarget) -> Self {
        Self {
            name: name.into(),
            description: None,
            target,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl fmt::Display for MembershipTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub id: String,
    pub shop_id: String,
    pub customer_id: String,
    pub tier: MembershipTier,
    pub discount_percentage: Decimal128,
    pub is_active: bool,
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

impl Membership {
    pub fn is_valid_at(&self, date: DateTime<Utc>) -> bool {
        if !self.is_active {
            return false;
        }

        match (self.starts_at, self.expires_at) {
            (Some(start), Some(end)) => date >= start && date <= end,
            (Some(start), None) => date >= start,
            (None, Some(end)) => date <= end,
            (None, None) => true,
        }
    }

    pub fn get_discount_value(&self) -> Decimal128 {
        self.discount_percentage
    }
}

#[tarpc::service]
pub trait MembershipService {
    async fn create_membership(membership: Membership) -> Result<Membership, String>;
    async fn get_membership(id: String) -> Result<Membership, String>;
    async fn update_membership(membership: Membership) -> Result<Membership, String>;
    async fn delete_membership(id: String) -> Result<(), String>;
    async fn list_memberships(shop_id: String) -> Result<Vec<Membership>, String>;
    async fn apply_membership_discount(
        membership_id: String,
        cart_total: Decimal128,
    ) -> Result<Decimal128, String>;
    async fn validate_membership(membership_id: String) -> Result<bool, String>;
    async fn get_membership_by_customer_id(customer_id: String) -> Result<Membership, String>;
    async fn get_membership_by_customer_id_and_shop(
        customer_id: String,
        shop_id: String,
    ) -> Result<Membership, String>;
    async fn get_membership_by_tier(tier: MembershipTier) -> Result<Membership, String>;
    async fn get_membership_by_tier_and_shop(
        tier: MembershipTier,
        shop_id: String,
    ) -> Result<Membership, String>;
    async fn get_membership_by_tier_and_customer(
        tier: MembershipTier,
        customer_id: String,
    ) -> Result<Membership, String>;
}
