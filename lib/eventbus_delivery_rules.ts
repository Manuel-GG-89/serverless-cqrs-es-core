export const ORDER_DELIVERY_RULES = {
    // Nomre del agregado de dominio
    source: ['order-aggregate'],
    //Nombres de eventos que se enviarán al event bus
    detailType: ['OrderCreated', 'OrderStatusChanged'],
}