use crate::ShippingPreference;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExperienceContextBase {
    /// The label that overrides the business name in the PayPal account on the PayPal site.
    /// The pattern is defined by an external party and supports Unicode.
    pub brand_name: Option<String>,

    /// The URL where the customer is redirected after the customer cancels the payment.
    pub cancel_url: Option<String>,

    /// The BCP 47-formatted locale of pages that the PayPal payment experience shows. PayPal
    /// supports a five-character code.  For example, da-DK, he-IL, id-ID, ja-JP, no-NO, pt-BR,
    /// ru-RU, sv-SE, th-TH, zh-CN, zh-HK, or zh-TW.
    pub locale: Option<String>,

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
}
