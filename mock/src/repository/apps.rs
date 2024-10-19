use super::{
    AddAppError, App, CreateAppError, DeleteAppError, GetAppError, ListAppsError, UpdateAppError,
};
use async_trait::async_trait;
use aws_sdk_dynamodb::types::{AttributeValue, ReturnValue};
use serde::{Deserialize, Serialize};
use serde_dynamo::aws_sdk_dynamodb_1::{from_item, from_items, to_item};
use tracing::instrument;

#[async_trait]
pub trait AppsRepository: std::fmt::Debug + Send + Sync + Clone {
    async fn get_app(&self, id: u16) -> Result<App, GetAppError>;
    async fn add_app(&self, app: App) -> Result<App, AddAppError>;
    async fn create_app(&self, app: App) -> Result<App, CreateAppError>;
    async fn delete_app(&self, id: u16) -> Result<(), DeleteAppError>;
    async fn list_apps(&self) -> Result<Vec<App>, ListAppsError>;
    async fn update_app(&self, app: App) -> Result<App, UpdateAppError>;
    async fn get_id(&self) -> Result<u16, ()>;
}

#[derive(Debug, Clone)]
pub struct DynamoAppsRepository {
    pub dynamo_client: aws_sdk_dynamodb::Client,
    pub table_name: String,
}

impl DynamoAppsRepository {
    #[instrument]
    pub fn new(dynamo_client: aws_sdk_dynamodb::Client, table_name: String) -> Self {
        Self {
            dynamo_client,
            table_name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDynamoItem {
    pk: u16,
    entity_type: String,

    #[serde(flatten)]
    app: App,
}

impl AppDynamoItem {
    #[instrument]
    pub fn new(app: App) -> Self {
        Self {
            pk: app.id,
            entity_type: "app".to_string(),
            app,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DynamoCounter {
    pk: String,
    count: u16,
}

#[async_trait]
impl AppsRepository for DynamoAppsRepository {
    #[instrument]
    async fn get_app(&self, id: u16) -> Result<App, GetAppError> {
        let result = self
            .dynamo_client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(id.to_string()))
            .send()
            .await
            .map_err(|e| {
                eprintln!("{:#?}", e);
                GetAppError::UnexpectedError
            })?;

        if let Some(item) = result.item {
            let app: App = from_item(item)?;

            Ok(app)
        } else {
            Err(GetAppError::ResourceNotFound(id.to_string()))
        }
    }

    #[instrument]
    async fn add_app(&self, app: App) -> Result<App, AddAppError> {
        todo!()
    }

    #[instrument]
    async fn create_app(&self, app: App) -> Result<App, CreateAppError> {
        todo!()
    }

    #[instrument]
    async fn delete_app(&self, id: u16) -> Result<(), DeleteAppError> {
        todo!()
    }

    #[instrument]
    async fn list_apps(&self) -> Result<Vec<App>, ListAppsError> {
        let result = self
            .dynamo_client
            .scan()
            .table_name(&self.table_name)
            .filter_expression("pk <> :pk")
            .expression_attribute_values(":pk", AttributeValue::S("atomic_counter".to_string()))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("DynamoDB SDK Error: {:?}", e);
                ListAppsError::UnexpectedError
            })?;

        if let Some(items) = result.items {
            let apps: Vec<App> = from_items(items)?;

            Ok(apps)
        } else {
            Ok(Vec::new())
        }
    }

    #[instrument]
    async fn update_app(&self, app: App) -> Result<App, UpdateAppError> {
        todo!()
    }

    #[instrument]
    async fn get_id(&self) -> Result<u16, ()> {
        let result = self
            .dynamo_client
            .update_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S("atomic_counter".to_string()))
            .update_expression("set count = :count + 1")
            .return_values(ReturnValue::AllNew)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("DynamoDB SDK Error: {:?}", e);
            })?;

        if let Some(item) = result.attributes {
            let counter: DynamoCounter = from_item(item).unwrap();

            Ok(counter.count)
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAppRequest {
    pub id_app: u16,
}
