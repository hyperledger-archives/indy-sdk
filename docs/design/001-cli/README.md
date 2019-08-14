# Indy CLI Design

## Re-implementation of CLI
This proposal follow the idea to re-implement CLI from scratch. Main reasons are:
* Existing code base is written in hard-to support way.
* Existing code base is too far from libindy entities model.
* Existing code base requires complex runtime (python) and additional dependencies (python libindy wrapper) that complicates deployment.
* It is just cheaper to re-implement CLI than to perform deep refactoring.

## Use Rust language
We propose to re-implement CLI in Rust. Main reasons are:
* Main libindy code base uses Rust. Team has deep Rust experience.
* No need to big runtime, small list of dependencies. As result simple packaging and deployment.
* Rust is nice and reliable cross-platform solution for native apps.

## Terminal input-output handling
To handle complex terminal input, history and autocompletion support on different terminals linefeed crate will be used (few additional alternatives are also available). To handle colored terminal output ansi_term crate will be used.

## Auto completion
The following autocompletion will be provided through readline infrastructure:
* Command group name completion
* Command name completion
* Command param name completion

## Libindy wrapper
CLI project will contain simple synchronous libindy wrapper:
* Code from libindy tests that provides similar wrapper will be partially reused.
* Synchronization will be performed through Rust channels:
  * Main thread creates channel and closure that will send message to this channel.
  * Calls libindy and puts this closure as callback.
  * Blocks on reading from channel.

As channel reading assumes timeouts it will be possible to emulate progress updating .

## Threading model
There will be one main thread that will perform io operations with terminal and libindy calls synchronized through Rust’s channel. Blocking will be limited by small channel read timeout.

## Execution modes
CLI will support 2 execution modes:
* Interactive. In this mode CLI will read commands from terminal interactively.
* Batch. In this code all commands will be read from file or pipe and executed in series.

## Code structure
* CLI code will define "Command" structure that will be container for:
  * Command meta information (name, command help, supported params, params help)
  * "Executor" function that will contain command execution logic
  * "Cleaner" function that will contain command cleanup logic
* Each command will be implemented as Rust module with one public "new" function that returns configured "Command" instance
* All commands will share one "CommandContext". "CommandContext" will hold application state and contain 2 parts:
  * Pre-defined application part. Part that holds application-level state like command prompt, isExit flag and etc...
  * Generic command specific part. This part will be key-value storage that will allow commands to store command-specific data like Indy SDK handles, used DID and etc... 
* "Executor" and "Cleaner" functions will get CommandContext as parameter
* Actual execution of commands will be performed by CommandExecutor class. This class will:
  * Instantiation of shared "CommandContext"
  * Hold all commands and command grouping info
  * Parse command lines according to commands meta information and search for relevant command
  * Triggering of command execution
  * Provide line auto completion logic
  * Triggerid of command cleanup
Command instances will optionally share few contexts:
* EntryPoint will:
* Instantiate CommandExecutor and provide commands to command executor instance.
* Determine execution mode.
  * In interactive mode it will start readline cycle and execute each line with CommandExecutor until Exit command received.
  * In batch mode it will execute each command in the list and finish execution after completion of all commands.

See diagram:

<img src="./cli-components.svg">

## Commands
Command format
```
indy> [<group>] <command> [[<main_param_name>=]<main_param_value>] [<param_name1>=<param_value1>] ... [<param_nameN>=<param_valueN>]
```
### Common commands

#### Help
Print list of groups with group help:
```
indy> help
```
Print list of group commands with command help:
```
indy> <group> help
```
Print command help, list of command param and help for each param:
```
indy> <group> <command> help
```

#### About
Print about and license info:
```
indy> about
```

#### Exit
Exit from CLI:
```
indy> exit
```

#### Prompt
Change command prompt:
```
indy> prompt <new_prompt>
```

#### Show
Print content of file:
```
indy> show [<file_path>
```

#### Load Plugin
Load plugin in Libindy:
```
indy> load-plugin library=<name/path> initializer=<init_function>
```

### Wallets management commands (wallet group)
```
indy> wallet <command>
```

#### Wallet create
Create new wallet and attach to Indy CLI:
```
indy> wallet create <wallet name> key [key_derivation_method=<key_derivation_method>] [storage_type=<storage_type>] [storage_config={config json}]
```
TODO: Think about custom wallet types support. Now we force default wallet security model.. 

