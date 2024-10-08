use lambda_runtime::{tracing, Error};
use order_core::entities::Order;
use super::commands::EnabledCommand;

pub async fn ejecutar_reglas(command: &EnabledCommand, current_state: &Order) -> Result<(), Error> {
    match command {
        EnabledCommand::CreateOrder(_) => {
            regla_negocio_1(command, current_state).await?;
            
            regla_negocio_2(command, current_state).await?;
        },
        EnabledCommand::ChangeOrderStatus(_) => {
            regla_negocio_1(command, current_state).await?;
            regla_negocio_2(command, current_state).await?;
            regla_negocio_3(command, current_state).await?;
        }
    }
    Ok(())
}



async fn regla_negocio_1(command: &EnabledCommand, current_state: &Order) -> Result<(), Error> {
    match command {
        EnabledCommand::CreateOrder(command) => tracing::info!("Llamando a async_fn1 con {} y {}", command, current_state),
        EnabledCommand::ChangeOrderStatus(command) => tracing::info!("Llamando a regla_negocio_1 con {} y {}", command, current_state),
       }
    Ok(())
}

async fn regla_negocio_2(command: &EnabledCommand, current_state: &Order) -> Result<(), Error> {
    match command {
        EnabledCommand::CreateOrder(command) => tracing::info!("Llamando a async_fn2 con {} y {}", command, current_state),
        EnabledCommand::ChangeOrderStatus(command) => tracing::info!("Llamando a regla_negocio_2 con {} y {}", command, current_state),
       }
    Ok(())
}

async fn regla_negocio_3(command: &EnabledCommand, current_state: &Order) -> Result<(), Error> {
    match command {
        EnabledCommand::ChangeOrderStatus(command) => tracing::info!("Llamando a regla_negocio_3 con {} y {}", command, current_state),
        _ => ()
       }
    Ok(())
}