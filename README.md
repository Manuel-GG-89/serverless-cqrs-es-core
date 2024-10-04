This is just a prof of concept in EARLY... early state. 
and, yes.. for the time being, fully coupled to the AWS platform.

note 1: The first sketches and proofs of concepts were developed in Typescript/Python... 
      ...now rewriting same things in Rust (just the functions) 
      
note 2: Front-end servers are cont considered on this project yet.. (But some serverless Node like framework hosted on Cloudflare will be considered in the future.

note 3: The next proyect: To decopule from AWS, try the same goal with other lowcost cloud providers like Cloudflare.


Objetive: 
  - To achive the most simple and lowcost way to mount a Serverless, Event Driven, Event Sourced CQRS core service on AWS
  - Done by making three / four common (almost generics) decopuled Domain Aggregates that cover most common cases of interactions
  - Without using frameworks, just AWS CDK.
  - No CDK meta programing or templating magic.. Aggregate1 -> aggregate1stack.ts, Aggregate2 -> Copy/Paste and Modify ggregate1stack.ts, and so.
  - Main goal is build a starting point of an architecture that allows to implement Domain Driven Design in an a little easier way.

Challenge stuffs (outside the obvious things):
  - Outside async comunication via websockets (to front-end servers / functions. Never directly to client sides apps)
  - To reduce Coltdown Start times to the minimum possible... (currently using simplest rust functions..  minimal coldown start time)
  - Security layers / Authentication / Authorization.
  - Keep it simple.

Resources, tools and technical stuffs:
  - AWS CDK to declare and configure all resources (default typescript cdk version)
  - No statefull servers, just serverless functions
  - No Kafka or any other similar.
  - Internal communication through EventBridge data bus (not sqs/sns pub-sub pattern, delivery rules configured on the bus)
  - Step functions in case needs to handle transactions (saga pattern) (flow nor even designed yet ...)
  - No relational database. Just Dynamodb as an events store (historical events, no mutatios)
  - No http/rest apis, no webhooks. Graphql (Appsync) acting like pub-sub midleware to expose commands (async operations) and handle (simple) querys from front-end servers
  - Rust lambda runtimes to reduce Coltdown Start at minimal.
  - Shared Type Schemmas between Domain Aggregates for Commands, Events and Querys when necesary at dev cycle or  compilation/build time.

  Basic Composition. A Domain Aggregate have, at least: 

      - A Command Handler with a Rules/Policies validator 
      - An Event Handler that just save events emited from (self and/or external) Commands Handlers
      - An Event Store to store the events
      - A Query handler that make posible do querys and get states from one o more aggregate instances to the Event Store (Dynamodb) and makes Querys.
      - A simple reducer function shared as a lib between Command and Query handlers to fold(reduce) and rebuild the actual state of the agregatereading the event store (historical) events
      - A set of delivery rules (configured on the event bus) to indicate the destinations of each event.
      - An async api.


This is a blank project for CDK development with TypeScript.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `npx cdk deploy`  deploy this stack to your default AWS account/region
* `npx cdk diff`    compare deployed stack with current state
* `npx cdk synth`   emits the synthesized CloudFormation template
