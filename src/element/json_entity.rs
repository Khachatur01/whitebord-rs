use crate::element::id::Id;
use crate::element::Build;
use entity_model_feature::entity::Entity;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonEntity {
    pub id: Id,
    pub model: serde_json::Value,
}

/* Try to parse json string. */
impl TryFrom<&str> for JsonEntity {
    type Error = serde_json::Error;

    fn try_from(entity: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(entity)
    }
}

/* Try to parse json string. */
impl TryInto<Entity<Id>> for JsonEntity {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<Entity<Id>, Self::Error> {
        Build::FromJson(self).build()
    }
}
