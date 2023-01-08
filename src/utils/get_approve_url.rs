use crate::Order;

impl Order {
    /// Finds the `approve` URL in an order's HATEOAS links. If the `approve` URL is not found, then
    /// `None` is returned. The URL is used to redirect the user to PayPal to approve the
    /// order.
    pub fn get_approval_url(&self) -> Option<String> {
        Some(
            self.links
                .as_ref()?
                .iter()
                .find(|link| link.rel == "approve")?
                .href
                .clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LinkDescription;

    #[test]
    fn approve_url_exists() {
        let order = Order {
            links: Some(vec![
                LinkDescription {
                    href: "https://example.com".to_string(),
                    rel: "approve".to_string(),
                    method: Some(crate::resources::HttpMethod::Get),
                },
                LinkDescription {
                    href: "https://example.com".to_string(),
                    rel: "other".to_string(),
                    method: Some(crate::resources::HttpMethod::Get),
                },
            ]),
            ..Default::default()
        };

        let approve_url = order.get_approval_url().unwrap();
        assert_eq!(approve_url, "https://example.com");
    }
}
