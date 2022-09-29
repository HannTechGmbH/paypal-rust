use crate::resources::enums::http_method::HttpMethod;
use serde::{Deserialize, Serialize};
// use serde_with::skip_serializing_none;

// #[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LinkDescription {
    /// The complete target URL. To make the related call, combine the method with this URI Template-formatted link. For pre-processing,
    /// include the $, (, and ) characters. The href is the key HATEOAS component that links a completed call with a subsequent call.
    pub href: String,

    /// The link relation type, which serves as an ID for a link that unambiguously describes the semantics of the link. See Link Relations.
    pub rel: String,

    /// The HTTP method required to make the related call.
    pub method: HttpMethod,
}
