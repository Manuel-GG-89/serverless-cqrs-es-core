This is just a prof of concept in EARLY... early state. 
and, yes.. for the time being, fully coupled to the AWS platform.

nota -1: Domain Driven Design...
  
note 0: This project is intended only as a System Core. Could it be used for the core of a web application? Yes, but not in the same way you would usually expect. There is no mutation, no CRUD, i

note 1: The first sketches and proofs of concepts were developed in Typescript/Python... now rewriting same things in Rust (just the functions) 

note 2: Front-end servers aren´t considered on this project yet.. 

note 3: The next proyect: To decopule from AWS, try the same goal with other lowcost cloud providers like Cloudflare or any other that a third world team can finance..


Objetive: 
  - To achive the most simple and lowcost way to mount a Stateless, Event Driven, Event Sourced CQRS core service on AWS
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

  
  - If a BUSINESS RULE VALIDATION ERROR ocurred, the aggregate endpoint (command-handlre) responds with the bussiness rule error (or an eventual consistency error .. its a hard topic to explain here, so watch  the youtube video linked at buttom).

  
  - But, if the command handler had no bussiness validations errors, JUST IN THAT CASE, it will respond whit an ID of the reservation created by the command handler. But this ID cannot be considered automatically materialized by the front-end, since this only validates that the command handler had no bussiness errors and issued (propagate) the 'createReservation' via the event bus, but there could be technical errors at the time of internal distribution and saving of this event, so we cannot assume that this event is already part of the system.

  
  - So the front-end should take a tentative stance: “Ok, I’m going to assume that the event validation is already Ok because command-handler give and ID. I’ll associat these ID  whith the command's parameters sended, showing the user the order as "Accepted, whaiting final confirmation"  (using the same parameters that the emited command has), . Internally, I’ll subscribe (via the grapql api) to the (query handler) query 'getReservationById': { reservationID: “ID generated by the command handler”} and I’ll only change the reservation status to ‘Reservation Ready’ on the user UI when I receive a success message through that subscription, or an error message if it responds with an error or timeout (in this case, the least complex option is to configure a timeout internally and externally (api) , so if the event issued by the command is not processed (not saved) in X amount of time, the aggregate invalidates the event internally and externally).”

    
  - Eventually, the aggregate EVENT HANDLER will receive the event emitted by the command handler, store the event in the even store and emit a success or error response to the graphql mutation 'reservationSaved{ reservationID:adfgadf}'. This mutation are binded via subscription (grapql subsription) to the getReservationById query by a subcription, so that subcription will be triggered, the getReservationById query JUST WHEN THE ID matches with the reservation ID generated by the command handler, unless an intenral (techical) error ocurred, . With a well designed grapql scheme , it will be able to inform errors too based on the same trigger.
    

  
So.. why Graphq and not an http api with webhooks?
  - Easy implementation of a sub-pub communication model.
  - Default asynchronous communication
  - Websockets under the hood, which provides more security, speed, and reduces the number of connections and redundant requests.
  - If webhooks were to handle asynchronous communication, the client (server client) would need to repeatedly query the webhook until a result is obtained (redundancy, resource wastage) and would also need to consider the configuration of parameters such as the frequency for querying the webhooks and the corresponding timeouts.
  - ...So far, this is the best option I’ve found to achieve that result, but there might be better ones. So, before implementing the communication interfaces, I’ll need to research again, I think.

Basic composition of a Domain Aggregate have, at least: 

  - A Command Handler with a Rules/Policies validator 
  - An Event Handler that just save (in a event store) the  emited events from (self and/or external) Commands Handlers
  - An Event Store that save the events and notify this outside via an async/pub-sub API
  - A Query handler that make posible do query to the Event Store (Dynamodb) and get states from one o more aggregate instances.
  - A simple reducer function shared as a lib between Command and Query handlers to fold(reduce) and rebuild the actual state of the agregate reading the (historical) events form the event-store.
  - A set of delivery rules (configured on the event bus) to indicate the destinations of each event.
  - An async api.

Pseudo Code:
```
  - enum CommandHandlerResponse<T, E> {
      Success(T),
      Error(E),
    }
  - enum QueryHandlerResponse<T, E> {
      Some(T),
      Error(E),
    }
  - type commandParams = [..inputParams, maybe(ID)]  

  - Client[ callApiAndWait( doSomethingCommand(commandParams)) >..waiting..> onCommandRespond(Success(ID)) >> queryAggerateByIdAndWait(ID) >..waiting..> ] 
                        ↓                                                                            ↑
  - >> Api[ doSomethingCommand(commandParams) >> implicitSendCommandAndWait(commandParams) && respondToWaitingClient(Success(ID))] 
                        ↓
  - >> Command_Handler[ validate_rules(commandParams) >> emmit_to_bus(newEventCreated) && responseWithID(newEventCreated) ] 
                        ↓
  - >> Evento_Bus[ delivers(newEventCreated) ] 
                        ↓
  - >> Event_Handler[ capture_and_store(newEventCreated) >> publish_to_api( onAggregateEventEmited(newEventCreated)) ] 
                        ↓
  - >> Api[ onAggregateEventEmited( passEventToApiQueryHandler(Some(newEventCreated)) ) ]
                        ↓
  - Client[ >..waiting..> queryAggerateByIdAndWait(Some(newEventCreated)) >>  sendToUserUI( Some(newEventCreated) ) && cachOrUpdateAggrProjectionOrAnyOtherSSRprocess(newEventCreated) ]
```


