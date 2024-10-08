This is only a proof of concept at a very early stage. A technical attempt to demystify and do something that has already been done for a long time behind the scenes by large tech companies to solve large scale problems in different industries (financial, logistics, marketing, video games, IoT, Robotics, AI, etc, etc.), but taken to terms that it is possible for me, as an average engineer with a very poor education and limited resources, to implement. It is a set of technical proposals and knowledge still under development that I have worked on for many years based on problems that I have faced in the corporate environment and I have had to solve by brute force... in companies that prefer to extend malformed projects for year... and waste lot of money trying to implement ineffective, inefficient, costly, based on patches and tricks, obsolete solutions...  and how this type of philosophy in software design can contribute immensely outside the context of large tech companies.  

well ..

This is about software design and architecture. This is about Domain Driven Design... and how to design systems that perceive, interpret, process, interact and remember changes in ourselves and the environment in the way our mind (currently) does... as a succession of immutable events over time that modify our own state and that of the world around us.

- note -1: Sorry for the spelling mistakes, English is not my mother speak tongue. For some things I used a translator, for others I wrote them as I went along, sorry for that. Also sorry if I expose the ideas and concepts in a scattered way ... in my defense, this is not a paper, it is just a memory aid for myself, that I hope someday, someone can share and implement.

- note 0: This project is intended to make a CORE SYSTEM ONLY.. and how it communicates internally and externally. Could it be used as Core of a web application? Yes, in fact, that is the goal... the thing is... not in the same way you would usually expect. There is no mutation, no CRUD, ..no database magic under de hood.

