use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Email {
    /// Up to 64 characters are allowed before and 255 characters are allowed
    /// after the @ sign. However, the generally accepted maximum length for an
    /// email address is 254 characters. The pattern verifies that an unquoted
    /// @ sign exists.
    pub value: String,

    /// Indicates whether the email address is the user's primary address.
    pub primary: bool,
}
