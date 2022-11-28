use crate::resources::enums::http_method::HttpMethod;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LinkDescription {
    /// The complete target URL. To make the related call, combine the method with this URI Template-formatted link. For pre-processing,
    /// include the $, (, and ) characters. The href is the key HATEOAS component that links a completed call with a subsequent call.
    pub href: String,

    /// The link relation type, which serves as an ID for a link that unambiguously describes the semantics of the link. See Link Relations.
    pub rel: String,

    /// The HTTP method. If present, use this method to make a request to the target URL. If absent, the default method is GET.
    pub method: Option<HttpMethod>,
}
