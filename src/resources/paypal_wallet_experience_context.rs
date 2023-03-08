use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{LandingPage, PaymentMethodPreference, ShippingPreference, UserAction};

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PayPalWalletExperienceContext {
    /// The label that overrides the business name in the PayPal account on the PayPal site.
    /// The pattern is defined by an external party and supports Unicode.
    pub brand_name: Option<String>,

    /// The URL where the customer is redirected after the customer cancels the payment.
    pub cancel_url: Option<String>,

    /// The type of landing page to show on the PayPal site for customer checkout.
    ///
    /// The possible values are:
    /// - LOGIN. When the customer clicks PayPal Checkout, the customer is redirected to a page to
    ///   log in to PayPal and approve the payment.
    /// - GUEST_CHECKOUT. When the customer clicks PayPal Checkout, the customer is redirected to a
    ///   page to enter credit or debit card and other relevant billing information required to
    ///   complete the purchase. This option has previously been also called as 'BILLING'
    /// - NO_PREFERENCE. When the customer clicks PayPal Checkout, the customer is redirected to
    ///   either a page to log in to PayPal and approve the payment or to a page to enter credit or
    ///   debit card and other relevant billing information required to complete the purchase,
    ///   depending on their previous interaction with PayPal.
    pub landing_page: Option<LandingPage>,

    /// The BCP 47-formatted locale of pages that the PayPal payment experience shows. PayPal
    /// supports a five-character code.  For example, da-DK, he-IL, id-ID, ja-JP, no-NO, pt-BR,
    /// ru-RU, sv-SE, th-TH, zh-CN, zh-HK, or zh-TW.
    pub locale: Option<String>,

    /// The merchant-preferred payment methods.
    ///
    /// The possible values are:
    /// - UNRESTRICTED. Accepts any type of payment from the customer.
    /// - IMMEDIATE_PAYMENT_REQUIRED. Accepts only immediate payment from the customer. For example,
    ///   credit card, PayPal balance, or instant ACH. Ensures that at the time of capture, the
    ///   payment does not have the `pending` status.
    pub payment_method_preference: Option<PaymentMethodPreference>,

    /// The URL where the customer is redirected after the customer approves the payment.
    pub return_url: Option<String>,

    /// The location from which the shipping address is derived.
    ///
    /// The possible values are:
    /// - GET_FROM_FILE. Get the customer-provided shipping address on the PayPal site.
    /// - NO_SHIPPING. Redacts the shipping address from the PayPal site. Recommended for digital
    ///   goods.
    /// - SET_PROVIDED_ADDRESS. Get the merchant-provided address. The customer cannot change this
    ///   address on the PayPal site. If merchant does not pass an address, customer can choose
    ///   the address on PayPal pages.
    pub shipping_preference: Option<ShippingPreference>,

    /// Configures a Continue or Pay Now checkout flow.
    ///     
    /// The possible values are:
    /// - CONTINUE. After you redirect the customer to the PayPal payment page, a Continue button
    ///   appears. Use this option when the final amount is not known when the checkout flow is
    ///   initiated and you want to redirect the customer to the merchant page without processing the
    /// payment.
    /// - PAY_NOW. After you redirect the customer to the PayPal payment page, a Pay Now button
    ///   appears. Use this option when the final amount is known when the checkout is initiated and
    ///   you want to process the payment immediately when the customer clicks Pay Now.
    pub user_action: Option<UserAction>,
}

impl PayPalWalletExperienceContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn brand_name(mut self, brand_name: String) -> Self {
        self.brand_name = Some(brand_name);
        self
    }

    pub fn cancel_url(mut self, cancel_url: String) -> Self {
        self.cancel_url = Some(cancel_url);
        self
    }

    pub fn landing_page(mut self, landing_page: LandingPage) -> Self {
        self.landing_page = Some(landing_page);
        self
    }

    pub fn locale(mut self, locale: String) -> Self {
        self.locale = Some(locale);
        self
    }

    pub fn payment_method_preference(
        mut self,
        payment_method_preference: PaymentMethodPreference,
    ) -> Self {
        self.payment_method_preference = Some(payment_method_preference);
        self
    }

    pub fn return_url(mut self, return_url: String) -> Self {
        self.return_url = Some(return_url);
        self
    }

    pub fn shipping_preference(mut self, shipping_preference: ShippingPreference) -> Self {
        self.shipping_preference = Some(shipping_preference);
        self
    }

    pub fn user_action(mut self, user_action: UserAction) -> Self {
        self.user_action = Some(user_action);
        self
    }
}