- note 1: The first sketches and proofs of concepts were developed in Typescript/Python... now rewriting same things in Rust (just the functions). Buy why Rust? only because I consider that, in this context (stateless, serverless web services), you get unsurpassed performance without using the language at low level, just using high-level api features, which is almost like developing in any other popular modern language (it's almost like writing typescript). There is no low level programming here, so there is no need to panic. Also, in essence, you can use whatever language you prefer or best suits each part of the system. It is totally valid that in this type of system, each domain aggregate ('micro-service' in old school terms) uses a different language, which best suits the needs of the domain aggregate.

- note 2: Front-end servers aren´t considered on this project yet.. but I will expose some ideas regarding the front-end and how this design be taken advantage of by fron-end system enginners in terms of asynchronous communication and caching by default. 

- note 3: For now, this project is intended to be deployed on AWS just for convenience, but with a little creativity, you can set it up entirely on-premises, or on a monolithic Linux server. In that case, you would have to implement by yourself an event bus that allows setting up simple distribution rules, a non-relational database (even json documents would work, it depends on each system), and an asynchronous API that allows connections via websocket and pub-sub communication model (I recommend Grapql).

*** IMPORTANT TAKE:  FRONT-END DEVELOPERS who have used React Redux or any other design pattern based on Redux… this is more or less the same but applied to the back-end context. So it might be easier for them to understand the essence and mechanisms described here ***


Theoretical framework:

I highly recommend reading a bit about DDD and its technical aspects in terms of implementation before reading further. I even highly recommend having a conversation with your preferred AI Chat about this topic. There are things that it is better to know beforehand so that my explanations make sense, if I tried to explain them in detail one by one... this would become a papyrus.

so, do some reserch to understand the basics:
  - https://www.google.com/search?q=domain+driven+design - "Domain Driven Design" ( DDD )
    
  - https://www.google.cl/search?q=event+storming Events Storming: It is the way in which these systems are designed, starting with an understanding of the domain, its problems, flows, interactions, etc, In terms of Imperative Commands (imperative orders, like many unix commands) and Events (inmutable past facts). And all you need is a big whiteboard and a lot of colored sticky notes and a collaboration between Users and Enginners to descrive it. 


Or just skip and go to the interesting things (implementation stuffs):
  - https://www.youtube.com/watch?v=GzrZworHpIk  - Event Sourcing You are doing it wrong by David Schmitz""

And talk with your AI Chat bot about DDD and how that matches in terms of implementation with Stateless, Event Driven, Event Sourced CQRS architectures.




Then.. Domain Driven Desing:
  
  - In the Domain Driven Design philosophy (and, paradoxically, in almost any software engineering project) the most important phase, in which more time should be invested, is the initial phase: Design and specifications.
    
  - Unless the project is being developed by an I+D team,or other one with developers without mastery of the technologies that are intended to be used, or by a team that, due to pressure from ambitious stakeholders and management who ignore how disastrous and expensive a project can become if it is started to be developed on the fly and without having a clear understanding of what the behaviors and flows are in order to design based on them... Software engineers, front-end/back-end developers and analysts, together with the users of the future system (if there are any) must focus on understanding and obtaining as much feedback as possible in order to describe each possible fact, interaction and flow of the system, possible errors, alternative flows, each possible automated or periodic tasks, edge cases, etc.

  - “Each state of anything in terms of an accumulation of past facts, immutable events. All the stories that could happen in that system and how, through them, we can project the current state (or the state on any past moment since the system started) for each entity in the system.”
 
  - For each User Story (user that interacts with a DDD system, called 'use cases' in other types of systems) the behavior, business rules, information flow, intervention of other users or systems in said flow must be identified at first.
 
  - Before developing any line of code, the behavior that the software should have for each case must be identified. The greatest amount of feedback from the users who will use the system (which are a fundamental part of this phase) must be taken into account, or, in the case of a new system, a consistent idea of ​​what is expected to be achieved, in order to define the specifications mentioned above.
 
  - Only when all the 'user stories' and system flows to get the basics sofware characteristics have been covered, should one move on to the development phase. And the above takes time. Normally it would take approximately 70% of the time (and cost) of the project. (sadly... this is not usually the case, at least not in the country I live in).
 
  - In DDD, the most important thing is the understanding, feedback and design of the stories that will be produced within the system. DDD is the design framework for a system based on its behavior, NOT based on the data models (entities) of the domain.
  
  - But the technical implementation of a DDD system is challenging, it has complexities that can make architects decide to use expensive frameworks and infrastructures that try to handle these complexities, or simply discard it and opt for the traditional (data model-based design) that, in the long run, turn out to be monolithic, difficult to scale, andconsiderably more expensive if it is explicitly required to obtain the features that a DDD system can provide by default... (A more detailed description of this last point will be left below).
 
  - It is this last point that I try to address in this project.


..So, what offer a sistem designed like this that really pay the effort???
  
  - Traceability and Consistency: The most important point in terms of the value it can bring to the business. but Why? 

  - Event Sourced: Allows reconstructing the system state of each "entity" (domain aggregate in DDD terms) from a sequence of inmutable events, providing greater traceability by default. You do not need to generate any kind of procedure by consulting logs obtained from a relational database (if there were enough of them) and try to reconstructfrom those the historical/operational traces of key system entities when it becomes necessary to generate statistical analyses and projections based on those traces, information that can be really valuable for the business,  I mean... USER BEHAVIOR or any entity behavior in:  
    - Social networks 
    - Online stores and marketplaces,
    - Financial systems 
    - Banking 
    - IoT 
    - Transport and logistics
    - Videogames ... 
    - etc, etc, etc... so, any big tech companies come to mind?
    
  - CQRS: Separates read and write operations, optimizing data performance and consistency.

  - Decoupling: Separates the domain logic from the infrastructure, facilitating changes in the business without affecting the technological infrastructure.
    
  - Scalability and Resilience:
  
  - Stateless: Reduces the load on the server by not storing state between requests, allowing for greater scalability.
      
  - Event-Driven: Improves responsiveness and resilience by processing events avsynchronously.

  - Maintainability: Promotes a more modular and maintainable architecture, where changes in one area of the system have a reduced impact on other areas.


In comparison, traditional data model-based systems are super easy develop and implemet but, as they grow in terms of components, entities and volume of data, tend to be more monolithic, less flexible and can have difficulty scaling and adapting to rapid changes in the business.


So, if You already reads some basic concepts of DDD, You may have noticed that a domain aggregate is, in practical terms that we already know, like a "Micro-Service" in terms of a traditional decentralized system that represents only one entity in a domain composed of several entities. 

A basic composition of a Domain Aggregate have, at least: 

  - A Command Handler with a Rules/Policies validator that generate system events based on recived command that pass all the varidations and rules. 
  - An Event Handler that just save (in a event store) the  emited events from (self and/or external) Commands Handlers, and notify this outside clients via an async/pub-sub API
  - An Event-Store to save self events.
  - A set of Event-Stores to store events from other domains (aggregates) of interest required to execute internal procedures. There are no calls to other entities (aggregates) in the domain, the current state of other required entities is achieved by storing others aggregates events and replaying their state (folding) internally just as the other entity would do. There are exceptions, but generally speaking this is the norm.
  - A Query handler that make posible do query to the Event Store and get states from one o more (self) aggregate instances.
  - A simple reducer function shared as a lib between Command and Query handlers to fold(reduce) and rebuild the actual state of the agregate reading the (historical) events form the event-store.
  - A set of delivery rules (configured on the event bus) to indicate the destinations of each event generated. Just a simple pointer to self and others aggreagtes that interested on the aggregate events generated. It can be described in the opposite way, a rule stating which aggregate events from other aggregates it will need, or in a more general way, which other domain aggregates it needs to receive all of its events from.
  - An async api.


Next, I will deviate a little from the theoretical framework and explain the objectives limited to this particular project, and then return to the theoretical framework so that the explanations revolve around the objectives of the project in a practical way, preventing this from becoming an indigestible papyrus.

Techical Objetives: 

  - To achive the most simple and lowcost way to mount a Stateless, Event Driven, Event Sourced CQRS Core System Services on AWS
  - Done by making three / four common (almost generics) decopuled Domain Aggregates that cover most common cases of interactions
  - Without using frameworks, just AWS CDK.
  - No CDK meta programing or templating magic.. Aggregate1 -> aggregate1stack.ts, Aggregate2 -> Copy/Paste and Modify ggregate1stack.ts, and so.
  - Main goal is build a starting point of an architecture that allows to implement Domain Driven Design in an a little easier way.

Challenge stuffs (outside the obvious DDD things):
  - Keep it simple. Keep it really simple.
  - Security layers / Authentication / Authorization.
  - Minimal cooldown start times in a lambda environment (currently I am using simplest Rust functions... 20ms or less at cooldownm start time, it's crazy)

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



Let's return to the theoretical framework.

Why Graphql as API ?
  - External systems (SSR web servers, for example) must communicate to this system in terms of asynchronous commands, (explicit orders of what needs to be done in the system), and in terms of asynchronous queries to request updated information from the system. Let's see an example to understand the problem:

    
  - Suppose a customer needs to make a hotel reservation. The front-end service (usually an SSR server) needs to send the request to the core in the form of a Command, specifying what is wanted, along with the required parameters to the 'Reservation Domain Aggregate Endpoint'.Lets supose the command “createReservation { userId: asdf1234, roomID: adsf1234, dateStart: ‘some date’, dateEnd: ‘some other date’}.

  
  - The Reservation Aggregate  needs to validate parameters and business rules of the command "createReservation" before respond. The SSR server needs to wait for the response of the Command(request) to show the user if the request could finally be 'approved', or if a bussine rule validation (or a technical error) occurred. But it only needs to wait for the response of the validation of the sent command, NOT the final result of the command.  The final result of the command (if all the validations were passed) is a new Event associated to the ID that was obtained or sent to Command handler  in step 1.  This event is propagated internally in an asynchronous way. Meanwhile, the client (front end server) can connect through another request to the Query Handler (via the GraphQl API) requesting the NEW STATE of the agregate instance that matches the ID mentioned above, and wait for a response with the new event (that is an updated status of aggregate instance) arrives. This process is asynchronous, and uses websockets as a means of communication (you need to have some understanding of how Grapql apis work, and how they can be used as middlewares to understand the benefit of this, I recommend reading a bit of this topic, or wait until you have the first version implemented in Rust ready to check for yourself). It seems to be a long operation... but in reality it is very fast, I am talking about milliseconds.  

- The SSR has two options: Block the user's UI (the dumb option), OR take for granted that, as the command was already approved, at some point the response with the status of the updated aggregate instance will arrive, and use a tentative strategy... which is to show the user the already updated content with a small warning that the final response is still being processed in the server.... This sounds like React Suspense API to me :). The user's UI is not blocked, and the user can continue doing other things. But if for some internal reason the event does not propagate (timeout) or if an error message is received, then the user is notified that there was an error to validate the request and that he will have to generate it again (a popup or something similar). Let's take a closer look:

*** As far as I know, grapql apis do not have default timeouts, or these are very long because the technology on which it is based (websocket) does not have default timeouts. However, there are several strategies to generate client-side timeouts, including dynamic timeouts for each type of command that can be reported by the response of the command handler. 
  
  - The graphql aggregate endpoint sends to the aggregate Command-handler component the command (request), and it will not respond immediately since the request, before generating any change in the system, must be validated in terms of business rules, generate and distribute internally an event if the request passed all business rules, or respond immediately with an error if the business rules validation failed through the same request. This operation introduces two cases:


  - Case 1: In case of error of any business rules validation: The Api respond through the same request (command graphql) in the same way the case 1, but with the error specification. This operation is synchronous, i.e., the one who generated it should always wait for a response through the command (however, for complex cases and extensive validations, it could also be handled asynchronously but, to keep it simple, it is better to handle this operation synchronously).
men
    
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


Let's see a simple flow of a Client - Server interaction
Consider that the client is a front-end server (ssr) that interacts through a graphql api with the Core of the system.

Pseudo Code:
```
 enum CommandHandlerResponse<T, E> {
      Success(T),
       Error(E),
    }
enum QueryHandlerResponse<T, E> {
      Some(T),
      Error(E),
    }
type commandParams = [..inputParams, maybe(ID)]  

Client[ callApiAndWait( doSomethingCommand(commandParams)) >..waiting..> onCommandRespond(Success(ID)) >> queryAggerateByIdAndWait(ID) >..waiting..> ] 
                        ↓                                                                     ↑
Api[ doSomethingCommand(commandParams) >> implicitSendCommandAndWait(commandParams) && respondToWaitingClient(Success(ID))] 
                        ↓
Command_Handler[ validate_rules(commandParams) >> emmit_to_bus(newEventCreated) && responseWithID(newEventCreated) ] 
                        ↓
Evento_Bus[ delivers(newEventCreated) ] 
                        ↓
Event_Handler[ capture_and_store(newEventCreated) >> publish_to_api( onAggregateEventEmited(newEventCreated)) ] 
                        ↓
Api[ onAggregateEventEmited( passEventToApiQueryHandler(Some(newEventCreated)) ) ]
                        ↓
Client[ >..waiting..> queryAggerateByIdAndWait(Some(newEventCreated)) >>  sendToUserUI( Some(newEventCreated) ) && cachOrUpdateAggrProjectionOrAnyOtherSSRprocess(newEventCreated) ]
```

.. well, it seems like a long and slow proccess just for a simple interaction... 
.. I can say you that meybe not :)



Considerations in front-end/back-end, communication, components and eventual consistency:

Note: The front-end is not my domain field. Nowadays it is an extensive field, it evolves very quickly and the main complexity is learning how to deal with it, it requires dedication, passion, creativity and good taste. Finding the best alternative that adapts to each project is not easy since it requires experience that I do not have, it is not within my current knowledge and capacity. However, there are some minimum considerations that must be taken on front-end/back-end servers when interacting with any system of this type (event-driven system, not mutation-driven system).

To recap: 

Front-end-servers must communicate via Graphql APIs (websockets under the hood) with domain aggregates (core-servers). The reasons I already mentioned above, the most important being: the ability of Graphql APIs to function as pub-sub brokers at the API level in a simple way and without additional components, which is essential if you want to achieve secure asynchronous communication. , fast, and without redundant requests (... without webhooks). 

The second important consideration is that front-end-servers should also communicate with UX/UI clients via websockets and, if possible, also in terms of commands and queries. 

Another consideration is that strategies could be implemented to cache within the context of the SSRs the projections (“current states”) of the recently requested domain aggregate instances, in this way redundant connections and queries to the cores could be avoided. servers. Although there may be trade-offs in adopting this strategy on a global level... but since domain aggregates live in different contexts, separated by well-defined contextual boundaries, each domain can implement a different caching strategy, tailored to the specific needs of that domain. . 
And the same thing happens with many other technical aspects: various technical and infrastructure parameters at the Core level can (and should) be adapted to each domain aggregate, but it is valid to start with something generic. However, the above DOES NOT APPLY to the general technical decisions, patterns, definitions and conventions that are part of the architecture on which this type of system is based.

Some things that can be configured, evolved, and scaled independently in each domain in your Core are: 

- Lambda runtime type (language in which it will be programmed)
- Whether there will only be one lambda per component (command, event and query handlers), or whether there will be more lambdas (internal) for specific functions that need higher performance settings given the nature of what it has to process (more granular approach).
- Timeouts of each API, at a general and granular level (for example, it can and (and should) be included in each response generated by the Command Handlers to the clients of each API (how long the client must wait to consider a crash for time out) parameter that can even be dynamic, defined in each response of the commands-handlers based on the load that the domain aggregate is having.
- Type of Event Storage that each aggregate will use.
- It may even be the case that some component of the domain aggregate or its completeness cannot be implemented without state, and a Statefull instance is required (dockers, EC2 instances, or other similar ones). This case will require a monolithic approach to the development of the basic components (command, event and query handles) and there could be several internal applications on the server collaborating to process the aggregate tasks. But even in this case, the software architecture patterns must be respected, that is, communications between the command-handler and event-handler will continue via Event Bus, the clients api will continue to be asynchronous via websocket, etc., the rules are the same.
- ... These are some configurable parameters, but there may be many more, and everything will depend on what each domain aggregate does. However, none of them should break the internal asynchronous communication model (data bus), command and event scheme, etc.
  
 When referring to projections, we refer to the current states of the instances of a domain aggregate. But, is it necessary to constantly synchronize with the core-servers to keep these instances updated and thus avoid inconsistencies? The answer is No. So how? 

…This is where things get interesting, not because it is difficult but on the contrary, it is because of how easy it is to use systems that have eventual consistency covered by design. I'll try to be concise: 

Suppose there is a domain aggregate named “Reservation” from which we want to request a new reservation. Uppercase parameters are MANDATORY for all events and commands that are generated during the flow, regardless of which domain aggregate they belong to. The other parameters are those associated with the particular domain of each aggregate:

... wriiting some diagrams, work in proggress

I'm currently writing more stuff, synthesizing some ideas, and will update this document as I can. The most interesting part comes when you realize the benefits it gives you in distributed systems... and the frequent data caching on the SSR side in the most efficient and simple way... I'm excited.


This is project build with AWS CDK TypeScript.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `npx cdk deploy`  deploy this stack to your default AWS account/region
* `npx cdk diff`    compare deployed stack with current state
* `npx cdk synth`   emits the synthesized CloudFormation template
