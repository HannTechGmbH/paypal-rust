use std::ops::AddAssign;

use http_types::url::ParseError;
use http_types::Url;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::client::paypal::USER_AGENT;

/// For most REST GET calls, you can include one or more query parameters on the request URI to filter, limit the size of,
/// and sort the data in an API response. For filter parameters, see the individual GET calls.
/// To limit, or page, and sort the data that is returned in some API responses, use these, or similar, query parameters:
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct QueryParams {
    /// The number of items to list in the response.
    pub count: Option<i32>,

    /// The end date and time for the range to show in the response,
    /// in [Internet date and time format](https://tools.ietf.org/html/rfc3339#section-5.6).
    /// For example, end_time=2016-03-06T11:00:00Z.
    pub end_time: Option<i32>,

    /// The page number indicating which set of items will be returned in the response.
    /// So, the combination of page=1 and page_size=20 returns the first 20 items.
    /// The combination of page=2 and page_size=20 returns items 21 through 40.
    pub page: Option<i32>,

    /// The number of items to return in the response.
    pub page_size: Option<i32>,

    /// Indicates whether to show the total count in the response.
    pub total_count_required: Option<bool>,

    /// Sorts the payments in the response by a specified value, such as the create time or update time.
    pub sort_by: Option<String>,

    /// Sorts the items in the response in ascending or descending order.
    pub sort_order: Option<String>,

    /// The ID of the starting resource in the response. When results are paged, you can use the
    /// next_id value as the start_id to continue with the next set of results.
    pub start_id: Option<String>,

    /// The start index of the payments to list. Typically, you use the start_index to jump to a
    /// specific position in the resource history based on its cart. For example, to start at the
    /// second item in a list of results, specify ?start_index=2.
    pub start_index: Option<i32>,

    /// The start date and time for the range to show in the response, in Internet date and time format.
    /// For example, start_time=2016-03-06T11:00:00Z.
    pub start_time: Option<String>,
}

impl QueryParams {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn count(mut self, count: i32) -> Self {
        self.count = Some(count);
        self
    }

    #[must_use]
    pub const fn end_time(mut self, end_time: i32) -> Self {
        self.end_time = Some(end_time);
        self
    }

    #[must_use]
    pub const fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    #[must_use]
    pub const fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    #[must_use]
    pub const fn total_count_required(mut self, total_count_required: bool) -> Self {
        self.total_count_required = Some(total_count_required);
        self
    }

    #[must_use]
    pub fn sort_by(mut self, sort_by: String) -> Self {
        self.sort_by = Some(sort_by);
        self
    }

    #[must_use]
    pub fn sort_order(mut self, sort_order: String) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    #[must_use]
    pub fn start_id(mut self, start_id: String) -> Self {
        self.start_id = Some(start_id);
        self
    }

    #[must_use]
    pub const fn start_index(mut self, start_index: i32) -> Self {
        self.start_index = Some(start_index);
        self
    }

    #[must_use]
    pub fn start_time(mut self, start_time: String) -> Self {
        self.start_time = Some(start_time);
        self
    }
}

