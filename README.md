This is just a prof of concept in EARLY... early state. 
and, yes.. for the time being, fully coupled to the AWS platform.

note -1: 
  - https://www.google.com/search?q=domain+driven+design&oq=domain+driven+design - "Domain Driven Design"
  - https://www.youtube.com/watch?v=GzrZworHpIk  - "Event Sourcing You are doing it wrong by David Schmitz"

note 0: This project is intended only as a core system. Could it be used for the core of a web application? Yes, but not in the same way you would usually expect. There is no mutation, no CRUD. 

note 1: The first sketches and proofs of concepts were developed in Typescript/Python... 
      ...now rewriting same things in Rust (just the functions) 

note 2: Front-end servers aren´t considered on this project yet.. (But some serverless Node framework hosted on a lowcost Cloud provider will be considered in the future).

note 3: The next proyect: To decopule from AWS, try the same goal with other lowcost cloud providers like Cloudflare or any other that a third world team can finance..


Objetive: 
  - To achive the most simple and lowcost way to mount a Serverless, Event Driven, Event Sourced CQRS core service on AWS
  - Done by making three / four common (almost generics) decopuled Domain Aggregates that cover most common cases of interactions
  - Without using frameworks, just AWS CDK.
  - No CDK meta programing or templating magic.. Aggregate1 -> aggregate1stack.ts, Aggregate2 -> Copy/Paste and Modify ggregate1stack.ts, and so.
  - Main goal is build a starting point of an architecture that allows to implement Domain Driven Design in an a little easier way.

Challenge stuffs (outside the obvious things):
  - Keep it simple.
  - Security layers / Authentication / Authorization.
  - Outside async comunication via websockets (Graphql, Appsync on AWS) (to front-end servers / functions. Never directly to client sides apps)
  - To reduce Coltdown Start times to the minimum possible... (currently using simplest rust functions..  minimal coldown start time)

Resources, tools and technical stuffs:
  - AWS CDK to declare and configure all resources (default typescript cdk version)
  - No statefull servers, just serverless functions
  - No Kafka or any other similar.
  - Internal communication through EventBridge data bus (not sqs/sns pub-sub pattern, delivery rules configured on the bus)
  - Step functions in case needs to handle transactions (saga pattern) (flow nor even designed yet ...)
  - No relational database. Just Dynamodb as an events store (historical events, no mutatios)
  - No http/rest apis, no webhooks. Graphql (Appsync) acting like pub-sub midleware to expose commands (async operations) and handle (simple) querys from front-end servers
  - Rust lambda runtimes to reduce Coltdown Start at minimal.
  - Shared Type schemmas between Domain Aggregates for Commands, Events and Querys when necesary at dev cycle and compilation/build time (easy on Rust and Typescript).


