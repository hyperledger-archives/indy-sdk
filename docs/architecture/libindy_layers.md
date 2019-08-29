There are 3 layers in libindy code organization:
* API layer
* Commands Executors layer
* Services layer

Each of them contains multiply modules devoted to particular part of the flow, like "wallet", "pool", "crypto", etc. 

## Layers purposes

### API layer

* Library interface layer
* Basic validation
* Conversion of C types to Rust types
* Propagation of execution to commands layer

### Commands Executors layer

* Working threads management
* JSONs conversion to internal types and corresponded validation
* Splitting complex commands to atomic operations
* Propagation of atomic operations execution to service layer
* Joining atomic operations results to complex result
* Execution of user defined callbacks

### Services layer

* Implements operations business logic and complex validation
* Management of sockets polling threads

## Layers interaction
### API -> API
There is no dependencies or calls between different API modules.

### API -> Commands Executor
Root Commands Executor contains "main" thread of the library. It accepts Commands from API via standard Rust mechanism `std::sync::mpsc::channel`.
A command contains user callback wrapped to Rust lambda. It will be called with a result of the processing.
Incoming commands are queued by `Reciver` in this channel synchronization.

### Command Executor X -> Command Executor Y
Commands Executors can communicate to each other using various Commands. These commands will be put into the same queue as incoming from API.

### Command Executor -> Services
Command Executor has access to all services required for the flow.
It calls public function of the services, gather partial results from them and aggregate the final result.
The final result is returned to the user via appropriate lambda.
Sometime service may have async API.
In this case Command Executor stores the lambda and wait "Ack" command from the service.

### Service -> Command Executor
In majority of cases services return result as the output of public functions from it.
For async responses "Ack" commands are used.

### Service -> Service
Services are completely isolated against each other, so no direct interaction between them is allowed. 

## Special units

### Pool Thread
It is used by Command Executors for CPU-heavy operations. The Command Executor prepare isolated data to be computed and pass a lambda to pool thread.
At the end of computation the lambda generates "Ack" command with a result.
While execution the lambdas Command Executors thread are free to process other commands.

### Domain
A set of structures definitions used while parsing incoming JSON from API of the library.