#### Wallet attach
Attach existing wallet to Indy CLI:
```
indy> wallet attach <wallet name> [storage_type=<storage_type>] [storage_config={config json}]
```

#### Wallet open
Open the wallet with specified name and make it available for commands that require wallet. If there was opened wallet it will be closed:
```
indy> wallet open <wallet name> key [key_derivation_method=<key_derivation_method>] [rekey] [rekey_derivation_method=<rekey_derivation_method>]
```

#### Wallet close
Close the opened wallet
```
indy> wallet close
```

#### Wallet delete
Delete the wallet
```
indy> wallet delete <wallet name> key [key_derivation_method=<key_derivation_method>]
```

#### Wallet detach
Detach wallet from Indy CLI
```
indy> wallet detach <wallet name>
```

#### Wallet list
List all attached wallets with corresponded status (indicates opened one):
```
indy> wallet list
```

### Export wallet
Exports opened wallet to the specified file.

```indy-cli
indy> wallet export export_path=<path-to-file> export_key=[<export key>] [export_key_derivation_method=<export_key_derivation_method>]
```

### Import wallet
Create new wallet and then import content from the specified file.

```indy-cli
indy> wallet import <wallet name> key=<key> [key_derivation_method=<key_derivation_method>] export_path=<path-to-file> export_key=<key used for export>  [storage_type=<storage_type>] [storage_config={config json}]
```

### Pool management commands
```
indy> pool <subcommand>
```

#### Create config
Create name pool (network) configuration
```
indy> pool create [name=]<pool name> gen_txn_file=<gen txn file path> 
```

#### Connect
Connect to Indy nodes pool and make it available for operation that require pool access. If there was pool connection it will be disconnected.
```
indy> pool connect [name=]<pool name> [protocol-version=<version>] [timeout=<timeout>] [extended-timeout=<timeout>] [pre-ordered-nodes=<node names>]
```

#### Refresh
Refresh a local copy of a pool ledger and updates pool nodes connections.
```
indy> pool refresh
```

#### Disconnect
Disconnect from Indy nodes pool
```
indy> pool disconnect
```

#### List
List all created pools configurations with status (indicates connected one)
```
indy> pool list
```

### Identity Management
```
indy> did <subcommand>
```

#### New
Create and store my DID in the opened wallet. Requires opened wallet.
```
indy> did new [did=<did>] [seed=<UTF-8, base64 or hex string>] [metadata=<metadata string>]
```

#### List
List my DIDs stored in the opened wallet as table (did, verkey, metadata). Requires wallet to be opened.:
```
indy> did list
```

#### Use
Use the DID as identity owner for commands that require identity owner:
```
indy> did use [did=]<did>
```

#### Rotate key
Rotate keys for used DID. Sends NYM to the ledger with updated keys. Requires opened wallet and connection to pool:
```
indy> did rotate-key [seed=<UTF-8, base64 or hex string>] [fees_inputs=<source-1,..,source-n>] [fees_outputs=(<recipient-1>,<amount>),..,(<recipient-n>,<amount>)] [extra=<extra>]
```

### Ledger transactions/messages
```
indy> ledger <subcommand>
```

#### NYM transaction
Send NYM transaction
```
ledger nym did=<did-value> [verkey=<verkey-value>] [role=<role-value>] [source_payment_address=<source_payment_address-value>] [fee=<fee-value>] [fees_inputs=<source-1,..,source-n>] [fees_outputs=(<recipient-1>,<amount>),..,(<recipient-n>,<amount>)] [extra=<extra>] [sign=<true or false>] [send=<true or false>] [endorser=<endorser did>]
```

#### GET_NYM transaction
Send GET_NYM transaction
```
ledger get-nym did=<did-value> [send=<true or false>]
```

#### ATTRIB transaction
Send ATTRIB transaction
```
ledger attrib did=<did-value> [hash=<hash-value>] [raw=<raw-value>] [enc=<enc-value>] [source_payment_address=<source_payment_address-value>] [fee=<fee-value>] [fees_inputs=<source-1,..,source-n>] [fees_outputs=(<recipient-1>,<amount>),..,(<recipient-n>,<amount>)] [extra=<extra>] [sign=<true or false>]  [send=<true or false>] [endorser=<endorser did>]
```

