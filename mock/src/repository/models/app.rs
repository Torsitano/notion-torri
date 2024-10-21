use chrono::{prelude::*, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, strum::Display, Clone, ToSchema)]
pub enum AppState {
    Discovered,
    Sanctioned,
    Closed,
}

#[derive(Debug, Serialize, Deserialize, strum::Display, Clone, ToSchema)]
pub enum AppCategory {
    Operations,
    #[strum(to_string = "Sales & Marketing")]
    SalesAndMarketing,
    #[strum(to_string = "Developer Tools")]
    DeveloperTools,
    Design,
    #[strum(to_string = "Project Management")]
    ProjectManagement,
    #[strum(to_string = "Customer Success")]
    CustomerSuccess,
    #[strum(to_string = "Human Resources")]
    HumanResources,
    #[strum(to_string = "IT & Security")]
    ItAndSecurity,
    Finance,
    Productivity,
    #[strum(to_string = "Analytics & BI")]
    AnalyticsAndBi,
    Other,
}

// List of all values available here:
// https://developers.toriihq.com/reference/getappsidapp
// https://developers.toriihq.com/reference/getapps

// None of the other API docs indicate the correct list of items
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct App {
    pub id: u16,
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    pub name: String,
    pub state: AppState,
    pub url: String,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    pub category: AppCategory,
    pub users: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    #[serde(rename = "creationTime")]
    pub creation_time: DateTime<Utc>,
    #[serde(rename = "lastUsageTime")]
    pub last_usage_time: Option<DateTime<Utc>>,
    #[serde(rename = "addedBy")]
    pub added_by: String,
    #[serde(rename = "primaryOwner")]
    pub primary_owner: String,
    #[serde(rename = "isCustom")]
    pub is_custom: bool,
    pub sources: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        // Will blow up if there's a conflict, just here for default because this isn't a real service
        let random_id = rand::thread_rng().gen();

        Self {
            id: random_id,
            is_hidden: false,
            name: format!("{random_id}-app"),
            added_by: "Default".to_string(),
            category: AppCategory::Other,
            creation_time: Utc::now(),
            description: None,
            image_url: None,
            is_custom: false,
            last_usage_time: None,
            primary_owner: "N/A".to_string(),
            sources: None,
            state: AppState::Discovered,
            tags: None,
            url: format!("default-{random_id}.com"),
            users: None,
        }
    }
}
