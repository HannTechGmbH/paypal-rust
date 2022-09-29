use crate::resources::enums::tax_id_type::TaxIdType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxInfo {
    /// The customer's tax ID value.
    pub tax_id: String,

    /// The customer's tax ID type.
    pub tax_id_type: TaxIdType,
}