Why Graphql as API ?
  - External systems (SSR web servers, for example) must communicate to this system in terms of asynchronous commands, (explicit orders of what needs to be done in the system), and in terms of asynchronous queries to request updated information from the system. Let's see an example to understand the problem:
    
  - Suppose a customer needs to make a hotel reservation. The front-end service (usually an SSR server) needs to send the request to the core in the form of a Command, specifying what is wanted, along with the required parameters to the 'Reservation Domain Aggregate Endpoint'.Lets supose the command “createReservation { userId: asdf1234, roomID: adsf1234, dateStart: ‘some date’, dateEnd: ‘some other date’}.
  
  - The Reservation Aggregate  needs to validate parameters and business rules of the command "createReservation" before respond. Meanwhile, the SSR server needs to wait for the response of the Command(request) to show the user if the request could finally be processed and approved, or if a bussine rule validation (or a technical error) occurred.
  
  - The graphql aggregate endpoint sends to the aggregate Command-handler component the command (request), and it will not respond immediately since the request, before generating any change in the system, must be validated in terms of business rules, generate and distribute internally an event if the request passed all business rules, or respond immediately with an error if the business rules validation failed through the same request. This operation introduces two cases:

  - Case 1: In case of error of any business rules validation: The Api respond through the same request (command graphql) in the same way the case 1, but with the error specification. This operation is synchronous, i.e., the one who generated it should always wait for a response through the command (however, for complex cases and extensive validations, it could also be handled asynchronously but, to keep it simple, it is better to handle this operation synchronously).
    
  - Case 2. In case that the event passes the validation of all business rules, the requester will still receive a response with an ID generated by the command-handler , ID that will be used to request the updated status of the domain aggregate instance created. This is where the asynchronous part comes into play. Let's see why, detailing a little more the flow:
  
  - If a BUSINESS RULE VALIDATION ERROR ocurred, the aggregate endpoint (command-handlre) responds with the bussiness rule error (or an eventual consistency error .. is a hard topic to explain here, so watch  the youtube video linked at top).
  
  - But, if the command handler had no bussiness validations errors, JUST IN THAT CASE, it will respond whit an ID of the reservation created by the command handler. But this ID cannot be considered automatically materialized by the front-end, since this only validates that the command handler had no bussiness errors and issued (propagate) the 'createReservation' via the event bus, but there could be technical errors at the time of internal distribution and saving of this event, so we cannot assume that this event is already part of the system.
  
  - So the front-end should take a tentative stance: “Ok, I’m going to assume that the event validation is already Ok, I’ll associate the parameters sended in the command with the ID that the command handler responded with, then I’ll show the user the result of the reservation is emited, usin the same parameters that the emited command has, but in some way I'll advice that the request is aproved but being processed yet in the system. Internally, I’ll subscribe (via the grapql api) to the (query handler) query 'getReservationById': { reservationID: “ID generated by the command handler”} and I’ll only change the reservation status to ‘Reservation Ready’ on the user UI when I receive a success message through that subscription, or an error message if it responds with an error or timeout (in this case, the least complex option is to configure a timeout internally and externally (api) , so if the event issued by the command is not processed (not saved) in X amount of time, the aggregate invalidates the event internally and externally).”
    
  - Eventually, the aggregate EVENT HANDLER will receive the event emitted by the command handler, store the event in the even store and emit a success or error response to the graphql mutation 'reservationSaved{ reservationID:adfgadf}'. This mutation are binded via subscription (grapql subsription) to the getReservationById query by a subcription, so that subcription will be triggered the getReservationById query JUST WHEN THE ID matches with the reservation ID generated by the command handler, unless an intenral (techical) error ocurred, . With a well designed grapql scheme , it will be able to inform errors too based on the same trigger.

So.. why Graphq and not an http api with webhooks?
  - Easy implementation of a sub-pub communication model.
  - Default asynchronous communication
  - Websockets under the hood, which provides more security, speed, and reduces the number of connections and redundant requests.
  - If webhooks were to handle asynchronous communication, the client (server client) would need to repeatedly query the webhook until a result is obtained (redundancy, resource wastage) and would also need to consider the configuration of parameters such as the frequency for querying the webhooks and the corresponding timeouts.
  - ...So far, this is the best option I’ve found to achieve that result, but there might be better ones. So, before implementing the communication interfaces, I’ll need to research again, I think.

  Basic Composition. A Domain Aggregate have, at least: 

  - A Command Handler with a Rules/Policies validator 
  - An Event Handler that just save events emited from (self and/or external) Commands Handlers
  - An Event Store to store the events
  - A Query handler that make posible do query to the Event Store (Dynamodb) and get states from one o more aggregate instances.
  - A simple reducer function shared as a lib between Command and Query handlers to fold(reduce) and rebuild the actual state of the agregate reading the (historical) events form the event-store.
  - A set of delivery rules (configured on the event bus) to indicate the destinations of each event.
  - An async api.


This is project build with AWS CDK TypeScript.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `npx cdk deploy`  deploy this stack to your default AWS account/region
* `npx cdk diff`    compare deployed stack with current state
* `npx cdk synth`   emits the synthesized CloudFormation template
