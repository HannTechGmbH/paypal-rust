use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Name {
    /// The prefix, or title, to the party's name.
    pub prefix: Option<String>,

    /// When the party is a person, the party's given, or first, name.
    pub given_name: Option<String>,

    /// When the party is a person, the party's surname or family name. Also known as the last name. Required when the party is a person.
    /// Use also to store multiple surnames including the matronymic, or mother's, surname.
    pub surname: Option<String>,

    /// When the party is a person, the party's middle name.
    /// Use also to store multiple middle names including the patronymic, or father's, middle name.
    pub middle_name: Option<String>,

    /// The suffix for the party's name.
    pub suffix: Option<String>,

    /// When the party is a person, the party's full name.
    pub full_name: Option<String>,
}
