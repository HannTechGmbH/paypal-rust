use crate::resources::address::Address;
use crate::resources::email::Email;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserInfo {
    /// The Private Personal Identifier (PPID) that is unique for the end user and Relying Party.
    pub user_id: String,

    /// The full name of the user. Includes all name parts, including titles and suffixes.
    /// The user's locale and preferences determine the syntax.
    pub name: String,

    /// The given, or first, name of the user.
    pub given_name: String,

    /// The surname or family name of the user. Also known as the last name. Used also to store multiple surnames including the matronymic,
    /// or mother's, surname.+
    pub family_name: String,

    /// The end user's external PayPal account ID. Returned only if the access_token
    /// has the https://uri.paypal.com/services/paypalattributes scope.
    pub payer_id: String,

    /// The end-user's preferred address.
    pub address: Address,

    /// The end user’s PayPal account status. Indicates whether the account is verified or not.
    pub verified_account: String,

    /// An array of email addresses for the user.
    pub emails: Vec<Email>,
}
