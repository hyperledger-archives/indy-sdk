# Actor model
The project is based on Actix - actor framework for Rust. More info about general idea of actors 
[here as video](https://www.youtube.com/watch?v=lPTqcecwkJg) 
or [here as a text](https://doc.akka.io/docs/akka/2.5.3/scala/general/index.html). Lot of information about actors
is available in regard to Akka, actor framework for JVM, but the general concepts are valid for Rust's Actix as well.

When you start looking into the code, you'll be mostly interested in code located at `src/actors`. These files define
several different actors within the agency. 

## Messages everywhere!
There's few kinds:
- the way to interact with agency from outside is by sending "messages" via a transport protocol, in case of this agency
it is mostly HTTP POST Requests with binary encoding.
- That binary data is considered a message which will be somehow processed. Note though that a message can contain 
another nested message (like an envelope in an envelope, but in this case each envelope represents some encrypted data).
- Actix framework itself is communicating via "messages". Actor framework is a lot like OOP, but the entities are not 
calling each other's methods, but sending messages. So instead of `router.router(foo=1, bar=2)` you would basically do 
something more like `dispatchMessage(router, Message {foo:1, bar:2})`.

## Protocols
The point of cloud agency is enabling asynchronous communication between agency which might not be always online and/or
have public IP address. 
- These agents talk to each other using certain protocol - for example protocol to send/receive verifiable credentials
 and proofs based on them. 
- But these agents need also a protocol for communication with the agency itself. Dummy Cloud Agency implements agency
protocol implemented by LibVCX library. Such protocol specified how to download messages or manage connections between
your cloud agent and other entities of outside word. 

# The actors
Let's go trough the agents within the agency and their roles. 

## Forward agent
This is sort of core agent of the agency and has it's own wallet. Let's call this `Forward Agent`'s wallet. 
Every message coming into the agency have to first pass through  (and be processed by) forward 
agent `forward_agent.rs`. Forward agent is basically represents identity of the agency
itself. Agency has it's own keypair (you can specify that in `"forward_agent":` section of configuration file) and 
every message coming through the agency (regardless of who the recipient is) must be anoncrypted by agency's keypair. 
So when a message arrives to agency, it first flows to forward agent`which anondecrypts the message. 

Next, the message is passed to Router actor `router.rs`.

## Router
Router is Actor, which as name suggests, routes messages. It can process few types of messages, which basically gives
Router interaction interface for following operations.
- Register Agent did route
- Register Agent Connection route
- Route a message 

In order for router to be able route messages anywhere, other actors (such as `Forward Agent`, `Agent Connection`, 
`Forward agent Connection`) must first register their DIDs which is like them saying "Hey Router, when you get
a message for DID=123456, that's me! Forward it to me please."  And soo when Router later gets message to route, 
the message must contain `fwd` field with value if recipient's DID.

## Forward agent connection
Let's get back to Forward Agent for a second. Apart from the mundane anondecryption Forward Agent performs, 
it can also handle 1 type of Agency protocol message - `Connect`. Anytime an agent from outer world wants to 
start interacting with agency, it first needs to establish connection with agency. That means the agency and 
the outer agent have to exchange public keys used uniquely for encryption and identification of the relationship 
between them. That is accommodated via the `Connect` agency protocol message.

Each forward agent connection represents connection with someone and it's Forward Agent Connection has its own DID.
This DID is registered in `Router`. Each forward agent connection is stored in `Forward Agent`'s wallet as pair of DIDs
with metadata.

Forward Agent Connection actor itself can handle just 2 types of agency protocol messages `CreateAgent` and `SingUp`.
These are usually called subsequently to upgrade your "a connection with the agency" to a full blown cloud agent inside
cloud agency. `CreateAgent` creates you `Agent` in the cloud agency.

## Agent
Agent is actor representing your cloud agent. It has its own DID and keypair stored in its own wallet. The wallet will
is later also used to manage pairwise connection between your agent and other agents. 
Agent has many capabilities - following LibVCX agency protocol, you can create new connections with other agents, 
requests messages from established connections, configure your agent and more. Particularly important agency protocol 
message type is `CreateKey` which as result create new actor `Agent Connection`. 
 
## Agent connection
Agent connection is representation of single pairwise relationship of particular `Agent`. The state of this actor
includes data such a connection status and list of messages exchanged within this relationship.

## Requester
This is "singleton", almost stateless actor which does not need to persist any state. Requester is solely used to 
forward messages to different cloud agencies.

 
 