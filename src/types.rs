use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize)]
pub struct Record {
    pub name:  String,
    pub times: Vec<f64>,
}
