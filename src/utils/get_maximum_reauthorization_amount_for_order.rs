use crate::Order;

impl Order {
    /// Gets the maximum reauthorization amount for an order.
    /// You can reauthorize an authorized payment once for up to 115% of the original authorized
    /// amount not to exceed an increase of $75.
    ///
    /// # Returns
    /// Returns the maximum reauthorization amount. If the purchase unit does not have an authorized
    /// payment, then `None` is returned.
    pub fn get_maximum_reauthorization_amount(&self, purchase_unit_index: usize) -> Option<f32> {
        let string_price = self
            .purchase_units
            .as_ref()?
            .get(purchase_unit_index)
            .as_ref()?
            .amount
            .as_ref()?
            .value
            .clone();

        match &string_price.parse::<f32>() {
            Err(_) => None,
            Ok(price) => {
                // Round to the nearest cent.
                let reauthorization_amount = ((price * 1.15) * 100.0).round() / 100.0;
                if reauthorization_amount > price + 75.0 {
                    Some(price + 75.0)
                } else {
                    Some(reauthorization_amount)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AmountWithBreakdown, PurchaseUnit};

    #[test]
    fn test_get_maximum_reauthorization_amount() {
        let order = create_order("100.00");
        let maximum_reauthorization_amount = order.get_maximum_reauthorization_amount(0).unwrap();
        assert_eq!(maximum_reauthorization_amount, 115.0);
    }

    #[test]
    fn test_maximum_threshold_reached() {
        let order = create_order("600.00");
        let maximum_reauthorization_amount = order.get_maximum_reauthorization_amount(0).unwrap();
        assert_eq!(maximum_reauthorization_amount, 675.0);
    }

    fn create_order(price: &str) -> Order {
        Order {
            purchase_units: Some(vec![PurchaseUnit {
                amount: Some(AmountWithBreakdown {
                    value: price.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            ..Default::default()
        }
    }
}
