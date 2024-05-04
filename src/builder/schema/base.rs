use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct BaseModel {
    pub id: i32,
    pub name: String,
}

table! {
    base_models (id) {
        id -> Integer,
        name -> Text,
    }
}
