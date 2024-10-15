use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, strum::Display)]
pub enum AppState {
    Discovered,
    Sanctioned,
    Closed,
}

#[derive(Debug, Serialize, Deserialize, strum::Display)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    id: Uuid,
    #[serde(rename = "isHidden")]
    is_hidden: bool,
    name: String,
    state: AppState,
    url: String,
    category: AppCategory,
    description: String,
    tags: String,
}

pub trait AppsRepository {
    async fn get_app();
    async fn create_app();
    async fn delete_app();
    async fn list_apps();
    async fn update_app();
}
