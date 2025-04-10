pub mod coupon;
pub mod datetime;
pub mod discount;
pub mod membership;

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr as _;

//     use super::*;
//     use chrono::TimeZone;
//     use uuid::Uuid;

//     fn create_test_context() -> EvaluationContext {
//         let mut product_quantities = HashMap::new();
//         product_quantities.insert("product1".to_string(), 2);

//         let mut product_categories = HashMap::new();
//         product_categories.insert("product1".to_string(), vec!["category1".to_string()]);

//         EvaluationContext {
//             shop_id: "shop1".to_string(),
//             cart_total: Decimal128::from_str("100.00").unwrap(),
//             product_quantities,
//             customer_groups: vec!["vip".to_string()],
//             order_count: 5,
//             now: Utc.with_ymd_and_hms(2025, 4, 10, 14, 0, 0).unwrap(),
//             is_first_purchase: false,
//             current_day: 3, // Wednesday
//             current_hour: 14,
//             applied_coupon: None,
//             product_categories,
//         }
//     }

//     fn create_test_rule() -> DiscountRule {
//         DiscountRule {
//             id: Uuid::new_v4().to_string(),
//             shop_id: "shop1".to_string(),
//             name: "Test Rule".to_string(),
//             conditions: vec![],
//             actions: vec![],
//             priority: 1,
//             start_date: None,
//             end_date: None,
//             is_active: true,
//             usage_count: 0,
//             max_usage: None,
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//         }
//     }

//     fn create_test_coupon(code: &str) -> Coupon {
//         Coupon {
//             id: Uuid::new_v4().to_string(),
//             shop_id: "shop1".to_string(),
//             code: code.to_string(),
//             description: None,
//             is_active: true,
//             discount_type: CouponDiscountType::Percentage,
//             discount_value: Decimal128::from_str("20.00").unwrap(),
//             is_single_use: false,
//             used_count: 0,
//             max_uses: Some(100),
//             starts_at: None,
//             expires_at: Some(Utc.with_ymd_and_hms(2025, 12, 31, 23, 59, 59).unwrap()),
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//         }
//     }

//     #[test]
//     fn test_cart_total_condition() {
//         let mut rule = create_test_rule();
//         rule.conditions = vec![Condition::CartTotal {
//             operator: Operator::GreaterThan,
//             value: Decimal128::from_str("50.00").unwrap(),
//         }];

//         let ctx = create_test_context();
//         assert!(rule.evaluate(&ctx));

//         rule.conditions = vec![Condition::CartTotal {
//             operator: Operator::GreaterThan,
//             value: Decimal128::from_str("150.00").unwrap(),
//         }];
//         assert!(!rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_customer_group_condition() {
//         let mut rule = create_test_rule();
//         rule.conditions = vec![Condition::CustomerGroup {
//             group_ids: vec!["vip".to_string()],
//         }];

//         let ctx = create_test_context();
//         assert!(rule.evaluate(&ctx));

//         rule.conditions = vec![Condition::CustomerGroup {
//             group_ids: vec!["premium".to_string()],
//         }];
//         assert!(!rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_product_quantity_condition() {
//         let mut rule = create_test_rule();
//         rule.conditions = vec![Condition::ProductQuantity {
//             product_id: "product1".to_string(),
//             operator: Operator::Equal,
//             quantity: 2,
//         }];

//         let ctx = create_test_context();
//         assert!(rule.evaluate(&ctx));

//         rule.conditions = vec![Condition::ProductQuantity {
//             product_id: "product1".to_string(),
//             operator: Operator::GreaterThan,
//             quantity: 2,
//         }];
//         assert!(!rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_time_of_day_condition() {
//         let mut rule = create_test_rule();
//         rule.conditions = vec![Condition::TimeOfDay {
//             start_hour: 9,
//             end_hour: 17,
//         }];

//         let ctx = create_test_context();
//         assert!(rule.evaluate(&ctx));

//         rule.conditions = vec![Condition::TimeOfDay {
//             start_hour: 18,
//             end_hour: 23,
//         }];
//         assert!(!rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_day_of_week_condition() {
//         let mut rule = create_test_rule();
//         rule.conditions = vec![Condition::DayOfWeek {
//             days: vec![3], // Wednesday
//         }];

//         let ctx = create_test_context();
//         assert!(rule.evaluate(&ctx));

