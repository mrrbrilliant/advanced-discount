use std::collections::HashMap;

use crate::{
    coupon::Coupon,
    datetime::datetime_serialization,
    membership::{Membership, MembershipTier},
};
use bson::Decimal128;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    DoesNotContain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountRule {
    pub id: String,
    pub shop_id: String,
    pub name: String,
    pub conditions: Vec<Condition>,
    pub actions: Vec<DiscountAction>,
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "datetime_serialization")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "datetime_serialization")]
    pub end_date: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub usage_count: i32,
    pub max_usage: Option<i32>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    CartTotal {
        operator: Operator,
        value: Decimal128,
    },
    ProductCategory {
        category_ids: Vec<String>,
    },
    CustomerGroup {
        group_ids: Vec<String>,
    },
    PurchaseHistory {
        min_orders: i32,
        timeframe_days: i32,
    },
    TimeOfDay {
        start_hour: i32,
        end_hour: i32,
    },
    DayOfWeek {
        days: Vec<u8>,
    },
    ProductQuantity {
        product_id: String,
        operator: Operator,
        quantity: i32,
    },
    FirstPurchase,
    Coupon {
        code: String,
    },
    MinimumSpend {
        amount: Decimal128,
    },
    MembershipTier {
        tiers: Vec<MembershipTier>,
    },
    MembershipActive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscountAction {
    PercentageOff {
        percent: Decimal128,
    },
    FixedAmountOff {
        amount: Decimal128,
    },
    FreeShipping,
    BuyXGetY {
        buy_product_id: String,
        buy_quantity: i32,
        get_product_id: String,
        get_quantity: i32,
    },
}

#[derive(Debug, Clone)]
pub struct EvaluationContext {
    pub shop_id: String,
    pub cart_total: Decimal128,
    pub product_quantities: HashMap<String, i32>,
    pub product_categories: HashMap<String, Vec<String>>,
    pub customer_groups: Vec<String>,
    pub order_count: i32,
    pub now: DateTime<Utc>,
    pub is_first_purchase: bool,
    pub current_day: u8,
    pub current_hour: i32,
    pub applied_coupon: Option<Coupon>,
    pub customer_membership: Option<Membership>,
}

impl DiscountRule {
    pub fn evaluate(&self, ctx: &EvaluationContext) -> bool {
        if !self.is_active {
            return false;
        }
        if let Some(start) = self.start_date {
            if ctx.now < start {
                return false;
            }
        }
        if let Some(end) = self.end_date {
            if ctx.now > end {
                return false;
            }
        }
        self.conditions.iter().all(|cond| cond.evaluate(ctx))
    }
}

impl Condition {
    pub fn evaluate(&self, ctx: &EvaluationContext) -> bool {
        match self {
            Condition::CartTotal { operator, value } => {
                compare_decimal(ctx.cart_total, *value, operator)
            }
            Condition::ProductQuantity {
                product_id,
                operator,
                quantity,
            } => {
                let qty = *ctx.product_quantities.get(product_id).unwrap_or(&0);
                compare_i32(qty, *quantity, operator)
            }
            Condition::CustomerGroup { group_ids } => {
                group_ids.iter().any(|g| ctx.customer_groups.contains(g))
            }
            Condition::PurchaseHistory {
                min_orders,
                timeframe_days: _,
            } => ctx.order_count >= *min_orders,
            Condition::TimeOfDay {
                start_hour,
                end_hour,
            } => ctx.current_hour >= *start_hour && ctx.current_hour <= *end_hour,
            Condition::DayOfWeek { days } => days.contains(&ctx.current_day),
            Condition::FirstPurchase => ctx.is_first_purchase,
            Condition::ProductCategory { category_ids } => {
                ctx.product_quantities
                    .iter()
                    .any(|(product_id, &quantity)| {
                        if quantity > 0 {
                            if let Some(product_categories) = ctx.product_categories.get(product_id)
                            {
                                category_ids
                                    .iter()
                                    .any(|cat_id| product_categories.contains(cat_id))
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
            }
            Condition::Coupon { code } => {
                if let Some(coupon) = &ctx.applied_coupon {
                    coupon.code == *code
                        && coupon.expires_at.map_or(true, |exp| ctx.now <= exp)
                        && coupon.max_uses.map_or(true, |max| coupon.used_count < max)
                } else {
                    false
                }
            }
            Condition::MinimumSpend { amount } => {
                compare_decimal(ctx.cart_total, *amount, &Operator::GreaterThanOrEqual)
            }
            Condition::MembershipTier { tiers } => {
                if let Some(membership) = &ctx.customer_membership {
                    if !membership.is_valid_at(ctx.now) {
                        return false;
                    }
                    tiers.iter().any(|tier| tier.name == membership.tier.name)
                } else {
                    false
                }
            }
            Condition::MembershipActive => {
                if let Some(membership) = &ctx.customer_membership {
                    membership.is_valid_at(ctx.now)
                } else {
                    false
                }
            }
        }
    }
}

pub fn compare_decimal(a: Decimal128, b: Decimal128, op: &Operator) -> bool {
    let a_f64 = a.to_string().parse::<f64>().unwrap_or(0.0);
    let b_f64 = b.to_string().parse::<f64>().unwrap_or(0.0);
    match op {
        Operator::Equal => a_f64 == b_f64,
        Operator::NotEqual => a_f64 != b_f64,
        Operator::GreaterThan => a_f64 > b_f64,
        Operator::LessThan => a_f64 < b_f64,
        Operator::GreaterThanOrEqual => a_f64 >= b_f64,
        Operator::LessThanOrEqual => a_f64 <= b_f64,
        _ => false,
    }
}

pub fn compare_i32(a: i32, b: i32, op: &Operator) -> bool {
    match op {
        Operator::Equal => a == b,
        Operator::NotEqual => a != b,
        Operator::GreaterThan => a > b,
        Operator::LessThan => a < b,
        Operator::GreaterThanOrEqual => a >= b,
        Operator::LessThanOrEqual => a <= b,
        _ => false,
    }
}
