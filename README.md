This is only a proof of concept at a very early stage. A technical attempt to demystify and do something that has already been done for a long time behind the scenes by large tech companies to "solve large scale problems" (...or creating large scale problems?...)  in different industries: financial, logistics, marketing, video games, IoT, Robotics, AI, etc, etc., but taken to terms that it is possible for me, as an average engineer with a very poor education and limited resources, to implement. It is a set of technical proposals and knowledge still under development that I have worked on for many years based on problems that I have faced in the corporate environment and I have had to solve by brute force... in companies that prefer to extend malformed projects for year... and waste lot of money trying to implement ineffective, inefficient, costly, based on patches and tricks, obsolete solutions...  and how this type of philosophy in software design can contribute immensely outside the context of large tech companies.  


*** 
This README file is under construction and has constant changes, sometimes they are just spelling corrections, but others are incoporation or correction of ideas. Please check. 

*** 

UPDATE - BACK-END ENGINNERS: If what you try is to migrate a system that is currently implemented with microservices, but without this pattern (of the traditional form) towards this type of systems (Event-Driven), you should start little by little, one service at the same time(Micro-service or set of microservices).

The simple way to talk microservices that work with a database and mutable tables is:

- Add an additional field 'version' to a table that the microservic entity reprieves.
- Every time one or more of said table is mutated in a only transaction, the microservice must increase by 1 (version 1, version 2, version 3, etc) the 'version' field of that table and, in addition, you must send aData bus event indicating the change of version of the instance, next to the fields that were mutated.
- The domain aggregates that are receiving the events of said entity must generate the chorosphere changes in their events (those who handle that entity)
- Each query that is carried out (externally) must be answered including the 'version' of the instance on which they were based to answer the Query.
- And, finally, in each Query that includes any data of the instances that are not event-soured, the version of said INTANCE on which they were based should be included.

The above will allow to maintain the eventual consistency even with services or microservices that are not operating with Event-Stores (only with traditional databases)

***

CLARIFICATION: 

Capturing telemetry data and tracking user behavior is not why this software design patterns were created. Can they be used for that? Of course, but that's not the goal.

This type of decentralized software design arose from the need to create systems in which their domain entities operates truly decoupled, both in their application layer and in their data layer. It is an answer to solve the problems (inconsistencies?) presented by decentralized software architectures based on microservices, where its application layer is decoupled, but its data layer is not... where, in the end, each microservice that needs data from another entity in the domain must make a request to said entity to obtain the states of its instances, or its databases are shared... which makes all the paraphernalia to decentralize the software in microservices become a non sense. 

You must keep in mind that not all components of the system needs to be handled by this pattern, for example, User's entity handlers and authentications/authorization handlers could by handled in a traditional services (within a sql database) or by other external systems.

... so, then:

In many domains, by definition and nature (and in many cases, because the law establishes it), the data needs to be stored as a succession of immutable events. 

For example, in banking and financial systems, at least where I live, by law the minutes of movements and transactions carried out must be kept as immutable events. 

Another example: The operations carried out in the judicial system, by law, must also be saved as a succession of immutable events. 

Another example: In passenger transport domain, by their nature, each signal recived from a bus, train, aeroplane, and any other, they are also stored as a succession of immutable events (GPS and sensors signals). And his pay systems saves historical events each time that you arrives and uses it.

Another example: Many video games, online or offline, especially sports and competitive ones, store the match in the form of a succession of events. For many years, even before the internet was popular, soccer videogames, racing videogames, shooters, and many others, were (and are) able to completely reproduce the entire match, in fact, you can save it and view it later. How do they achieve this? because the core of their systems saves each state change as a sequence of events by default. 

Have you ever played Fortnite? You can completely replay the match when you finish it... and it is not stored by saving the rendered data, since each state changes in the match is an event, and its history is a succession of events, these can be replayed later whithin no  additional cost in the videogame engine. The Ram memory records each state change of the match as a succession of events that, from time to time, are dumped into a cache file (to free up memory), thus which finally achieves a complete record of each action of the match. This is why many video games are capable of replaying matches in their entirety after finishing the match, or even before having finished playing them... In soccer videogames and other sports, at any time during the match you can pause and check the replay back. Also, this data could be used by Epic (and their third party afiliates) to improve anti-cheats and make better mechanics to catch troll/hater users analyzing suspicious user behavior in real time, but I'm not sure if they do this.

Even in Play Station 1, Winning Eleven (a Classic, Anthological soccer video game) you could replay the match at any time while you were playing it. How do you think they were able to do it? Because its core was event-driven by default, that was (and still is) the most efficient way to handle changes in the game state by its core. Enabling the functionality to replay those events was trivial, since the software design already provided this feature out of the box.
This doesn't apply to all video games, but it's a common strategy applied to competitive video games.

