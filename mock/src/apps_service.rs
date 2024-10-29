use std::collections::HashMap;

use crate::{
    repository::{
        AddAppError, App, AppCategory, AppState, AppsRepository, CreateAppError, DeleteAppError,
        GetAppError, ListAppsError, UpdateAppError,
    },
    routes::{
        AddAppHttpRequestBody, CreateAppHttpRequestBody, SearchAppsQueryParams,
        UpdateAppHttpRequestBody,
    },
};
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct KnownApp {
    id: u16,
    name: String,
    category: AppCategory,
    url: String,
}

#[async_trait]
pub trait AppsServiceTrait: std::fmt::Debug + Send + Sync + Clone {
    async fn get_app(&self, id: u16) -> Result<App, GetAppError>;
    async fn add_app(&self, request: AddAppHttpRequestBody) -> Result<App, AddAppError>;
    async fn create_app(&self, request: CreateAppHttpRequestBody) -> Result<App, CreateAppError>;
    async fn delete_app(&self, id: u16) -> Result<(), DeleteAppError>;
    async fn list_apps(&self) -> Result<Vec<App>, ListAppsError>;
    async fn list_known_apps(&self) -> Vec<KnownApp>;
    async fn update_app(
        &self,
        request: UpdateAppHttpRequestBody,
        id: u16,
    ) -> Result<App, UpdateAppError>;
    async fn search_apps(&self, params: SearchAppsQueryParams) -> Result<Vec<App>, ListAppsError>;
}

#[derive(Debug, Clone)]
pub struct AppsService<R>
where
    R: AppsRepository,
{
    pub repo: R,
}

impl<R> AppsService<R>
where
    R: AppsRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> AppsServiceTrait for AppsService<R>
where
    R: AppsRepository,
{
    #[tracing::instrument(skip(self))]
    async fn get_app(&self, id: u16) -> Result<App, GetAppError> {
        self.repo.get_app(id).await
    }

    #[tracing::instrument(skip(self))]
    async fn add_app(&self, body: AddAppHttpRequestBody) -> Result<App, AddAppError> {
        if let Some(app) = get_default_app(body.id_app) {
            self.repo.add_app(app).await
        } else {
            Err(AddAppError::ResourceNotFound(body.id_app))
        }
    }

    #[tracing::instrument(skip(self))]
    async fn create_app(&self, request: CreateAppHttpRequestBody) -> Result<App, CreateAppError> {
        if let Ok(_) = self.repo.get_app_by_name(&request.name).await {
            return Err(CreateAppError::ResourceAlreadyExists { name: request.name });
        }

        let id = self
            .repo
            .get_id()
            .await
            .map_err(|_| CreateAppError::UnexpectedError)?;

        let app = App {
            id,
            name: request.name,
            state: request.state,
            url: request.url,
            category: request.category,
            tags: request.tags,
            is_custom: true,
            ..Default::default()
        };

        self.repo.create_app(app).await
    }

    #[tracing::instrument(skip(self))]
    async fn delete_app(&self, id: u16) -> Result<(), DeleteAppError> {
        self.repo.delete_app(id).await
    }

    #[tracing::instrument(skip(self))]
    async fn list_apps(&self) -> Result<Vec<App>, ListAppsError> {
        self.repo.list_apps().await
    }

    #[tracing::instrument(skip(self))]
    async fn list_known_apps(&self) -> Vec<KnownApp> {
        let known_apps: Vec<KnownApp> = default_apps()
            .values()
            .map(|item| KnownApp {
                id: item.id,
                category: item.category.clone(),
                name: item.name.clone(),
                url: item.url.clone(),
            })
            .collect();

        known_apps
    }

    #[tracing::instrument(skip(self))]
    async fn update_app(
        &self,
        request: UpdateAppHttpRequestBody,
        id: u16,
    ) -> Result<App, UpdateAppError> {
        let mut app = self.get_app(id).await.map_err(|e| match e {
            GetAppError::ResourceNotFound(id) => UpdateAppError::ResourceNotFound(id),
            _ => {
                error!("{}", e);
                UpdateAppError::UnexpectedError
            }
        })?;

        if let Some(name) = request.name {
            app.name = name
        };

        if let Some(state) = request.state {
            app.state = state
        };

        if let Some(url) = request.url {
            app.url = url
        };

        if let Some(category) = request.category {
            app.category = category
        };

        app.description = request.description;
        app.tags = request.tags;
        app.last_updated_at = Utc::now();

        self.repo.update_app(app).await
    }

    #[tracing::instrument(skip(self))]
    async fn search_apps(&self, params: SearchAppsQueryParams) -> Result<Vec<App>, ListAppsError> {
        let apps = self.repo.list_apps().await?;

        let found_apps = apps
            .into_iter()
            .filter(|app| app.name.contains(&params.query))
            .collect();

        Ok(found_apps)
    }
}

/// Pre-defined apps that can be added
#[tracing::instrument]
fn get_default_app(id: u16) -> Option<App> {
    let default_apps = default_apps();

    default_apps.get(&id).cloned()
}

fn default_apps() -> HashMap<u16, App> {
    let mut default_apps: HashMap<u16, App> = HashMap::new();

    default_apps.insert(
        1000,
        App {
            id: 1000,
            name: "Salesforce".to_string(),
            category: AppCategory::SalesAndMarketing,
            url: "salesforce.com".to_string(),
            state: AppState::Sanctioned,
            ..Default::default()
        },
    );

    default_apps.insert(
        1001,
        App {
            id: 1001,
            name: "Zoom".to_string(),
            category: AppCategory::Productivity,
            url: "zoom.com".to_string(),
            state: AppState::Sanctioned,
            ..Default::default()
        },
    );

    default_apps.insert(
        1002,
        App {
            id: 1002,
            name: "GitHub".to_string(),
            category: AppCategory::DeveloperTools,
            url: "github.com".to_string(),
            state: AppState::Sanctioned,
            ..Default::default()
        },
    );

    default_apps.insert(
        1003,
        App {
            id: 1003,
            name: "Cats".to_string(),
            category: AppCategory::Other,
            url: "cats.com".to_string(),
            state: AppState::Sanctioned,
            ..Default::default()
        },
    );

    default_apps
}