/// The commonly used HTTP request headers
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpRequestHeaders {
    /// The response format, which is required for operations with a response body.
    #[serde(rename = "Accept")]
    pub accept: String,

    /// Required to get an access token or make API calls:
    /// * Get an access token `partial:partials/docs/oauth-credentials.en-XC`
    ///  `partial:partials/docs/access-tokens.en-XC` `partial:partials/docs/rest/bearer-token.en-XC`
    /// * Make REST API calls
    ///
    /// Include the access token in the Authorization header with the `Bearer` authentication scheme.
    #[serde(rename = "Authorization")]
    pub authorization: String,

    /// The request format, which is required for operations with a request body.
    #[serde(rename = "Content-Type")]
    pub content_type: String,

    /// An API client-provided JSON Web Token (JWT) assertion that identifies the merchant.
    ///
    /// To use this header, you must get consent to act on behalf of a merchant.
    /// You might want to use a JWT if you act on behalf of multiple merchants at the same time,
    /// because it is difficult and expensive to generate and manage multiple access tokens.
    /// Instead of managing multiple access tokens, you can use this header to provide a JWT
    /// assertion that identifies the merchant when you call the API.
    ///
    /// ### Constructing the JWT
    /// While you can use either a signed or unsigned (unsecured) JWT, PayPal recommends using the
    /// unsigned JWT for simplicity because the information you pass with the JWT is not sensitive
    /// data.
    ///
    /// A JWT consists of three parts:
    /// * Header: Identifies the algorithm that generated the signature. Because PayPal recommends
    ///           an unsigned JWT, pass an alg of none for the header.
    /// * Payload: Defines a set of claims, or statements, about an entity. In the case of the
    ///            PayPal-Auth-Assertion header, the entity is typically the merchant.
    ///            The payload can contain iss, which identifies the third-party calling the API,
    ///            and one of the following to identify the merchant for whom the call is made:
    ///            email or payer_id.
    ///            **Note**: Because a merchant's email address can change, PayPal recommends
    ///                      using payer_id to specify a merchant.
    /// * Signature: Validates the token and consists of the encoded header, the encoded payload,
    ///              a secret, and the algorithm. Because PayPal recommends an unsigned JWT, pass an
    ///              empty string for the signature.
    ///             **Note**: If you prefer a signed JWT, you can sign it using your secret from
    ///                       your API credentials.
    ///
    /// # Examples
    /// JOSE header:
    /// "alg": "none"
    /// Payload:
    /// ```json
    /// "iss": "client_id",
    /// "payer_id": "merchant_payer_id"
    /// ```
    /// Signature. Use "" for the unsigned case.
    ///
    /// To pass the JWT easily in HTTP environments, use Base64 to encode all three parts of the
    /// JWT separately, then concatenate them with a period (.). The following code shows the
    /// previous example after Base64 encoding and compact serialization (or concatenation):
    /// `ew0KICAiYWxnIjogIm5vbmUiDQp9.ew0KICAiaXNzIjogImNsaWVudF9pZCIsDQogICJwYXllcl9pZCI6ICJtZXJjaGFudF9wYXllcl9pZCINCn0=.`
    ///
    /// Refer to [Issue a Refund](https://developer.paypal.com/docs/multiparty/issue-refund/)
    /// in the PayPal Commerce Platform documentation for an example of using the
    /// `PayPal-Auth-Assertion` header in an API call.
    #[serde(
        rename = "PayPal-Auth-Assertion",
        skip_serializing_if = "Option::is_none"
    )]
    pub paypal_auth_assertion: Option<String>,

    /// Verifies that the payment originates from a valid, user-consented device and application.
    /// Reduces fraud and decreases declines. Transactions that do not include a client metadata ID
    /// are not eligible for PayPal Seller Protection.
    #[serde(
        rename = "PayPal-Client-Metadata-Id",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_client_metadata_id: Option<String>,

    /// Optional. Identifies the caller as a PayPal partner. To receive revenue attribution,
    /// specify a unique build notation (BN) code. BN codes provide tracking on all transactions
    /// that originate or are associated with a particular partner. To find your BN code,
    /// see [Code andCredential Reference](https://developer.paypal.com/docs/multiparty/get-started#code-and-credential-reference).
    #[serde(
        rename = "PayPal-Partner-Attribution-Id",
        skip_serializing_if = "Option::is_none"
    )]
    pub paypal_partner_attribution_id: Option<String>,

    /// For example, a user calls refund captured payment with the PayPal-Request-Id header that
    /// contains a unique user-provided ID. The user can make the call again with the same ID in the
    /// `PayPal-Request-Id` header for up to 45 days because the server stores this ID for this long
    /// for this call.
    ///
    /// If the initial call fails with the HTTP `500` status code but the server has already
    /// refunded the payment, the caller does not need to worry that the server will refund the
    /// payment again.
    ///
    /// **Note**: Not all APIs support this header. To determine whether your API supports it and
    /// for information about how long the server stores the ID, see the reference for your API.
    #[serde(rename = "PayPal-Request-Id", skip_serializing_if = "Option::is_none")]
    pub paypal_request_id: Option<String>,

    #[serde(rename = "User-Agent")]
    pub user_agent: String,
}

