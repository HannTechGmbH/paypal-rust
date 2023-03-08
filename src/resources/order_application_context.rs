use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::resources::enums::landing_page::LandingPage;
use crate::resources::enums::shipping_preference::ShippingPreference;
use crate::resources::enums::user_action::UserAction;
use crate::resources::payment_method::PaymentMethod;
use crate::resources::stored_payment_source::StoredPaymentSource;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrderApplicationContext {
    /// The label that overrides the business name in the PayPal account on the PayPal site.
    #[deprecated(
        note = "The fields in application_context are now available in the experience_context object"
    )]
    pub brand_name: Option<String>,

    /// The BCP 47-formatted locale of pages that the PayPal payment experience shows. PayPal supports a five-character code.
    /// For example, da-DK, he-IL, id-ID, ja-JP, no-NO, pt-BR, ru-RU, sv-SE, th-TH, zh-CN, zh-HK, or zh-TW.
    #[deprecated(
        note = "The fields in application_context are now available in the experience_context object"
    )]
    pub locale: Option<String>,

    /// The type of landing page to show on the PayPal site for customer checkout.
    #[deprecated(
        note = "The fields in application_context are now available in the experience_context object"
    )]
    pub landing_page: Option<LandingPage>,

    /// The shipping preference:
    ///  * Displays the shipping address to the customer.
    ///  * Enables the customer to choose an address on the PayPal site.
    ///  * Restricts the customer from changing the address during the payment-approval process.
    #[deprecated(
        note = "The fields in application_context are now available in the experience_context object"
    )]
    pub shipping_preference: Option<ShippingPreference>,

    /// Configures a Continue or Pay Now checkout flow.
    #[deprecated(
        note = "The fields in application_context are now available in the experience_context object"
    )]
    pub user_action: Option<UserAction>,

    ///The customer and merchant payment preferences.
    #[deprecated(
        note = "The fields in application_context are now available in the experience_context object"
    )]
    pub payment_method: Option<PaymentMethod>,

    /// The URL where the customer is redirected after the customer approves the payment.
    #[deprecated(
        note = "The fields in application_context are now available in the experience_context object"
    )]
    pub return_url: Option<String>,

    /// The URL where the customer is redirected after the customer cancels the payment.
    #[deprecated(
        note = "The fields in application_context are now available in the experience_context object"
    )]
    pub cancel_url: Option<String>,

    /// Provides additional details to process a payment using a payment_source that has been stored or is intended to be stored
    /// (also referred to as stored_credential or card-on-file).
    ///
    /// Parameter compatibility:
    /// payment_type=ONE_TIME is compatible only with payment_initiator=CUSTOMER.
    /// usage=FIRST is compatible only with payment_initiator=CUSTOMER.
    /// previous_transaction_reference or previous_network_transaction_reference is compatible only with payment_initiator=MERCHANT.
    /// Only one of the parameters - previous_transaction_reference and previous_network_transaction_reference - can be present in the
    /// request.
    #[deprecated(
        note = "The fields in application_context are now available in the experience_context object"
    )]
    pub stored_payment_source: Option<StoredPaymentSource>,
}

#[allow(deprecated)]
impl OrderApplicationContext {
    pub fn new() -> OrderApplicationContext {
        OrderApplicationContext::default()
    }

    pub fn brand_name(mut self, brand_name: String) -> OrderApplicationContext {
        self.brand_name = Some(brand_name);
        self
    }

    pub fn locale(mut self, locale: String) -> OrderApplicationContext {
        self.locale = Some(locale);
        self
    }

    pub fn landing_page(mut self, landing_page: LandingPage) -> OrderApplicationContext {
        self.landing_page = Some(landing_page);
        self
    }

    pub fn shipping_preference(
        mut self,
        shipping_preference: ShippingPreference,
    ) -> OrderApplicationContext {
        self.shipping_preference = Some(shipping_preference);
        self
    }

    pub fn user_action(mut self, user_action: UserAction) -> OrderApplicationContext {
        self.user_action = Some(user_action);
        self
    }

    pub fn payment_method(mut self, payment_method: PaymentMethod) -> OrderApplicationContext {
        self.payment_method = Some(payment_method);
        self
    }

    pub fn return_url(mut self, return_url: String) -> OrderApplicationContext {
        self.return_url = Some(return_url);
        self
    }

    pub fn cancel_url(mut self, cancel_url: String) -> OrderApplicationContext {
        self.cancel_url = Some(cancel_url);
        self
    }

    pub fn stored_payment_source(
        mut self,
        stored_payment_source: StoredPaymentSource,
    ) -> OrderApplicationContext {
        self.stored_payment_source = Some(stored_payment_source);
        self
    }
}
