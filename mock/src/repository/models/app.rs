use chrono::{prelude::*, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, strum::Display, Clone)]
pub enum AppState {
    Discovered,
    Sanctioned,
    Closed,
}

#[derive(Debug, Serialize, Deserialize, strum::Display, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct App {
    id: Uuid,
    #[serde(rename = "isHidden")]
    is_hidden: bool,
    name: String,
    state: AppState,
    url: String,
    category: AppCategory,
    description: Option<String>,
    tags: Option<String>,
    #[serde(rename = "creationTime")]
    creation_time: DateTime<Utc>,
    #[serde(rename = "primaryOwner")]
    primary_owner: String,
    #[serde(rename = "isCustom")]
    is_custom: bool,
}
