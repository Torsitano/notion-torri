use aws_config::{self, BehaviorVersion};
use aws_sdk_dynamodb;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    app_service::AppsService,
    repository::{AppsRepository, DynamoAppsRepository},
};

#[derive(Debug, Clone)]
pub struct Backend<R>
where
    R: AppsRepository,
{
    apps_service: AppsService<R>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TableSettings {
    pub table_name: String,
    pub endpoint_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub environment: String,
    pub table: TableSettings,
}

impl<R> Backend<R>
where
    R: AppsRepository + std::fmt::Debug,
{
    #[instrument]
    pub async fn new(apps_repo: R, settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            apps_service: AppsService { repo: apps_repo },
        })
    }
}

#[instrument]
pub async fn setup() -> Backend<DynamoAppsRepository> {
    let settings = Settings {
        environment: "dev".to_string(),
        table: TableSettings {
            endpoint_url: None,
            table_name: "test".to_string(),
        },
    };

    let dynamo_client = get_dynamo_client(&settings).await;

    let apps_repo = DynamoAppsRepository::new(dynamo_client, settings.table.table_name.clone());

    Backend::new(apps_repo, &settings).await.unwrap()
}

#[instrument]
async fn get_dynamo_client(settings: &Settings) -> aws_sdk_dynamodb::Client {
    let region_provider =
        aws_config::meta::region::RegionProviderChain::default_provider().or_else("us-east-1");

    if let Some(endpoint_url) = settings.table.endpoint_url.to_owned() {
        let client_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .endpoint_url(endpoint_url)
            .load()
            .await;

        aws_sdk_dynamodb::Client::new(&client_config)
    } else {
        let client_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        aws_sdk_dynamodb::Client::new(&client_config)
    }
}
