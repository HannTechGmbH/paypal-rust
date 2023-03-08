use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{PayPalAddressPortable, PayPalWalletExperienceContext, PayPalWalletName, TaxInfo};

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PayPalWallet {
    /// The address of the PayPal account holder. Supports only the address_line_1, address_line_2,
    /// admin_area_1, admin_area_2, postal_code, and country_code properties. Also referred to as
    /// the billing address of the customer.
    pub address: Option<PayPalAddressPortable>,

    /// The birth date of the PayPal account holder in YYYY-MM-DD format.
    pub birth_date: Option<String>,

    ///  The email address of the PayPal account holder.
    pub email_address: Option<String>,

    /// Customizes the payer experience during the approval process for payment with PayPal.
    /// **Note**: Partners and Marketplaces might configure brand_name and shipping_preference
    /// during partner account setup, which overrides the request values.
    pub experience_context: Option<PayPalWalletExperienceContext>,

    /// The name of the PayPal account holder. Supports only the given_name and surname properties.
    pub name: Option<PayPalWalletName>,

    /// The tax information of the PayPal account holder. Required only for Brazilian PayPal account
    /// holder's. Both tax_id and tax_id_type are required.
    pub tax_info: Option<TaxInfo>,
}
