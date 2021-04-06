use actix_web::web::Json;
use common::ClientCommand;

use crate::api;

pub async fn commands(_input: Json<ClientCommand>) -> Result<api::Response<bool>, crate::Error> {
    Ok(api::Response::ok(true))
}