impl Default for HttpRequestHeaders {
    fn default() -> Self {
        Self {
            accept: "application/json".to_string(),
            authorization: "Bearer ".to_string(),
            content_type: "application/json".to_string(),
            user_agent: USER_AGENT.to_string(),
            paypal_partner_attribution_id: None,
            client_client_metadata_id: None,
            paypal_auth_assertion: None,
            paypal_request_id: None,
        }
    }
}

impl HttpRequestHeaders {
    pub fn new(authentication: impl Into<String>) -> Self {
        Self {
            authorization: authentication.into(),
            ..Default::default()
        }
    }

    pub fn to_vec(&self) -> Vec<(&str, &str)> {
        let mut headers = Vec::new();
        headers.push(("Accept", self.accept.as_str()));
        headers.push(("Content-Type", self.content_type.as_str()));
        headers.push(("User-Agent", self.user_agent.as_str()));
        if let Some(paypal_partner_attribution_id) = &self.paypal_partner_attribution_id {
            headers.push((
                "PayPal-Partner-Attribution-Id",
                paypal_partner_attribution_id.as_str(),
            ));
        }
        if let Some(client_client_metadata_id) = &self.client_client_metadata_id {
            headers.push((
                "PayPal-Client-Metadata-Id",
                client_client_metadata_id.as_str(),
            ));
        }
        if let Some(paypal_auth_assertion) = &self.paypal_auth_assertion {
            headers.push(("PayPal-Auth-Assertion", paypal_auth_assertion.as_str()));
        }
        if let Some(paypal_request_id) = &self.paypal_request_id {
            headers.push(("PayPal-Request-Id", paypal_request_id.as_str()));
        }
        headers
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub url: Url,
    pub headers: HttpRequestHeaders,
    pub query: Option<QueryParams>,
    pub body: Option<serde_json::Value>,
    pub method: reqwest::Method,
}

impl Request {
    pub const fn new(
        url: Url,
        headers: HttpRequestHeaders,
        query: Option<QueryParams>,
        body: Option<serde_json::Value>,
    ) -> Self {
        Self {
            url,
            headers,
            query,
            body,
            method: reqwest::Method::GET,
        }
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            url: RequestUrl::Sandbox.as_url().expect("Invalid URL"),
            headers: Default::default(),
            query: None,
            body: None,
            method: reqwest::Method::GET,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum RequestUrl {
    #[default]
    Sandbox,
    Live,
}

impl RequestUrl {
    pub fn as_url(&self) -> Result<Url, ParseError> {
        let sandbox_url = Url::parse("https://api-m.sandbox.paypal.com")?;
        let live_url = Url::parse("https://api-m.paypal.com")?;

        Ok(match self {
            Self::Sandbox => sandbox_url,
            Self::Live => live_url,
        })
    }
}

impl std::fmt::Display for RequestUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_url().unwrap().as_str().fmt(formatter)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct RetryCount(u32);

impl RetryCount {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl From<i32> for RetryCount {
    fn from(value: i32) -> Self {
        if value < 0 {
            Self(0)
        } else {
            Self(value as u32)
        }
    }
}

impl AddAssign for RetryCount {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

/// The strategy to use for executing a request. Defaults to `RetryStrategy::Once`.
#[derive(Clone, Debug, Default)]
pub enum RequestStrategy {
    /// Fire the request once and return the response.
    #[default]
    Once,
    /// Fire the request once and return the response. If the response is an error, retry the request
    Retry(RetryCount),
}

impl RequestStrategy {
    pub const fn is_retry(&self) -> bool {
        matches!(self, Self::Retry(_))
    }

    pub const fn get_retry_count(&self) -> Option<&RetryCount> {
        match self {
            Self::Once => None,
            Self::Retry(count) => Some(count),
        }
    }
}
