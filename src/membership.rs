use crate::datetime::datetime_serialization;
use chrono::{DateTime, Utc};
use mongodb::bson::Decimal128;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MembershipTier {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl MembershipTier {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    // Convenience methods for common tiers
    pub fn basic() -> Self {
        Self::new("Basic")
    }

    pub fn silver() -> Self {
        Self::new("Silver")
    }

    pub fn gold() -> Self {
        Self::new("Gold")
    }

    pub fn platinum() -> Self {
        Self::new("Platinum")
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
