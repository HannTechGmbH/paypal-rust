use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::LinkDescription;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WebhookEvent {
    /// The date and time when the webhook event notification was created, in Internet date and time format.
    pub create_time: String,

    /// The event that triggered the webhook event notification.
    pub event_type: String,

    /// The event version in the webhook notification.
    pub event_version: String,

    /// The ID of the webhook event notification.
    pub id: String,

    /// An array of request-related HATEOAS links.
    pub links: Vec<LinkDescription>,

    /// The resource that triggered the webhook event notification.
    pub resource: serde_json::Value,

    /// The name of the resource related to the webhook notification event.
    pub resource_type: String,

    /// resource_versionstring
    /// The resource version in the webhook notification.
    pub resource_version: String,

    /// A summary description for the event notification.
    pub summary: String,
}
