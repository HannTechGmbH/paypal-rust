use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DateNoTime {
    /// The stand-alone date, in Internet date and time format. To represent special legal values, such as a date of birth,
    /// you should use dates with no associated time or time-zone data. Whenever possible, use the standard date_time type.
    /// This regular expression does not validate all dates. For example, February 31 is valid and nothing is known about leap years.
    ///
    /// Pattern: ^[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_no_time: Option<String>,
}
