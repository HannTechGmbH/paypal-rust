use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Response<T> {
    pub headers: Vec<(String, String)>,
    pub body: T,
}
