use super::App;
use serde::{Deserialize, Serialize};

pub trait AppsRepository {
    async fn get_app() -> anyhow::Result<App>;
    async fn create_app() -> anyhow::Result<App>;
    async fn delete_app() -> anyhow::Result<()>;
    async fn list_apps() -> anyhow::Result<Vec<App>>;
    async fn update_app() -> anyhow::Result<App>;
}

#[derive(Debug, Clone)]
pub struct DynamoAppsRepository {
    pub dynamo_client: aws_sdk_dynamodb::Client,
    pub table_name: String,
}

impl DynamoAppsRepository {
    pub fn new(dynamo_client: aws_sdk_dynamodb::Client, table_name: String) -> Self {
        Self {
            dynamo_client,
            table_name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDynamoItem {
    pk: String,
    sk: String,
    entity_type: String,

    #[serde(flatten)]
    app: App,
}

impl AppsRepository for DynamoAppsRepository {
    async fn get_app() -> anyhow::Result<App> {
        todo!()
    }

    async fn create_app() -> anyhow::Result<App> {
        todo!()
    }

    async fn delete_app() -> anyhow::Result<()> {
        todo!()
    }

    async fn list_apps() -> anyhow::Result<Vec<App>> {
        todo!()
    }

    async fn update_app() -> anyhow::Result<App> {
        todo!()
    }
}
