use crate::Service;
use std::sync::Arc;
use warp::Filter;

#[derive(Debug)]
pub struct AppState {
    pub service: Service,
}

impl AppState {
    pub fn new(service: Service) -> AppState {
        AppState { service }
    }
}

pub fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
