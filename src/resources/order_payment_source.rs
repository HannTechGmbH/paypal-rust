use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    BancontactRequest, BlikRequest, CardRequest, EpsRequest, GiropayRequest, IdealRequest,
    MyBankRequest, P24Request, PayPalWallet, SofortRequest, Token, TrustlyRequest,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentSource {
    /// Bancontact is the most popular online payment in Belgium.
    pub bancontact: Option<BancontactRequest>,

    /// BLIK is a mobile payment system, created by Polish Payment Standard in order to allow
    /// millions of users to pay in shops, payout cash in ATMs and make online purchases and
    /// payments.
    pub blik: Option<BlikRequest>,

    /// The payment card to use to fund a payment. Can be a credit or debit card.
    ///
    /// **Note:** Note: Passing card number, cvv and expiry directly via the API requires
    /// PCI SAQ D compliance. PayPal offers a mechanism by which you do not have to take on the PCI
    /// SAQ D burden by using hosted fields - refer to this Integration Guide.
    pub card: Option<CardRequest>,

    /// The eps transfer is an online payment method developed by many Austrian banks.
    pub eps: Option<EpsRequest>,

    /// Giropay is an Internet payment System in Germany, based on online banking.
    pub giropay: Option<GiropayRequest>,

    /// The Dutch payment method iDEAL is an online payment method that enables consumers to pay
    /// online through their own bank.
    pub ideal: Option<IdealRequest>,

    /// MyBank is an e-authorisation solution which enables safe digital payments and identity
    /// authentication through a consumer’s own online banking portal or mobile application.
    pub mybank: Option<MyBankRequest>,

    /// P24 (Przelewy24) is a secure and fast online bank transfer service linked to all the major
    /// banks in Poland.
    pub p24: Option<P24Request>,

    /// Indicates that PayPal Wallet is the payment source. Main use of this selection is to
    /// provide additional instructions associated with this choice like vaulting.
    pub paypal: Option<PayPalWallet>,

    /// SOFORT Banking is a real-time bank transfer payment method that buyers use to transfer
    /// funds directly to merchants from their bank accounts.
    pub sofort: Option<SofortRequest>,

    /// The tokenized payment source to fund a payment.
    pub token: Option<Token>,

    /// Trustly is a payment method that allows customers to shop and pay from their bank account.
    pub trustly: Option<TrustlyRequest>,
}