#### GET_ATTRIB transaction
Send GET_ATTRIB transaction
```
ledger get-attrib did=<did-value> [raw=<raw-value>] [hash=<hash-value>] [enc=<enc-value>] [send=<true or false>]
```

#### SCHEMA transaction
Send SCHEMA transaction
```
ledger schema name=<name-value> version=<version-value> attr_names=<attr_names-value> [source_payment_address=<source_payment_address-value>] [fee=<fee-value>] [fees_inputs=<source-1,..,source-n>] [fees_outputs=(<recipient-1>,<amount>),..,(<recipient-n>,<amount>)] [extra=<extra>] [sign=<true or false>]  [send=<true or false>] [endorser=<endorser did>]
```

#### GET_SCHEMA transaction
```
ledger get-schema did=<did-value> name=<name-value> version=<version-value> [send=<true or false>]
```

#### CRED_DEF transaction
Send CRED_DEF transaction
```
ledger cred-def schema_id=<schema_id-value> signature_type=<signature_type-value> [tag=<tag>] primary=<primary-value> [revocation=<revocation-value>] [source_payment_address=<source_payment_address-value>] [fee=<fee-value>] [fees_inputs=<source-1,..,source-n>] [fees_outputs=(<recipient-1>,<amount>),..,(<recipient-n>,<amount>)] [extra=<extra>] [sign=<true or false>]  [send=<true or false>] [endorser=<endorser did>]
```

#### GET_CRED_DEF transaction
Send GET_CRED_DEF transaction
```
ledger get-cred-def schema_id=<schema_id-value> signature_type=<signature_type-value> origin=<origin-value> [send=<true or false>]
```

#### NODE transaction
Send NODE transaction
```
ledger node target=<target-value> alias=<alias-value> [node_ip=<node_ip-value>] [node_port=<node_port-value>] [client_ip=<client_ip-value>] [client_port=<client_port-value>] [blskey=<blskey-value>] [blskey_pop=<blskey-proof-of-possession>] [services=<services-value>] [sign=<true or false>]  [send=<true or false>]
```

#### GET_VALIDATOR_INFO transaction
Send GET_VALIDATOR_INFO transaction to get info from all nodes
```
ledger get-validator-info [nodes=<node names>] [timeout=<timeout>]
```

#### POOL_UPGRADE transaction
Send POOL_UPGRADE transaction
```
ledger pool-upgrade name=<name> version=<version> action=<start or cancel> sha256=<sha256> [timeout=<timeout>] [schedule=<schedule>] [justification=<justification>] [reinstall=<true or false (default false)>] [force=<true or false (default false)>] [package=<package>] [sign=<true or false>]  [send=<true or false>]
```

#### POOL_CONFIG transaction
Send POOL_CONFIG transaction
```
ledger pool-config writes=<true or false (default false)> [force=<true or false (default false)>] [sign=<true or false>]  [send=<true or false>]
```

#### POOL_RESTART transaction
Send POOL_RESTART transaction
```
ledger pool-restart action=<start or cancel> [datetime=<datetime>] [nodes=<node names>] [timeout=<timeout>]
```

#### Custom transaction
Send custom transaction with user defined json body and optional signature
```
ledger custom [txn=]<txn-json-value> [sign=<true|false>]
```

#### AUTH_RULE transaction
Send AUTH_RULE transaction
```
ledger auth-rule txn_type=<txn type> action=<add or edit> field=<txn field> [old_value=<value>] [new_value=<new_value>] constraint=<{constraint json}> [sign=<true or false>]  [send=<true or false>]
```

#### GET_AUTH_RULE transaction
Send GET_AUTH_RULE transaction
```
ledger get-auth-rule [txn_type=<txn type>] [action=<ADD or EDIT>] [field=<txn field>] [old_value=<value>] [new_value=<new_value>] [send=<true or false>]
```

#### GET_PAYMENT_SOURCES transaction
Send GET_PAYMENT_SOURCES transaction
```
ledger get-payment-sources payment_address=<payment_address> [send=<true or false>]
```

#### PAYMENT transaction
Send PAYMENT transaction
```
ledger payment [source_payment_address=<payment address>] [target_payment_address=<payment address>] [amount=<number>] [fee=<transaction fee amount>] [inputs=<source-1>,..,<source-n>] [outputs=(<recipient-1>,<amount>),..,(<recipient-n>,<amount>)] [extra=<extra>] [sign=<true or false>]  [send=<true or false>]
```

