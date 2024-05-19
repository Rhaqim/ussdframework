use std::future::Future;

use actix_web::{HttpResponse, Responder};
use serde::Deserialize;

use super::{DatabaseManager, ScreenModel, ServiceModel};

pub mod file;
pub mod screens;
pub mod services;

async fn with_database<F, Fut>(operation: F) -> impl Responder
where
    F: FnOnce(&mut DatabaseManager) -> Fut,
    Fut: Future<Output = HttpResponse>,
{
    let mut manager = DatabaseManager::new();
    operation(&mut manager).await
}

#[derive(Deserialize)]
pub struct PathInfo {
    id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceModelUpdate {
    pub id: i32,
    pub service: ServiceModel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScreeModelUpdate {
    pub id: i32,
    pub screen: ScreenModel,
}
