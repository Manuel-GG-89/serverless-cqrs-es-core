use lambda_runtime::{run, service_fn, tracing, Error};
mod commands_processor;
mod commands_business_rules;
mod commands;
mod command_handler;
use command_handler::command_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(command_handler)).await
}