use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Secrets;

impl Default for Secrets {
    fn default() -> Self {
        Self
    }
}