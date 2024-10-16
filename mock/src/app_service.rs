use crate::repository::{App, AppsRepository};

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
