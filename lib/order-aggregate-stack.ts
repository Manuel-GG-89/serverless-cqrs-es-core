import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as path from 'path';
import * as events from 'aws-cdk-lib/aws-events';
import * as iam from 'aws-cdk-lib/aws-iam';
import * as targets from 'aws-cdk-lib/aws-events-targets';
import { AttributeType, ProjectionType } from 'aws-cdk-lib/aws-dynamodb';
import { RustFunction } from 'cargo-lambda-cdk';

export class OrderAggregateStack extends cdk.Stack {
  constructor(scope: Construct, id: string) {
    super(scope, id);

    /* UNDER CONSTRUCTION */

    /**
     * DEFINICION Y CONFIGURACION DE RECURSOS 
    */

    // Funcion para manejar los comandos del agregado
    const commandHandlerFunction = new RustFunction(this, 'order-command-handler', {
      manifestPath: path.join(__dirname, '..', 'aggregates', 'handlers', 'order', 'command_handler'),
      functionName: 'order-command-handler',
    });


    // Funcion para manejar los eventos del agregado
    const eventHandlerFunction = new RustFunction(this, 'order_event_handler', {
      manifestPath: path.join(__dirname, '..', 'aggregates', 'handlers', 'order', 'event_handler'),
      functionName: 'order-event-handler',
    });

    // Obtiene el bus de eventos por defecto de EventBridge
    const defaultEventBus = events.EventBus.fromEventBusName(this, 'DefaultEventBus', 'default');

    // Permite que commandHandlerFunction pueda enviar eventos hacia el bus por defecto
    commandHandlerFunction.addToRolePolicy(new iam.PolicyStatement({
      actions: ['events:PutEvents'],
      resources: [defaultEventBus.eventBusArn],
    }));

    // crea una tabla DynamoDb para almacenar los eventos
    const eventStoreTableName = "order-eventstore";
    const eventStoreTable = new cdk.aws_dynamodb.Table(this, eventStoreTableName, {
      tableName: eventStoreTableName,
      partitionKey: { name: 'id', type: cdk.aws_dynamodb.AttributeType.STRING },
      sortKey: { name: 'serialNumber', type: cdk.aws_dynamodb.AttributeType.NUMBER },
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    // Agrega un indice global secundario para buscar eventos por el campo eventName
    eventStoreTable.addGlobalSecondaryIndex({
      indexName: 'EventNameIndex',
      partitionKey: { name: 'eventName', type: AttributeType.STRING },
      sortKey: { name: 'id', type: AttributeType.STRING },
      projectionType: ProjectionType.KEYS_ONLY,
    });

    // Permitir que event handler function pueda insertar items en order-eventstore
    eventStoreTable.grantWriteData(eventHandlerFunction);

    // Permitir que command handler function pueda leer items en order-eventstore
    eventStoreTable.grantReadData(commandHandlerFunction);

    // Permitir que query handler function pueda leer items en order-eventstore
    const eventBusDeliveryRules = require(path.join(__dirname, '/eventbus-delivery-rules.ts')).ORDER_DELIVERY_RULES;

    // Crear una regla en EventBridge para el agregado de dominio y enviar todos los eventos a la función eventHandlerFunction
    const deliveryRuleName = 'Send-all-to-self-event-handler';
    const rule = new events.Rule(this, deliveryRuleName, {
      ruleName: deliveryRuleName,
      eventPattern: eventBusDeliveryRules,
      targets: [new targets.LambdaFunction(eventHandlerFunction)],
      enabled: true,
      eventBus: defaultEventBus,
    });
  } // end constructor

} // end Stack class


