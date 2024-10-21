use super::{
    AddAppError, App, CreateAppError, DeleteAppError, GetAppError, ListAppsError, UpdateAppError,
};
use async_trait::async_trait;
use aws_sdk_dynamodb::error::DisplayErrorContext;
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::types::{AttributeValue, ReturnValue};
use serde::{Deserialize, Serialize};
use serde_dynamo::aws_sdk_dynamodb_1::{from_item, from_items, to_item};
use tracing::{debug, info, instrument};

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

    #[instrument(skip(self))]
    pub async fn create_atomic_counter(&self) {
        let result = self
            .dynamo_client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S("atomic_counter".to_string()))
            .send()
            .await
            .unwrap();

        debug!("{:#?}", result);

        if result.item.is_none() {
            info!("Atomic counter was not found, creating...");

            let start_count = 1100;

            let new_counter = to_item(DynamoCounter {
                pk: "atomic_counter".to_string(),
                count: start_count,
            })
            .expect("Creation of atomic counter should not fail");

            self.dynamo_client
                .put_item()
                .table_name(&self.table_name)
                .set_item(Some(new_counter))
                .send()
                .await
                .expect("Unable to create atomic counter");
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDynamoItem {
    pk: String,
    entity_type: String,

    #[serde(flatten)]
    app: App,
}

impl AppDynamoItem {
    #[instrument]
    pub fn new(app: &App) -> Self {
        Self {
            pk: app.id.to_string(),
            entity_type: "app".to_string(),
            app: app.clone(),
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
    #[instrument(skip(self))]
    async fn get_app(&self, id: u16) -> Result<App, GetAppError> {
        let result = self
            .dynamo_client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(id.to_string()))
            .send()
            .await
            .map_err(|e| {
                let err = e.into_service_error();

                match err {
                    GetItemError::ResourceNotFoundException(_) => GetAppError::ResourceNotFound(id),
                    _ => {
                        tracing::error!("DynamoDB SDK Error: {}", DisplayErrorContext(&err));
                        GetAppError::UnexpectedError
                    }
                }
            })?;

        if let Some(item) = result.item {
            let app: App = from_item(item)?;

            Ok(app)
        } else {
            Err(GetAppError::ResourceNotFound(id))
        }
    }

    #[instrument(skip(self))]
    /// add_app is intended for adding "pre-existing" applications defined by the service
    async fn add_app(&self, app: App) -> Result<App, AddAppError> {
        let item = to_item(AppDynamoItem::new(&app))?;

        info!("{:?}", item);

        let _result = self
            .dynamo_client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(pk)")
            .send()
            .await
            .map_err(|e| {
                let err = e.into_service_error();

                match err {
                    PutItemError::ConditionalCheckFailedException(_) => {
                        AddAppError::ResourceAlreadyExists {
                            name: app.name.clone(),
                        }
                    }
                    _ => {
                        tracing::error!("DynamoDB SDK Error: {}", DisplayErrorContext(&err));
                        AddAppError::UnexpectedError
                    }
                }
            })?;

        Ok(app)
    }

    #[instrument(skip(self))]
    async fn create_app(&self, app: App) -> Result<App, CreateAppError> {
        let item = to_item(AppDynamoItem::new(&app))?;

        info!("{:?}", item);

        let _result = self
            .dynamo_client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(pk)")
            .send()
            .await
            .map_err(|e| {
                let err = e.into_service_error();

                match err {
                    PutItemError::ConditionalCheckFailedException(_) => {
                        CreateAppError::ResourceAlreadyExists {
                            name: app.name.clone(),
                        }
                    }
                    _ => {
                        tracing::error!("DynamoDB SDK Error: {}", DisplayErrorContext(&err));
                        CreateAppError::UnexpectedError
                    }
                }
            })?;

        Ok(app)
    }

    #[instrument(skip(self))]
    async fn delete_app(&self, id: u16) -> Result<(), DeleteAppError> {
        let _result = self
            .dynamo_client
            .delete_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(id.to_string()))
            .condition_expression("attribute_exists(pk)")
            .send()
            .await
            .map_err(|e| {
                let err = e.into_service_error();

                match err {
                    DeleteItemError::ResourceNotFoundException(_) => {
                        DeleteAppError::ResourceNotFound(id)
                    }
                    _ => {
                        tracing::error!("DynamoDB SDK Error: {}", DisplayErrorContext(&err));
                        DeleteAppError::UnexpectedError
                    }
                }
            })?;

        Ok(())
    }

    #[instrument(skip(self))]
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
                tracing::error!("DynamoDB SDK Error: {}", DisplayErrorContext(e));
                ListAppsError::UnexpectedError
            })?;

        if let Some(items) = result.items {
            let apps: Vec<App> = from_items(items)?;

            Ok(apps)
        } else {
            Ok(Vec::new())
        }
    }

    #[instrument(skip(self))]
    async fn update_app(&self, app: App) -> Result<App, UpdateAppError> {
        let item = to_item(AppDynamoItem::new(&app))?;

        let _result = self
            .dynamo_client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| {
                let err = e.into_service_error();

                match err {
                    _ => {
                        tracing::error!("DynamoDB SDK Error: {}", DisplayErrorContext(&err));
                        UpdateAppError::UnexpectedError
                    }
                }
            })?;

        Ok(app)
    }

    /// Atomic counter will be set to a number that should be above the staticly configured
    /// apps in the apps_service.rs. This is a manually specified number, and is not intended
    /// to be able to handle any situations beyond that. If you are shenaniganizing, this can
    /// blow up, and you deserve it
    #[instrument(skip(self))]
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
            .expect("Should not error because atomic counter is checked on startup");

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