.. well, it seems like a long and slow proccess... 
.. meybe not :)



Theoretical framework:
  - https://www.google.com/search?q=domain+driven+design&oq=domain+driven+design - "Domain Driven Design" ( DDD )
  - https://www.youtube.com/watch?v=GzrZworHpIk  - Event Sourcing You are doing it wrong by David Schmitz""

Then: Domain Driven Desing..
  
  - In the Domain Driven Design philosophy (and, paradoxically, in almost any software engineering project) the most important phase, in which more time should be invested, is the initial phase: Design and specifications.
    
  - Unless the project is being developed by an I+D team,or other one with developers without mastery of the technologies that are intended to be used, or by a team that, due to pressure from ambitious stakeholders and management who ignore how disastrous and expensive a project can become if it is started to be developed on the fly and without having a clear understanding of what the behaviors and flows are in order to design based on them, software engineers, front-end/back-end developers and analysts, together with the users of the future system (if there are any) must focus on understanding and obtaining as much feedback as possible in order to describe each interaction and flow of the system, possible errors, alternative flows, automated or periodic periodic tasks, edge cases, etc.
 
  - For each User Story (interaction with the system, called 'use cases' in other types of systems) the behavior, business rules, information flow, intervention of other users or systems in said flow must be identified.
 
  - Before developing any line of code, the behavior that the software should have for each case must be identified. The greatest amount of feedback from the users who will use the system (which are a fundamental part of this phase) must be taken into account, or, in the case of a new system, a consistent idea of ​​what is expected to be achieved, in order to define the specifications mentioned above.
 
  - Only when all the 'user stories' and system flows have been covered, should one move on to the development phase. And the above takes time. Normally it would take approximately 70% of the time (and cost) of the project. (sadly... this is not usually the case, at least not in the country I live in).
 
  - In DDD, the most important thing is the understanding, feedback and design of the stories that will be produced within the system. DDD is the design framework for a system based on its behavior, NOT based on the data models (entities) of the domain.
  
  - But the technical implementation of a DDD system is challenging, it has complexities that can make architects decide to use expensive frameworks and infrastructures that try to handle these complexities, or simply discard it and opt for the traditional (data model-based design) that, in the long run, turn out to be monolithic, difficult to scale, andconsiderably more expensive if it is explicitly required to obtain the features that a DDD system can provide by default... (A more detailed description of this last point will be left below).
 
  - It is this last point that I try to address in this project.


..So, what offer a sistem designed like this that really pay the effort???
  
  - Traceability and Consistency: The most important point in terms of the value it can bring to the business. ..Why? 

   - Event Sourced: Allows reconstructing the system state from a sequence of events, providing greater traceability by default. You do not need to generate any kind of procedure by consulting logs obtained from a relational database (if there were enough of them) and try to reconstructfrom those the  historical operational traces of key system entities when it becomes necessary to generate statistical analyses and projections based on those traces (information that can be really valuable for the business). (... I mean, user behavior in social networks, online stores and marketplaces, financial systems, banking, IoT, logistics... practically everything.
    
  - CQRS: Separates read and write operations, optimizing data performance and consistency.

  - Decoupling: Separates the domain logic from the infrastructure, facilitating changes in the business without affecting the technological infrastructure.
    
  - Scalability and Resilience:
  
  - Stateless: Reduces the load on the server by not storing state between requests, allowing for greater scalability.
      
  - Event-Driven: Improves responsiveness and resilience by processing events avsynchronously.

  - Maintainability: Promotes a more modular and maintainable architecture, where changes in one area of the system have a reduced impact on other areas.


In comparison, traditional data model-based systems are super easy develop and implemet but, as they grow in terms of components, entities and volume of data, tend to be more monolithic, less flexible and can have difficulty scaling and adapting to rapid changes in the business. 









This is project build with AWS CDK TypeScript.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `npx cdk deploy`  deploy this stack to your default AWS account/region
* `npx cdk diff`    compare deployed stack with current state
* `npx cdk synth`   emits the synthesized CloudFormation template
