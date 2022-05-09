use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "crawl_queue")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    // Will definitely run into namespace issues later on, something to think about.
    #[sea_orm(unique)]
    pub name: String,
    pub author: String,
    pub description: Option<String>,
    pub version: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            ..ActiveModelTrait::default()
        }
    }
}

/// True if the lens was added, False if it already exists.
pub async fn add(
    db: &DatabaseConnection,
    name: &str,
    author: Option<&String>,
    description: Option<&String>,
    version: &str,
) -> anyhow::Result<bool> {
    let exists = Entity::find()
        .filter(Column::Name.eq(name.to_string()))
        .one(db)
        .await?;

    if exists.is_some() {
        return Ok(false);
    }

    let new_lens = ActiveModel {
        name: Set(name.to_owned()),
        author: Set(author.unwrap_or(&"Unknown".to_string()).to_string()),
        description: Set(description.map(String::from)),
        version: Set(version.to_owned()),
        ..Default::default()
    };
    new_lens.insert(db).await?;
    Ok(true)
}