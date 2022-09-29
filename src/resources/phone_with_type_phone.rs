use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PhoneWithTypePhone {
    /// The national number, in its canonical international E.164 numbering plan format. The combined length of the
    /// country calling code (CC) and the national number must not be greater than 15 digits. The national number consists of a national
    /// destination code (NDC) and subscriber number (SN).
    pub national_number: String,
}
