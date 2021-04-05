use actix_web::web::Json;

pub async fn handle_command(
    input: Json<ClientCommand>,
) -> Result<api::Response<Success>, kernel::Error> {

    Ok(api::Response::ok(true.into()))
}
