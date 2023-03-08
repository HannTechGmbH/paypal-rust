use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{CardAddressPortable, CardStoredCredential};

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardRequest {
    /// The billing address for this card. Supports only the address_line_1, address_line_2,
    /// admin_area_1, admin_area_2, postal_code, and country_code properties.
    pub billing_address: Option<CardAddressPortable>,

    /// The card expiration year and month, in Internet date format.
    pub expiry: Option<String>,

    /// The card holder's name as it appears on the card.
    pub name: Option<String>,

    /// The primary account number (PAN) for the payment card.
    pub number: Option<String>,

    /// The three- or four-digit security code of the card. Also known as the CVV, CVC, CVN, CVE, or CID. This parameter cannot be present in the request when payment_initiator=MERCHANT.
    pub security_code: Option<String>,

    /// Provides additional details to process a payment using a card that has been stored or is
    /// intended to be stored (also referred to as stored_credential or card-on-file).
    /// Parameter compatibility:
    /// - payment_type=ONE_TIME is compatible only with payment_initiator=CUSTOMER.
    /// - usage=FIRST is compatible only with payment_initiator=CUSTOMER.
    /// - previous_transaction_reference or previous_network_transaction_reference is compatible
    ///   only with payment_initiator=MERCHANT.
    /// - Only one of the parameters - previous_transaction_reference and
    /// previous_network_transaction_reference - can be present in the request.
    pub stored_credential: Option<CardStoredCredential>,

    /// The PayPal-generated ID for the saved card payment source. Typically stored on the merchant's server.
    pub vault_id: Option<String>,
}
