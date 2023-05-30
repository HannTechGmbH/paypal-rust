use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum Op {
    ///  Depending on the target location reference, completes one of these functions:
    ///  1. The target location is an array index. Inserts a new value into the array at the specified index.
    ///  2. The target location is an object parameter that does not already exist. Adds a new parameter to the object.
    ///  3. The target location is an object parameter that does exist. Replaces that parameter's value.
    ///  4. The value parameter defines the value to add. For more information, see 4.1. add.
    #[default]
    #[serde(rename = "add")]
    Add,
    /// Removes the value at the target location. For the operation to succeed, the target location must exist.
    #[serde(rename = "remove")]
    Remove,
    /// Replaces the value at the target location with a new value. The operation object must contain a value parameter that
    /// defines the replacement value. For the operation to succeed, the target location must exist.
    #[serde(rename = "replace")]
    Replace,
    /// Removes the value at a specified location and adds it to the target location. The operation object must contain a from
    /// parameter, which is a string that contains a JSON pointer value that references the location in the target document from which to
    /// move the value. For the operation to succeed, the from location must exist.
    #[serde(rename = "move")]
    Move,
    /// Copies the value at a specified location to the target location. The operation object must contain a from parameter,
    /// which is a string that contains a JSON pointer value that references the location in the target document from which to copy the
    /// value. For the operation to succeed, the from location must exist.
    #[serde(rename = "copy")]
    Copy,
    /// Tests that a value at the target location is equal to a specified value. The operation object must contain a value
    /// parameter that defines the value to compare to the target location's value. For the operation to succeed, the target location must
    /// be equal to the value value. For test, equal indicates that the value at the target location and the value that value defines are of
    /// the same JSON type. The data type of the value determines how equality is defined:
    /// * **strings**  Contain the same number of Unicode characters and their code points are byte-by-byte equal.
    /// * **numbers**  Are numerically equal.
    /// * **arrays**  Contain the same number of values, and each value is equal to the value at the corresponding position in the other array,
    ///               by using these type-specific rules.
    /// * **objects**  Contain the same number of parameters, and each parameter is
    /// * **equal** to a parameter in the other object, by comparing their keys (as strings) and their values (by using these type-specific rules).
    /// * **literals** (false, true, and null)  Are the same. The comparison is a
    ///                 logical comparison. For example, whitespace between the parameter values of an array is not significant.
    ///                 Also, ordering of the serialization of object parameters is not significant.
    #[serde(rename = "test")]
    Test,
}

impl Op {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Remove => "remove",
            Self::Replace => "replace",
            Self::Move => "move",
            Self::Copy => "copy",
            Self::Test => "test",
        }
    }
}

impl AsRef<str> for Op {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
