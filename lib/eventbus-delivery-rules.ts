/* UNDER CONSTRUCTION */

export const ORDER_DELIVERY_RULES = {
    // Nomre del agregado de dominio
    source: ['order-aggregate'],
    //Nombres de eventos que se enviarán al event bus
    detailType: ['OrderCreated', 'OrderStatusChanged'],
}

export const RESERVATION_DELIVERY_RULES = {
    // Nomre del agregado de dominio
    source: ['reservation-aggregate'],
    //Nombres de eventos que se enviarán al event bus
    detailType: ['ReservationAcepted', 'ReservationStatusChanged'],
}