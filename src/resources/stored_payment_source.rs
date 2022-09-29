use crate::resources::enums::payment_initiator::PaymentInitiator;
use crate::resources::enums::payment_type::PaymentType;
use crate::resources::enums::usage::Usage;
use crate::resources::network_transaction_reference::NetworkTransactionReference;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StoredPaymentSource {
    /// The person or party who initiated or triggered the payment.
    pub payment_initiator: PaymentInitiator,

    /// Indicates the type of the stored payment_source payment.
    pub payment_type: PaymentType,

    /// Indicates if this is a first or subsequent payment using a stored payment source
    /// (also referred to as stored credential or card on file).
    pub usage: Option<Usage>,

    /// Reference values used by the card network to identify a transaction.
    pub previous_network_transaction_reference: Option<NetworkTransactionReference>,
}
