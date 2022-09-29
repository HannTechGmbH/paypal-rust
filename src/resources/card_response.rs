use crate::resources::card_address_portable::CardAddressPortable;
use crate::resources::enums::card_type::CardType;
use crate::resources::enums::network::Network;
use crate::resources::enums::payment_card_type::PaymentCardType;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardResponse {
    /// The PayPal-generated ID for the card.
    pub id: Option<String>,

    /// The card holder's name as it appears on the card.
    pub name: Option<String>,

    /// The primary account number (PAN) for the payment card.
    pub number: String,

    /// The card expiration year and month, in Internet date format.
    pub expiry: String,

    /// The three- or four-digit security code of the card. Also known as the CVV, CVC, CVN, CVE, or CID.
    /// This parameter cannot be present in the request when `payment_initiator=MERCHANT`.
    pub security_string: Option<String>,

    /// The last digits of the payment card.
    pub last_digits: Option<String>,

    ///The card brand or network. Typically used in the response.
    pub card_type: Option<CardType>,

    /// The payment card type.
    #[serde(rename = "type")]
    pub type_: Option<PaymentCardType>,

    /// The card brand or network. Typically used in the response.
    pub brand: Option<Network>,

    /// The billing address for this card. Supports only the address_line_1, address_line_2, admin_area_1, admin_area_2, postal_code,
    /// and country_code properties.
    pub billing_address: Option<CardAddressPortable>,
}