//         rule.conditions = vec![Condition::DayOfWeek {
//             days: vec![1, 2], // Monday, Tuesday
//         }];
//         assert!(!rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_purchase_history_condition() {
//         let mut rule = create_test_rule();
//         rule.conditions = vec![Condition::PurchaseHistory {
//             min_orders: 3,
//             timeframe_days: 30,
//         }];

//         let ctx = create_test_context();
//         assert!(rule.evaluate(&ctx));

//         rule.conditions = vec![Condition::PurchaseHistory {
//             min_orders: 10,
//             timeframe_days: 30,
//         }];
//         assert!(!rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_first_purchase_condition() {
//         let mut rule = create_test_rule();
//         rule.conditions = vec![Condition::FirstPurchase];

//         let mut ctx = create_test_context();
//         assert!(!rule.evaluate(&ctx));

//         ctx.is_first_purchase = true;
//         assert!(rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_rule_date_range() {
//         let mut rule = create_test_rule();
//         rule.start_date = Some(Utc.with_ymd_and_hms(2025, 4, 1, 0, 0, 0).unwrap());
//         rule.end_date = Some(Utc.with_ymd_and_hms(2025, 4, 30, 0, 0, 0).unwrap());

//         let ctx = create_test_context();
//         assert!(rule.evaluate(&ctx));

//         rule.end_date = Some(Utc.with_ymd_and_hms(2025, 4, 1, 0, 0, 0).unwrap());
//         assert!(!rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_rule_active_status() {
//         let mut rule = create_test_rule();
//         assert!(rule.evaluate(&create_test_context()));

//         rule.is_active = false;
//         assert!(!rule.evaluate(&create_test_context()));
//     }

//     #[test]
//     fn test_multiple_conditions() {
//         let mut rule = create_test_rule();
//         rule.conditions = vec![
//             Condition::CartTotal {
//                 operator: Operator::GreaterThan,
//                 value: Decimal128::from_str("50.00").unwrap(),
//             },
//             Condition::CustomerGroup {
//                 group_ids: vec!["vip".to_string()],
//             },
//             Condition::DayOfWeek { days: vec![3] },
//         ];

//         let ctx = create_test_context();
//         assert!(rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_coupon_condition() {
//         let mut rule = create_test_rule();
//         let coupon_code = "SAVE20";
//         rule.conditions = vec![Condition::Coupon {
//             code: coupon_code.to_string(),
//         }];

//         let mut ctx = create_test_context();
//         ctx.applied_coupon = Some(create_test_coupon(coupon_code));
//         assert!(rule.evaluate(&ctx));

//         // Test invalid coupon code
//         ctx.applied_coupon = Some(create_test_coupon("DIFFERENT"));
//         assert!(!rule.evaluate(&ctx));

//         // Test expired coupon
//         let mut expired_coupon = create_test_coupon(coupon_code);
//         expired_coupon.expires_at = Some(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
//         ctx.applied_coupon = Some(expired_coupon);
//         assert!(!rule.evaluate(&ctx));

//         // Test usage limits
//         let mut maxed_coupon = create_test_coupon(coupon_code);
//         maxed_coupon.max_uses = Some(10);
//         maxed_coupon.used_count = 10;
//         ctx.applied_coupon = Some(maxed_coupon);
//         assert!(!rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_minimum_spend_condition() {
//         let mut rule = create_test_rule();
//         rule.conditions = vec![Condition::MinimumSpend {
//             amount: Decimal128::from_str("50.00").unwrap(),
//         }];

//         let ctx = create_test_context();
//         assert!(rule.evaluate(&ctx));

//         rule.conditions = vec![Condition::MinimumSpend {
//             amount: Decimal128::from_str("150.00").unwrap(),
//         }];
//         assert!(!rule.evaluate(&ctx));
//     }

//     #[test]
//     fn test_product_category_condition() {
//         let mut rule = create_test_rule();
//         rule.conditions = vec![Condition::ProductCategory {
//             category_ids: vec!["category1".to_string()],
//         }];

//         let ctx = create_test_context();
//         assert!(rule.evaluate(&ctx));

//         rule.conditions = vec![Condition::ProductCategory {
//             category_ids: vec!["category2".to_string()],
//         }];
//         assert!(!rule.evaluate(&ctx));
//     }
// }