There are hundreds of cases where this paradigm is not an option, but rather an intrinsic requirement of the domain.

Was generative artificial intelligence created in order to maximize the economic benefits of the companies that finance its development?? 
I think not, but the reality is that this technology allows it. It depends on each person and entity what use is given to said technology.


 ***



well ..

This is about software design and architecture. This is about Domain Driven Design... and how to design systems that perceive, interpret, process, interact and remember changes in ourselves and the environment in the way our mind (currently) does... as a succession of immutable events over time that modify our own state and that of the world around us.

- note -1: Sorry for the spelling mistakes, English is not my mother speak tongue. For some things I used a translator, for others I wrote them as I went along, sorry for that. Also sorry if I expose the ideas and concepts in a scattered way ... in my defense, this is not a paper, it is just a memory aid for myself, that I hope someday, someone can share and implement.

- note 0: This project is intended to make a CORE SYSTEM ONLY.. and how it communicates internally and externally. Could it be used as Core of a web application? Yes, in fact, that is the goal... the thing is... not in the same way you would usually expect. There is no mutation, no CRUD, ..no database magic under de hood.

- note 1: The first sketches and proofs of concepts were developed in Typescript/Python... now rewriting same things in Rust (just the functions). Buy why Rust? only because I consider that, in this context (stateless, serverless web services), you get unsurpassed performance without using the language at low level, just using high-level api features, which is almost like developing in any other popular modern language (it's almost like writing typescript). There is no low level programming here, so There is no need to panic. Also, in essence, you can use whatever language you prefer or best suits each part of the system. It is totally valid that in this type of system, each domain aggregate ('micro-service' in old school terms) uses a different language, which best suits the needs of the domain aggregate.

- note 2: Front-end servers aren´t considered on this project yet.. but I will expose some ideas regarding the front-end and how this design be taken advantage of by fron-end system enginners in terms of asynchronous communication and caching by default. 

- note 3: For now, this project is intended to be deployed on AWS just for convenience, but with a little creativity, you can set it up entirely on-premise (on your desktop), or on a monolithic Linux server. In that case, you would have to implement by yourself an event bus that allows setting up simple distribution rules, a non-relational database (even json documents would work, it depends on each system), and an asynchronous API that allows connections via websocket and pub-sub communication model (I recommend Grapql).

*** 
IMPORTANT TAKE:  FRONT-END DEVELOPERS who have used React Redux or any other design pattern based on Redux… this is more or less the same but applied to the back-end context. So it might be easier for you to understand the essence and mechanisms described here ...Also, in case you haven't noticed, if you've been using Hooks in React for a while and consider yourself good at it... surprise, you're a functional programmer. I mean, you don't build Applicative Functors or Monads... but you consume them as APIs all the time, and you use a paradigm where everything is immutable and the state of components are sequences of immutable events behind the scenes.
***

*** 
ANOTHER TAKE: BACK-END Developers... You should not be intimidated by the Rust language. We are not low-level programmers. Our code is, in most cases, high-level code. Rust can be complex when used for low-mid-level software, but we... we just write functions and make APIs. Rust's rules are easy to learn and respect by working with this language at a high level. They are easy to master if your coding style is not based on OOP. If your code tends to be functional at the general level, and procedural at the granular level, you should not have problems. But if your coding style is based on OOP, inheritance, state mutation... Better not use Rust, use any other language that you master in which OOP is the base paradigm (java, python, go, ryuby, php, c++, etc.) following the architectural patterns described here. The language does not matter. It's just that Rust... for this kind of stuff is easy, and you get NASA performance without much effort. Give it a try. 

Another personal recommendation is that you use Go (Golang) language if your programming style tends to be procedural, but you know or have notions of how to work based on types, functions and routines.It is a language similar to C in its syntax for what simple to learn if you come from the C environment or even Python environment, it has an execution system of synchronous and asynchronous routines (it is one of the main characteristics of language), it is much less verbose than Rust, Kotlino Java (modern) and, although it does not incorporate many predefined types in its base library, its type system allows you to easily declare your own types (structures) and interfaces (traits to which the types can be associated) which converts the Go language on fertile land to implement your own types according to the needs of the domain you are manipulating. Also its compiled, that means you will get highly boosted performance when you code is compiled to final production build.  
I am currently learning this language and generating some simple proposals to demonstrate how the types system of Go can be approved in this context.But it is a job in process, I will upload some examples when I have something more orderly.

***

*** 
My respect to low-mid level programers working on the layer  7 building good tools.  Without them, we would hardly be able to address this in a simple way 
***   


Theoretical framework:

I highly recommend reading a bit about DDD and its technical aspects in terms of implementation before reading further. I even highly recommend having a conversation with your preferred AI Chat about this topic. There are things that it is better to know beforehand so that my explanations make sense, if I tried to explain them in detail one by one... this would become a papyrus.

so, do some reserch to understand the basics:
  - https://www.google.com/search?q=domain+driven+design - "Domain Driven Design" ( DDD )


  - https://www.youtube.com/watch?v=P1vES9AgfC4 Moving IO to the edges of your app: Functional Core, Imperative Shell - Scott Wlaschin - 12 abril 2024


Events Storming: It is the way in which these systems are designed, starting with an understanding of the domain, its problems, flows, interactions, rules, policies, etc, In terms of Imperative Commands (imperative orders, like many unix commands) and Events (inmutable past facts) produced by those commands. And all you need is a big whiteboard,  a lot of colored sticky notes and a collaboration effort between Users and Enginners to descrive it. No technical knowledge is required at this stage, only knowledge of the domain to be described.

  - https://www.google.cl/search?q=event+storming 

Architecture Modernization: Aligning Software, Strategy & Structure • Nick Tune • GOTO 2024
- https://www.youtube.com/watch?v=DwAI2NqscMo
- I emphasically recommend watching the video this video.


Or just skip and go to the interesting things (implementation stuffs):
  - https://www.youtube.com/watch?v=GzrZworHpIk  - Event Sourcing You are doing it wrong by David Schmitz""

And talk with your AI Chat bot about DDD and how that matches in terms of implementation with Stateless, Event Driven, Event Sourced CQRS architectures.




Then.. Domain Driven Desing:
  
  - In the Domain Driven Design philosophy (and, paradoxically, in almost any software engineering project) the most important phase, in which more time should be invested, is the initial phase: Design and specifications.
    
  - Unless the project is being developed by an I+D team,or other one with developers without mastery of the technologies that are intended to be used, or by a team that, due to pressure from ambitious stakeholders and management who ignore how disastrous and expensive a project can become if it is started to be developed on the fly and without having a clear understanding of what the behaviors and flows are in order to design based on them... Software engineers, front-end/back-end developers and analysts, together with the users of the future system (if there are any) must focus on understanding and obtaining as much feedback as possible in order to describe each possible order (command), rule (policy,validation)   fact (posible events), interaction, flows , possible errors, alternative flows,  each possible automated or periodic tasks, edge cases, etc.

  - “Each state of anything in terms of an accumulation of past facts, immutable events. All the stories that could happen in that system to, through them, project the current state (or the state on any past moment since the system started) for each entity in the system.”
 
  - For each User Story (user that interacts with a DDD system, called 'use cases' in other types of systems) the behavior, business rules, information flow, intervention of other users or systems in said flow must be identified at first in terms of commands, policies (rules) and events. 
 
  - Before developing any line of code, the behavior that the software should have for each case must be identified. The greatest amount of feedback from the users who will use the system (which are a fundamental part of this phase) must be taken into account, or, in the case of a new system, a consistent idea of ​​what is expected to be achieved, in order to define the specifications mentioned above.
 
  - Only when all the 'user stories' and system flows to get the basics sofware characteristics have been covered, should one move on to the development phase. And the above takes time. Normally it would take approximately 70% of the time (and cost) of the project. (sadly... this is not usually the case, at least not in the country I live in).
 
  - In DDD, the most important thing is the understanding, feedback and design of the stories that will be produced within the system. DDD is the design framework for a system based on its behavior, NOT based on the data models (entities) of the domain.
  
  - But the technical implementation of a DDD system is challenging, it has complexities that can make architects decide to use expensive frameworks and infrastructures that try to handle these complexities, or simply discard it and opt for the traditional (data model-based design) that, in the long run, turn out to be monolithic, difficult to scale, andconsiderably more expensive if it is explicitly required to obtain the features that a DDD system can provide by default... (A more detailed description of this last point will be left below).
 
  - It is this last point that I try to address in this project.


..So, what offer a system designed like this that really pay the effort???

  
  - Traceability and Consistency: The most important point in terms of the value it can bring to the business. but Why? 

  - Event Sourced: Allows reconstructing the system state of each "entity" (domain aggregate in DDD terms) from a sequence of inmutable events, providing greater traceability by default. You do not need to generate any kind of procedure by consulting logs obtained from a relational database (if there were enough of them) and try to reconstructfrom those the historical/operational traces of key system entities when it becomes necessary to generate statistical analyses and projections based on those traces, information that can be really valuable for the business,  I mean... user's, system's and any other entity behavior in::  
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
  
  - Stateless: Reduces the load on the server by not keep states in session's memory between requests, allowing for greater scalability (state changes are stored has domine events).
      
  - Event-Driven: Improves responsiveness and resilience by processing events avsynchronously.

  - Maintainability: Promotes a more modular and maintainable architecture, where changes in one area of the system have a reduced impact on other areas.


In comparison, traditional data model-based systems are super easy develop and implemet but, as they grow in terms of components, entities and volume of data, tend to be more monolithic, less flexible and can have difficulty scaling and adapting to rapid changes in the business.


So, if You already reads some basic concepts of DDD, You may have noticed that a domain aggregate is, in practical terms that we already know, like a "Micro-Service" in terms of a traditional decentralized system that represents only one entity in a domain composed of several entities. 

In this type of systems, communication between each domain aggregate occurs through a globally accessible event bus. Depending on the nature of the domain and the concurrent load it must support, you can configure 1 global bus that will receive and distribute all system events regardless of the domain aggregate to which they belong, or you can also configure it in a granular way, that is, one event bus for each domain aggregate or a set of buses that manage events for groups of domain aggregates. 

Everything will depend on the characteristics and load that each domain aggregate or group of domain aggregates requires (... I think that laptops, PCs, smartphones, consoles and servers, at the hardware level, also work that way).

It should be noted that, in this case, it is recommended that the event bus be the one that has the event distribution rules, since it is easier to manage and is visible to everyone at a general level.

Taking all of the above into consideration, a basic composition of a Domain Aggregate have, at least: 

  - A Command Handler with a Rules/Policies validator that generate system events based on recived command that pass all the varidations and rules. 
  - A set of delivery rules (configured on the event bus) to indicate required events. Just a self pointer (to catch all self events) and a reference to others (some or the entire set) domain aggregate events that will be needed in the self context to achieve his tasks.
  - An Event Handler that just save (in a event store) the  emited events from (self and/or external) Commands Handlers, and notify this outside clients via an async/pub-sub API.
  - An Event-Store to save self events. Techically, a NoSQL Database, or even just a json file. It will depends on each aggregate.
  - A set of Event-Stores to save events from other domains aggregates of interest, required to execute internal procedures. There are no calls to other entities (aggregates) in the domain, the get the current state of other required entities is achieved by storing events of those aggregaters internally (inside) and replaying (folding) them,  just as the other entity would do. There may be exceptions, but generally speaking this is the norm.
  - A partitioning rule defined in the Event-Stores to generate snapshots (and backups) for each event-sore. Depending on the needs of the aggregate of domain, this can be configured to perform every 1 year, every 6 months, one month, a week or even one day if the domain aggregate manipulates to many events. Everything will depend on the needs of the domain aggregate but, usually, snapchos are performed every 6 months or once a year.When performing a Snapshot, the system can be restarted from the final stored project Snapshot, which improves the speed of response in the reading and generation of new projections (starting with a base projection a more limited list of events to fold).In addition, this strategy can help generate system recoveries in case of failures, if necessary.
  - A Query handler that make posible do query to the Event Store and get states from one o more (self) aggregate instances.
  - A simple reducer function shared as a lib between Command and Query handlers to fold(reduce) and rebuild the actual state of an agregate instance reading the (historical) events form the event-store.
  - An async, low-latency, API. In this context, Graphql can by good. But you can implement your own over websocket or you sefl-maded fine-tunned protocol and channel. The point is that it must be able to communicate with the SSR servers (server-to-server clients) in a simple, secure and efficient way.

***
SUGGESTION:

Software developers/engineers should work primarily on command handlers and query handlers. 
In addition, they must work on the management of errors in the internal (at Event Bus level) and external (at Async Api level) distribution of events, which can be handled in different ways, but the usual thing is through Timeouts, Dead-Letter Queues and compensation events.
These components, depending on which part of the domain the domain aggregate they belong to is solving, may grow in logic (although good software design shouldn't allow this to happen). To test the correct functioning of these components, integration tests are used, not unit tests.

Is it necessary to do unit tests on these components? (I mean, functions and modules declared inside lambda functions)
It is valid, but it should be avoided. 
It is valid when these components are developed based on OOP, imperative programming, procedural programming or with dynamic languages. 
It is avoidable when a declarative programming, functional programming pattern with static languages ​​is used (Regardless of the language is used and if this is "pure functional language"  or a multiparadigma language). 

Components should not incorporate too many software pieces and low-level programmed pieces unless they are really necessary. 
Tests should be focused on the integration of all aggregate domain components, correct event distribute and on the correct processing and execution of business rules. 

So, as a suggestion in this context, it is preferable to use functional and declarative programming patterns, rather than object-oriented patterns, even if these language are multiparadigm like C#, Java, Go, Typescript, C++, etc. This can be achieved without much fuss in any of those languages following these conventions:

- Avoid using global variables and objects, unless they are final (immutable) and static, declared only once in the Lambda context (for example: initializing the 'client' to connect to DynamoDb or Event Bridge).
- Although it is recommended to avoid it as it adds complexity, if a 'context' type object is really necessary, it should be treated as a sequence of 'context' states, where each new state is a copy of the previous one with the updated state (field) value (just like the React 'useState' state hook does). Or, a locking mechanism should be used on the context object so that it is only accessible by one function at a time. It is recommended to avoid using context type objects.
- Express all operations as functions, and that these receive as arguments all the parameters they need to operate (even the 'clients' to connect to the different APIs must be passed as arguments)
- Each function (inside the lambda) must be declared with a concrete parameter list, and a concrete return (final objects, preferably immutable) based on the arguments it receives, and DO NOT modify (mutate) variables that are accessible outside the context of the function, even if they are received as parameters.
- A function must NEVER return NULL, even if the language allows it. If the operation performed inside a function fails, it must return a concrete object declared in the function signature, which can be expressed as the union of two types or use compound types that the language used has, which could be enums, built-in types or types declared by oneself, for example: type MyDecoderFunctionResult = Okay(String) | Error(String) in Typescript.
Lo anterior puede ser logrado en lenguajes que no tienen tipos incorporados para hacer esto, pero su sistema de tipos es consistente. Por ejemplo, en Go, seria algo como:

https://github.com/Manuel-GG-89/Golang-Proposals/blob/main/sync-async-dispatch-with-result-types-incorporated.go


- It is valid to use mutability in a granular way (for example, in for loop blocks), ONLY within the function or block that will use the declared mutable variables.
- Do not call a function within another function unless it is a 'helper' type function and not part of the procedure. This could produce spaghetti code. The main Handler of the lambda function must be the one that orchestrates the operations that occur in the lambda in a sequential way.
- DO NOT USE INHERITANCE, OBJECTS WITH MUTABLE STATE CHANGES, OR ANY OTHER TYPE OF OOP FEATURE. If the code grows in volume (many functions), separate them into files in a modular way. If it is pertinent to use inheritance to describe complex types with inherited fields, use ENUMs (Rust way), Structs (Go, C and Typescript way) or Classes without methods (Python, Java, Kotlin and many other). In this context, INHERITANCE OF BEHAVIOR (methods) IS NOT RECOMMENDED, but if required, they should be implemented as 'Traits' (Intefaces in Go and  Functional Interfaces in Java, for example) or some similar mechanism that the language used has.
- AVOID the use of dynamic languages.

Finally, if you want to make sure that it is easier for you to comply with these conventions, use Rust. Your code will not compile if you do not follow these conventions, since they are incorporated as rules of the language.

***

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
  - DSL, Domain-specific language: Separating business rules from code, so that they can be modified by people who only have business knowledge, but not IT knowledge. Json, XML or another common format can be used, so that the software, at compile time (better performance and security) or even at runtime, can interpret them and incorporate them into the business policy/rule validator. It is a common practice in corporate software, but there must be a consistent design of the internal and external DSL.

Resources, tools and technical stuffs:
  - AWS CDK to declare and configure all resources (default typescript cdk version)
  - No statefull servers, just serverless functions
  - No Kafka or any other similar.
  - Internal communication through EventBridge data bus (not sqs/sns pub-sub pattern, delivery rules configured on the bus)
  - Step functions in case needs to handle transactions (saga pattern) (flow nor even designed yet ...)
  - No relational database. Just Dynamodb as an events store (historical events, no mutatios)
  - No http/rest apis, no webhooks or callbacks. Graphql (Appsync) acting like pub-sub midleware to expose allowed commands and querys (async operations) from each domain aggregate to front-end servers. This api should NEVER, NEVER! be exposed, only SSRs servers should be able to make requests, and they should have been a strict security layer armored in this layer.
  - Rust lambda runtimes to reduce cooldown start at minimal.
  - Shared Type schemmas between Domain Aggregates for Commands, Events and Querys when necesary at dev cycle and compilation/build time (easy on Rust, Go and Typescript).



Let's return to the theoretical framework.

*** Note: The front-end is not my domain field. Nowadays it is an extensive field, it evolves very quickly and the main complexity is learning how to deal with it, it requires dedication, passion, creativity and good taste. Finding the best alternative that adapts to each project is not easy for me, since it requires experience that I do not have, it is not within my current knowledge and capacity. However, there are some minimum considerations that must be taken on front-end/back-end servers when interacting with any system of this type (event-driven system, without mutations). ***

-*** Another Note: SSR servers do not need to be serverless or stateless. They can perfectly well be Node/Deno servers, PHP servers, Rails servers, or any other that fits the application’s requirements. The only consideration to keep in mind is that the framework and platform used must have good support for interacting with asynchronous APIs (preferably GraphQL APIs) without blocking the user interface. Additionally, it is ideal if the framework and platform used have good mechanisms for caching the results received from queries and interacting with them before making requests to the backend/core. Most queries made through GraphQL APIs to domain aggregates respond with one or more updated states (projections) of the instances of a domain aggregate. In principle, if the queries come from an SSR node to obtain one or more states of aggregate instances, and only that SSR node interacts with those instances, then the SSR should first check the cache if it already has projections of those instances in cache and trust them before making a request to the Core. Eventual consistency is not a problem, as each instance projection has an associated sequence number, and the commands sent to the server include this sequence number (they include the version of the instance they are based on), which is compared each time a new command enters the system using that projection as a source. If the sequence number does not match the projection generated within the command-handler (directly from the event-store), then the command is rejected. Only if the versions match, the command is processed. Since each SSR node only works with segments of instances and not with all available instances, those nodes will (almost) always have updated projections for the segment of instances ***


Why Graphql as API ?
  - External systems (SSR web servers, for example) must communicate to this system in terms of asynchronous commands, (explicit orders of what needs to be done in the system), and in terms of asynchronous queries to request updated information from the system. Let's see an example to understand the problem:

    
  - Suppose a customer needs to make a hotel reservation. The front-end service (usually an SSR server) needs to send the request to the core in the form of a Command, specifying what is wanted, along with the required parameters to the 'Reservation Domain Aggregate Endpoint'.Lets supose the command “createReservation { userId: asdf1234, roomID: adsf1234, dateStart: ‘some date’, dateEnd: ‘some other date’}.

  
  - The Reservation Aggregate  needs to validate parameters and business rules of the command "createReservation" before respond. The SSR server needs to wait for the response of the Command(request) to show the user if the request could finally be 'acepted', or if a bussine rule validation (or a technical error) occurred. But it only needs to wait for the response of the validation of the sent command, NOT the final result of the command.  The final result of the command (if all the validations were passed) is a new Event associated to the ID that was obtained or sent to Command handler  in step 1.  This event is propagated internally in an asynchronous way. Meanwhile, the client (front end server) can connect through another request to the Query Handler (via the GraphQl API) requesting the NEW STATE of the agregate instance that matches the ID mentioned above, and wait for a response with the new event (that is an updated status of aggregate instance) arrives. This process is asynchronous, and uses websockets as a means of communication (you need to have some understanding of how Grapql apis work, and how they can be used as middlewares to understand the benefit of this, I recommend reading a bit of this topic, or wait until you have the first version implemented in Rust ready to check for yourself). It seems to be a long operation... but in reality it is very fast, I am talking about milliseconds.  

- The SSR has two options: Block the user's UI (the dumb option), OR take for granted that, as the command was already approved, at some point the response with the status of the updated aggregate instance will arrive, and use a tentative strategy... which is to show the user the already updated content with a small warning that the final response is still being processed in the server.... This sounds like React Suspense API to me :). The user's UI is not blocked, and the user can continue doing other things. But if for some internal reason the event does not propagate (timeout) or if an error message is received, then the user is notified that there was an system error processing the aproved command request and that he will have to generate it again (a popup notification or something similar). Let's take a closer look:

*** As far as I know, grapql apis do not have default timeouts, or these are very long because the technology on which it is based (websocket) does not have default timeouts. However, there are several strategies to generate client-side timeouts, including dynamic timeouts for each type of command that can be reported by the response of the command handler. ***
  
  - The graphql aggregate endpoint sends to the aggregate Command-handler component the command (request), and it will not respond immediately since the request, before generating any change in the system, must be validated in terms of business rules, generate and distribute internally an event if the request passed all business rules, or respond immediately with an error if the business rules validation failed through the same request. This operation introduces two cases:


  - Case 1: In case of error of any business rules validation: The Api respond through the same request (above, client command-request), but with the error specification. This operation is synchronous, the one who generated it should always wait for a response by the  command-handler (however, for complex cases and extensive validations, it could also be handled asynchronously but, to keep it simple, it is better to handle this operation synchronously).
    
  - Case 2. In case that the event passes the validation of all business rules, the requester will receive a response with an ID generated by the command-handler (or the ID sendded) and a version number og the projection in what the command was based. This is that will be used to request the updated status of the domain aggregate instance created (or modifyed). This is where the asynchronous part comes into play. Let's see why, detailing a little more the flow:

  
  - If a BUSINESS RULE VALIDATION ERROR ocurred, the aggregate endpoint (linked to the command-handlre) responds with the bussiness rule error (or an eventual consistency erro).

  
  - But, if the command handler had no bussiness validations errors, JUST IN THAT CASE, it will respond whit an ID of the reservation created (or modifyed) by the command handler. But this ID cannot be considered automatically materialized by the front-end, since this only validates that the command handler had no bussiness errors and issued (propagate) the 'createReservation' via the event bus, but there could be technical errors at the time of internal distribution and saving of this event, so we cannot assume that this event is already part of the system.

  
  - So the front-end should take a tentative stance: “Ok, I’m going to assume that the event validation is already Ok because command-handler givees me an ID and a nuew version number from the event generated by the command-handler. I’ll associat these ID and version number whith the command's parameters sended and showing the user something like "Recervation Accepted, whaiting final confirmation"  (using the same parameters that the emited command has), . Internally, I’ll subscribe (via the grapql api) to the (query handler) query 'getReservationById': { reservationID: “ID + version number generated by the command handler”} and I’ll only change the reservation status to ‘Reservation Ready’ on the user UI, and I'll update the projection (chached) state of that incance when I receive a success message through that subscription, or an error message if it responds with an error or timeout (in this case, the least complex option is to configure a timeout internally and externally (api) , so if the event issued by the command is not processed (not saved) in X amount of time, the aggregate invalidates the event internally and externally).”

    
  - Eventually, the aggregate EVENT HANDLER will receive the event emitted by the command handler, store the event in the even store and emit a success or error response to the graphql mutation 'reservationSaved{ reservationID:adfgadf}'. This mutation are binded via subscription (grapql subsription) to the getReservationById query by a subcription, so that subcription will be triggered, the getReservationById query JUST WHEN THE ID matches with the reservation ID generated by the command handler, unless an intenral (techical) error ocurred, . With a well designed grapql scheme , it will be able to inform errors too based on the same trigger.

      
So.. why Graphq and not an http api with webhooks or callbacks?
  - Easy implementation of a sub-pub communication model.
  - Default asynchronous communication
  - Websockets under the hood, which provides more security, speed, and reduces the number of connections and redundant requests.
  - If webhooks were to handle asynchronous communication, the client (server client) would need to repeatedly query the webhook until a result is obtained (redundancy, resource wastage) and would also need to consider the configuration of parameters such as the frequency for querying the webhooks and the corresponding timeouts.
  - And callbacks.... well, callback hell? I dont even know...
  - ...So far, this is the best option I’ve found to achieve that result, but there might be better ones. So, before implementing the communication interfaces, I’ll need to research again, I think.

*** 
IMPORTANT TAKE ABOUT FRONT-END SERVER: THEY CONTAIN A PART OF THE CORE: THE PROJECTIONS (passively  cached current/updated aggregate instances, that is, the updated data models (instances) with which each front-end node has frequently interacted or interacts)
***

Let's see a simple flow of a Client - Server interaction. 
Consider that the 'client' is a front-end server (ssr) that interacts through a graphql api with the core of the system (the 'server') asynchronously, without blocking the user's UI (perhaps using a feature like React Suspense or similar)

*** 
WARNING: The flow described below does not incorporate  the management of errors both in the internal and external distribution of the events produced by the Command-Handler.This is an issue that will be discussed later in detail, but in sishesis, the errors that can occur in the distribution process of an event 
They are treated as follows:

1-Through Timeout on the client's side and also on the event bus, which is sent the incoming event to a Dead-Leet Queue in the case that problems occur in the distribution of the new event from the Command-Handler to the data bus (If the new event does not reach all the Event-Handlers consumers). 
If the event was only received by some consumers, then a compensation event is generated that reverses the information added by the partially ditribuated event to consumers who received said event.
If the event was not received by any consumer, then there is no need to redistribute the event, however, it is important to generate a response to the webserver client that the command that was requested could only be validated, but not processed.

2 - In the event that the new event is received by all consumers, but some of them have a failure in the process of notification to the client webserver, the client must be notified that a problem in the internal distribution of the event generated by the event generated by theCommand, but tried to trigger the distribution of the generated event, including in this notification the Timeout that the client must wait to erronely give the process.There are more strategies, but these are the generic ones.
***

Happy path of a generic client-server domain aggregate flow. An SSR server (the 'client') send a command to an aggregate (the 'server') through their grapql api. 
Pseudo Code:
```
# Proccess: create or modify an instance of a (generic) domain aggregate by send a command.

# AWS event flow (Only Happy path. Error paths have a dead-letter Queue that handle and notify errors to clients):
Webserver[(readCachedProjectionInstanceIfExist || getProjectionInsanceFromAppsyncApiQuery) && buildAndSendCommand] 
<-> Appsync 
<-> Lambda 
-> Event bridge 
-> Lambda (saveEventInDynamoDb && notifyWebserverClients(s)ThroughAppsyncApi) 
-> Appsync 
-> Webserver[(updateOrCreateCachedProjection && notifyClient)] 

# We need types that reprecent the responses
# in a functional way (since lambdas are functions):

enum CommandHandlerResponse<T, E> {
    Success(T),
    Error(E),
}
enum QueryHandlerResponse<T, E> {
    Some(T),
    Error(E),
}

# At basic level, the aggregate projection ID (the current state of a domain aggregate instance)
# is composed by the ID of the instance plus the version (serial number) of the projection.
# The version change every time that a new event is generated for that aggregate instance (is a correlative number) 
# This composition is 'Maybe' (maybe type) because 'createNewIntance' commands have not id and version yet.
# So:

type aggProjectionID = Maybe({ID, version}) 

type commandParams = [...inputParams, aggProjectionID] 

# Each container described below is a phisical (or virtual) component: 
# Client is an SSR server, 
# Event Bus is the internal-global (at back-end level) comunication channel, 
# The other ones are part of a domain aggregate (lambda functions).

# then (pseudo code flow):

Client[ callApiAndWait( doSomethingCommand(commandParams)) >..waiting..> onCommandRespond(Success(aggProjectionID)) >> queryAggerateByIdAndWait(aggProjectionID) >..waiting..> ] 
                        ↓                                                              ↑
Api[ doSomethingCommand(commandParams) >> implicitSendCommandAndWait(commandParams) && respondToWaitingClient(Success(aggProjectionID))] 
                        ↓                                                              ↑
Command_Handler[ validate_rules(commandParams) >>  emmit_to_bus(newEventCreated) && respond_to_api_with_ID(Success(aggProjectionID)) ] 
                        ↓
Evento_Bus[ delivers(newEventCreated) ] 
                        ↓
Event_Handler[ capture_and_store(newEventCreated) >> rebuild_new_aggregate_instance() >>  publish_to_api( onAggregateEventEmited(newAggregateInstanceState)) ] 
                        ↓
Api[ onAggregateEventEmited( passEventToApiQueryHandler(Some(newAggregateInsanceState)) ) ]
                        ↓
Client[ >..waiting..> onQueryByIdResponds(Some(newAggregateInstanceState)) >>  cachOrUpdatAggregateInstanceProjectionAndAnyOtherSSRprocess(newAggregateInstanceState) && updateOrNotifyUserUI(newAggregateInstanceState) ]
```

.. well, it seems like a long and slow proccess just for a simple interaction... 
.. I can say you that meybe not :)


Considerations in front-end/back-end, communication, components and eventual consistency:

To recap: 

Front-end-servers must communicate via Graphql APIs (websockets under the hood) with domain aggregates (core-servers). The reasons I already mentioned above, the most important being: the ability of Graphql APIs to function as pub-sub brokers at the API level in a simple way and without additional components, which is essential if you want to achieve secure asynchronous communication. , fast, and without redundant requests (... without webhooks). 

The second important consideration is that front-end-servers should also communicate with UX/UI clients via websockets and, if possible, also in terms of commands and queries. 

Another consideration is that strategies could be implemented to cache within the context of the SSRs the projections (“current states”) of the recently requested domain aggregate instances, in this way redundant connections and queries to the cores could be avoided. servers. Although there may be trade-offs in adopting this strategy on a global level... but since domain aggregates live in different contexts, separated by well-defined contextual boundaries, each domain can implement a different caching strategy, tailored to the specific needs of that domain. . 

And the same thing happens with many other technical aspects: various technical / infrastructure parameters at the Core level can (and should) be adapted to each domain aggregate, but it is valid to start with something generic. However, it DOES NOT APPLY to the general technical decisions, patterns, definitions and conventions that are part of the architecture on which this type of system is based.

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

... wriiting, fixing and rewriting...  

Some diagrams will be added soon, work in proggress

I'm currently fixing and rewriting a lot, synthesizing some ideas, and will update this document as I can. The most interesting part comes when you realize the benefits it gives you in distributed systems... and the frequent data caching on the SSR side in the most efficient and simple way... I'm excited.


This is project build with AWS CDK TypeScript.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `npx cdk deploy`  deploy this stack to your default AWS account/region
* `npx cdk diff`    compare deployed stack with current state
* `npx cdk synth`   emits the synthesized CloudFormation template
