use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::resources::enums::op::Op;
use crate::resources::money::Money;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Patch {
    /// The operation.
    pub op: Op,

    /// The JSON Pointer to the target document location at which to complete the operation.
    pub path: Option<String>,

    /// The value to apply. The `remove` operation does not require a value.
    pub value: Option<PatchValue>,

    /// The JSON Pointer to the target document location from which to move the value. Required for the move operation.
    pub from: Option<String>,
}

impl Patch {
    pub fn new(op: Op) -> Self {
        Self {
            op,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    #[must_use]
    pub fn value(mut self, value: PatchValue) -> Self {
        self.value = Some(value);
        self
    }

    #[must_use]
    pub fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum PatchValue {
    Int(i32),
    Boolean(bool),
    String(String),
    Vec(Vec<PatchValue>),
    Money(Money),
}

impl PatchValue {
    #[must_use]
    pub fn money(self, money: Money) -> Self {
        Self::Money(money)
    }

    #[must_use]
    pub fn string(self, string: String) -> Self {
        Self::String(string)
    }

    #[must_use]
    pub fn bool(self, boolean: bool) -> Self {
        Self::Boolean(boolean)
    }

    #[must_use]
    pub fn vec(self, vec: Vec<PatchValue>) -> Self {
        Self::Vec(vec)
    }

    #[must_use]
    pub fn int(self, int: i32) -> Self {
        Self::Int(int)
    }
}
