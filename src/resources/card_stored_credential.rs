use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{NetworkTransactionReference, PaymentInitiator, PaymentType, Usage};

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardStoredCredential {
    /// The person or party who initiated or triggered the payment.
    ///
    /// The possible values are:
    /// - CUSTOMER. Payment is initiated with the active engagement of the customer. e.g. a
    ///   customer checking out on a merchant website.
    /// - MERCHANT. Payment is initiated by merchant on behalf of the customer without the active
    ///   engagement of customer. e.g. a merchant charging the monthly payment of a subscription to
    ///   the customer.
    pub payment_initiator: PaymentInitiator,

    /// Indicates the type of the stored payment_source payment.
    ///
    /// The possible values are:
    /// - ONE_TIME. One Time payment such as online purchase or donation.
    ///   (e.g. Checkout with one-click).
    /// - RECURRING. Payment which is part of a series of payments with fixed or variable amounts,
    ///   following a fixed time interval. (e.g. Subscription payments).
    /// - UNSCHEDULED. Payment which is part of a series of payments that occur on a non-fixed
    ///   schedule and/or have variable amounts. (e.g. Account Topup payments).
    pub payment_type: PaymentType,

    /// Reference values used by the card network to identify a transaction.
    pub previous_network_transaction_reference: Option<NetworkTransactionReference>,

    /// Indicates if this is a first or subsequent payment using a stored payment source
    /// (also referred to as stored credential or card on file).
    ///
    /// The possible values are:
    /// - FIRST. Indicates the Initial/First payment with a payment_source that is intended to be
    ///   stored upon successful processing of the payment.
    /// - SUBSEQUENT. Indicates a payment using a stored payment_source which has been successfully
    ///   used previously for a payment.
    /// - DERIVED. Indicates that PayPal will derive the value of `FIRST` or `SUBSEQUENT` based on
    ///   data available to PayPal.
    pub usage: Option<Usage>,
}
