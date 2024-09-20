use aggregates_common_lib::current_state::get_aggregate_state;
use lambda_runtime::tracing;
use lambda_runtime::Error;
use order_core::entities::Order;
use order_core::entities::OrderStatus;
use order_core::event_types::OrderCreated;
use aggregates_common_lib::appsync_event_structure::*;
use super::commands::*;
use super::commands_business_rules::*;
use chrono::Utc; 
use aggregates_common_lib::get_dynamodb_client;
use aggregates_common_lib::event_emitter::emit_event;


static TABLE_NAME: &str = "order-eventstore";

/*  Aqui van las funciones encargadas de procesar cada comando porveniente de appsync */

pub async fn create_order(event: &AppsyncEvent) -> Result<AppsyncObjectResponse, Error> {
    let command_from_appsync= event.arguments.operation.clone();
    let create_order_command: EnabledCommand = EnabledCommand::CreateOrder(serde_json::from_value(command_from_appsync)?);
    let current_state: Order = Order::default();
    match ejecutar_reglas(&create_order_command, &current_state).await {
        Ok(_) => tracing::info!("El comando será procesado ya que todas las reglas de negocio fueron ejecutadas correctamente."),
        Err(e) => {
            tracing::error!("El comando NO será procesado. Error al ejecutar las reglas de negocio: {:?}", e);
            return Err(e);
        }
    }
    match create_order_command {
        EnabledCommand::CreateOrder(create_order) => {
            let order_created: OrderCreated = OrderCreated {
                event_number: 1,
                event_name: "OrderCreated".to_string(),
                id: create_order.id.clone(),
                user_id: create_order.user_id.clone(),
                product_id: create_order.product_id.clone(),
                quantity: create_order.quantity,
                created_at: Utc::now().to_rfc3339(), 
                order_status: OrderStatus::Created,
            }; 
            let json_event = serde_json::to_string(&order_created)?;
            let event_name = order_created.event_name;
            let source_origin = "order-aggregate";
            let eventbus_name = "default";
        
            emit_event(&json_event, &event_name, &source_origin, &eventbus_name).await?;
    
            let order = Order {
                id: create_order.id,
                event_number: 1,
                user_id: create_order.user_id,
                product_id: create_order.product_id,
                quantity: create_order.quantity,
                order_status: OrderStatus::Created,
                created_at: order_created.created_at,
            };
            Ok(AppsyncObjectResponse::Order(order))
        },
        _ => {
            tracing::error!("El comando que se intenta procesar no es del tipo CreateOrder");
            Err(Error::from("El comando que se intenta procesar no es del tipo CreateOrder"))
        }
    }
}

 
pub async fn change_order_status(event: &AppsyncEvent) -> Result<AppsyncObjectResponse, Error> {
    let command_from_appsync= event.arguments.operation.clone();
    tracing::info!("command_from_appsync: {:?}", command_from_appsync); 
    let change_order_status_command: EnabledCommand = EnabledCommand::ChangeOrderStatus(serde_json::from_value(command_from_appsync)?);
    match change_order_status_command {
        ref matched_command @ EnabledCommand::ChangeOrderStatus(ref change_order_status) => {
            tracing::info!("change_order_status: {:?}", change_order_status);
            let dynamodb_client = get_dynamodb_client().await;
            let current_state = match get_aggregate_state(&dynamodb_client, &TABLE_NAME, &change_order_status.id).await {
                Ok(state) => {
                    tracing::info!("Estado actual del agregado: {:?}", state);
                    let order: Order = serde_json::from_value(state)?;
                    tracing::info!("order: {:?}", order);
                    order
                },
                Err(e) => {
                    tracing::error!("Error al obtener el estado actual del agregado: {:?}", e);
                    return Err(e);
                }
            };
            if change_order_status.last_event_number != current_state.event_number {
                tracing::info!("El comando ChangeOrderStatus no será procesado. El número de evento no coincide con el último evento registrado en el agregado.");
                return Err(Error::from("El comando ChangeOrderStatus no será procesado. El número de evento no coincide con el último evento registrado en el agregado."));
            }
            match ejecutar_reglas(&matched_command, &current_state).await {
                Ok(_) => tracing::info!("El comando será procesado ya que todas las reglas de negocio fueron ejecutadas correctamente."),
                Err(e) => {
                    tracing::error!("El comando NO será procesado. Error al ejecutar las reglas de negocio: {:?}", e);
                    return Err(e);
                }
            }
            let order_status = change_order_status.order_status;
            let json_event = serde_json::to_string(&change_order_status)?;
            let event_name = "OrderStatusChanged";
            let source_origin = "order_aggregate";
            let eventbus_name = "default";

            emit_event(&json_event, &event_name, &source_origin, &eventbus_name).await?;  

            let order = Order {
                id: change_order_status.id.clone(),
                event_number: current_state.event_number + 1,
                user_id: current_state.user_id,
                product_id: current_state.product_id,
                quantity: current_state.quantity,
                order_status: order_status,
                created_at: current_state.created_at,
            };
            return Ok(AppsyncObjectResponse::Order(order))
        },
        _ => {
            tracing::error!("El comando que se intenta procesar no es del tipo ChangeOrderStatus");
            return Err(Error::from("El comando que se intenta procesar no es del tipo ChangeOrderStatus"));
        }
    }

}

 