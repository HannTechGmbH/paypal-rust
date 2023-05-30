use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum TaxIdType {
    /// The individual tax ID type, typically is 11 characters long.
    #[serde(rename = "BR_CPF")]
    BrCPF,
    /// The business tax ID type, typically is 14 characters long.
    #[serde(rename = "BR_CNPJ")]
    BrCNPJ,
}

impl Default for TaxIdType {
    fn default() -> Self {
        Self::BrCPF
    }
}

impl TaxIdType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrCPF => "BR_CPF",
            Self::BrCNPJ => "BR_CNPJ",
        }
    }
}

impl AsRef<str> for TaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIdType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
