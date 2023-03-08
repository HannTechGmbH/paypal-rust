use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PayPalWalletName {
    /// When the party is a person, the party's given, or first, name.
    pub given_name: Option<String>,

    /// When the party is a person, the party's surname or family name. Also known as the last name.
    /// Required when the party is a person  Use also to store multiple surnames including the
    /// matronymic, or mother's, surname.
    pub surname: Option<String>,
}
