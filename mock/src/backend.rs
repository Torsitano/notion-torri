use std::{env, sync::Arc};

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
    pub auth_api_key: String,
}

#[instrument]
pub async fn setup() -> Backend<impl AppsServiceTrait> {
    let environment = env::var_os("APP_ENVIRONMENT")
        .unwrap_or_default()
        .into_string()
        .unwrap();

    let endpoint_url = if let Some(endpoint_url) = env::var_os("ENDPOINT_URL") {
        Some(endpoint_url.into_string().unwrap())
    } else {
        None
    };

    let api_key_secret = env::var_os("TORII_SECRET").unwrap().into_string().unwrap();

    let settings = Settings {
        environment,
        table: TableSettings {
            endpoint_url,
            table_name: "torii-table".to_string(),
        },
    };

    let dynamo_client = get_dynamo_client(&settings).await;
    let apps_repo = DynamoAppsRepository::new(dynamo_client, settings.table.table_name.clone());

    apps_repo.create_atomic_counter().await;

    let apps_service = AppsService::new(apps_repo);

    let app_state = Backend {
        apps_service: Arc::new(apps_service),
        auth_api_key: get_secret_value(&api_key_secret).await,
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

#[instrument]
pub async fn get_secret_value(secret_name: &str) -> String {
    let region_provider =
        aws_config::meta::region::RegionProviderChain::default_provider().or_else("us-east-1");

    let client_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let secrets_client = aws_sdk_secretsmanager::Client::new(&client_config);

    let resp = secrets_client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await
        .unwrap();

    resp.secret_string().unwrap().to_string()
}
