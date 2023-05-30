use crate::{Capture, CaptureStatus, Order};

impl Order {
    /// Checks if a payment for an Order is already captured.
    pub fn is_already_captured(&self, purchase_unit_index: usize, capture_index: usize) -> bool {
        self.get_capture(purchase_unit_index, capture_index)
            .map_or(false, |capture| {
                capture.status == CaptureStatus::Completed.as_str()
            })
    }

    fn get_capture(&self, purchase_unit_index: usize, capture_index: usize) -> Option<&Capture> {
        self.purchase_units
            .as_ref()?
            .get(purchase_unit_index)?
            .payments
            .as_ref()?
            .captures
            .as_ref()?
            .get(capture_index)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Capture, CaptureStatus, Order, PaymentCollection, PurchaseUnit};

    #[test]
    fn is_already_captured_true() {
        let order = Order {
            purchase_units: Some(vec![PurchaseUnit {
                payments: Some(PaymentCollection {
                    authorizations: None,
                    captures: Some(vec![Capture {
                        status: CaptureStatus::Completed.to_string(),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            ..Default::default()
        };

        assert!(order.is_already_captured(0, 0));
    }
}
