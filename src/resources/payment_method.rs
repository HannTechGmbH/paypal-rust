use crate::resources::enums::payee_preferred::PayeePreferred;
use crate::resources::enums::standard_entry_class_code::StandardEntryClassCode;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethod {
    /// The merchant-preferred payment methods.
    ///
    /// The possible values are:
    ///
    /// - UNRESTRICTED. Accepts any type of payment from the customer.
    /// - IMMEDIATE_PAYMENT_REQUIRED. Accepts only immediate payment from the customer. For example, credit card, PayPal balance,
    ///   or instant ACH. Ensures that at the time of capture, the payment does not have the `pending` status.
    pub payee_preferred: Option<PayeePreferred>,

    /// NACHA (the regulatory body governing the ACH network) requires that API callers (merchants, partners) obtain the consumer’s explicit
    /// authorization before initiating a transaction. To stay compliant, you’ll need to make sure that you retain a compliant authorization
    /// for each transaction that you originate to the ACH Network using this API. ACH transactions are categorized (using SEC codes) by how
    /// you capture authorization from the Receiver (the person whose bank account is being debited or credited). PayPal supports the
    /// following SEC codes.
    ///
    /// The possible values are:
    ///
    /// - TEL. The API caller (merchant/partner) accepts authorization and payment information from a consumer over the telephone.
    /// - WEB. The API caller (merchant/partner) accepts Debit transactions from a consumer on their website.
    /// - CCD. Cash concentration and disbursement for corporate debit transaction. Used to disburse or consolidate funds. Entries are
    ///   usually Optional high-dollar, low-volume, and time-critical. (e.g. intra-company transfers or invoice payments to suppliers).
    /// - PPD. Prearranged payment and deposit entries. Used for debit payments authorized by a consumer account holder, and usually
    ///   initiated by a company. These are usually recurring debits (such as insurance premiums).
    pub standard_entry_class_code: Option<StandardEntryClassCode>,
}
