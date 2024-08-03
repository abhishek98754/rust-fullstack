use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "posts")]
pub struct Mode1 {
    #[sea_orm(primary_key)]
    #serde(skip_deserializing)
    pub id:i32,
    pub title: String,
    pub text: String,
}

#[derive(Copy, clone, Febug, Emulter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}