#### GET_FEES transaction
Send GET_FEES transaction
```
ledger get-fees payment_method=<payment_method> [send=<true or false>]
```

#### MINT transaction
Prepare MINT transaction
```
ledger mint-prepare outputs=(<recipient-1>,<amount>),..,(<recipient-n>,<amount>) [extra=<extra>]
```

#### SET_FEES transaction
Prepare SET_FEES transaction
```
ledger set-fees-prepare payment_method=<payment_method> fees=<txn-type-1>:<amount-1>,..,<txn-type-n>:<amount-n>
```

#### VERIFY_PAYMENT_RECEIPT transaction
Prepare VERIFY_PAYMENT_RECEIPT transaction
```
ledger verify-payment-receipt <receipt> [send=<true or false>]
```

#### Add multi signature to transaction
Add multi signature by current DID to transaction
```
ledger sign-multi txn=<txn_json>
```

#### Save transaction to a file.
Save stored into CLI context transaction to a file.
```
ledger save-transaction file=<path to file>
```

#### Load transaction from a file.
Read transaction from a file and store it into CLI context.
```
ledger load-transaction file=<path to file>
```

#### TXN_AUTHR_AGRMT transaction.
Request to add a new version of Transaction Author Agreement to the ledger.
```
ledger ledger txn-author-agreement [text=<agreement content>] [file=<file with agreement>] version=<version> [source_payment_address=<source_payment_address-value>] [fee=<fee-value>] [fees_inputs=<source-1,..,source-n>] [fees_outputs=(<recipient-1>,<amount>),..,(<recipient-n>,<amount>)] [extra=<extra>] [sign=<true or false>]  [send=<true or false>]
```

#### SET_TXN_AUTHR_AGRMT_AML transaction.
Request to add new acceptance mechanisms for transaction author agreement.
```
ledger txn-acceptance-mechanisms [aml=<acceptance mechanisms>] [file=<file with acceptance mechanisms>] version=<version> [context=<some context>] [source_payment_address=<source_payment_address-value>] [fee=<fee-value>] [fees_inputs=<source-1,..,source-n>] [fees_outputs=(<recipient-1>,<amount>),..,(<recipient-n>,<amount>)] [extra=<extra>] [sign=<true or false>]  [send=<true or false>]
```

### Payment Address commands
```
indy> payment-address <subcommand>
```

#### Create
Create the payment address for specified payment method. Requires opened wallet.
```
payment-address create payment_method=<payment_method> [seed=<seed-value>]
```

#### List
Lists all payment addresses. Requires opened wallet.
```
payment-address list
```

#### Sign
Create a proof of payment address control by signing an input and producing a signature.
```
payment-address sign address=<payment_address> input=<string to sign>
```

#### Verify
Verify a proof of payment address control by verifying a signature.
```
payment-address verify address=<payment_address> input=<signed string> signature=<signature>
```

## Examples

#### Create pool configuration and connect to pool
```
indy> pool create sandbox gen_txn_file=/etc/sovrin/sandbox.txn
indy> pool connect sandbox
pool(sandbox):indy> pool list
```

#### Create and open wallet
```
sandbox|indy> wallet create alice_wallet key
sandbox|indy> wallet open alice_wallet key
pool(sandbox):wallet(alice_wallet):indy> wallet list
```

#### Create DID in the wallet from seed and use it for the next commands
```
pool(sandbox):wallet(alice_wallet):indy> did new seed=SEED0000000000000000000000000001 metadata="Alice DID"
pool(sandbox):wallet(alice_wallet):indy> did use MYDID000000000000000000001
pool(sandbox):wallet(alice_wallet):did(MYD...001):indy> did list
```

#### Create new DID for BOB
```
pool(sandbox):wallet(alice_wallet):did(MYD...001):indy> did new metadata="Bob DID"
```

#### Post new NYM to the ledger
```
pool(sandbox):wallet(alice_wallet):did(MYD...001):indy> ledger nym did=MYDID000000000000000000001
```

#### Send GET_NYM transaction
```
pool(sandbox):wallet(alice_wallet):did(MYD...001):indy> ledger get-nym did=MYDID000000000000000000001
```
