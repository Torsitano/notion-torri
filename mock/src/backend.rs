use std::sync::Arc;

use aws_config::{self, BehaviorVersion};
use aws_sdk_dynamodb;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    apps_service::{AppsService, AppsServiceTrait},
    repository::DynamoAppsRepository,
};

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

#[derive(Debug, Clone)]
pub struct Backend<AS>
where
    AS: AppsServiceTrait,
{
    pub apps_service: Arc<AS>,
}

#[instrument]
pub async fn setup() -> Backend<impl AppsServiceTrait> {
    let settings = Settings {
        environment: "local".to_string(),
        table: TableSettings {
            endpoint_url: Some("http://127.0.0.1:8001".to_string()),
            table_name: "torii-table".to_string(),
        },
    };

    let dynamo_client = get_dynamo_client(&settings).await;
    let apps_repo = DynamoAppsRepository::new(dynamo_client, settings.table.table_name.clone());
    let apps_service = AppsService::new(apps_repo);

    let app_state = Backend {
        apps_service: Arc::new(apps_service),
    };

    app_state
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
