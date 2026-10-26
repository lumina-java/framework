use lumina::prelude::*;

#[test]
fn test_prelude_financial_types_and_uuid() {
    let id = Uuid::new_v4();
    assert_eq!(id.get_version_num(), 4);

    let price = dec!(150000.75);
    let qty = dec!(3);
    let total = price * qty;
    assert_eq!(total.to_string(), "450002.25");
}
