use crate::Order;

/// Parses the `authorization_id` from an order.
/// **Note**: The order has to be authorized before attempting to parse the `authorization_id`.
///
/// # Arguments
/// * `order` - The order to parse the `authorization_id` from.
/// * `purchase_unit_index` - The index of the purchase unit to parse the `authorization_id` from.
/// * `authorization_index` - The index of the authorization to parse the `authorization_id` from.
///
/// # Returns
/// Returns the `authorization_id` if it was found, otherwise `None`.
pub fn get_authorization_id_from_order(
    order: &Order,
    purchase_unit_index: usize,
    authorization_index: usize,
) -> Option<String> {
    order
        .purchase_units
        .as_ref()?
        .get(purchase_unit_index)?
        .payments
        .as_ref()?
        .authorizations
        .as_ref()?
        .get(authorization_index)?
        .id
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorization_id_exists() {
        let order = Order {
            purchase_units: Some(vec![crate::resources::purchase_unit::PurchaseUnit {
                payments: Some(crate::resources::payment_collection::PaymentCollection {
                    authorizations: Some(vec![crate::resources::authorization_with_additional_data::AuthorizationWithAdditionalData {
                        id: Some("AUTH-123".to_string()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let authorization_id = get_authorization_id_from_order(&order, 0, 0).unwrap();
        assert_eq!(authorization_id, "AUTH-123");
    }
}
