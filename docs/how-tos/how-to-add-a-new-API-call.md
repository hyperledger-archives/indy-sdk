# How to add a new API call (code walkthrough)

In this guide you will see how to add a new call to Libindy. As an example we will take `indy_create_and_store_my_did` call.

Code will be splitted to the following layers:
* API layer. Library interface layer: 
    * Basic validation
    * Conversion of C types to Rust types
    * Propagation of execution to commands layer
* Commands layer:
    * Working threads management
    * JSONs conversion to internal types and corresponded validation
    * Splitting complex commands to atomic operations
    * Propagation of atomic operations execution to service layer
    * Joining atomic operations results to complex result
    * Execution of user defined callbacks
* Service layer:
    * Implements operations business logic and complex validation
    * Management of sockets polling threads


### API Layer

These changes should be made in directory [`libindy/src/api`](libindy/src/api) in file `did.rs`. For your function you should choose which file it will logically fit or create a new one (first option is preferred)

#### Call definition:
```rust
#[no_mangle]
pub extern fn indy_create_and_store_my_did(command_handle: CommandHandle,
                                           wallet_handle: WalletHandle,
                                           did_info: *const c_char,
                                           cb: Option<extern fn(command_handle_: CommandHandle,
                                                                err: ErrorCode,
                                                                did: *const c_char,
                                                                verkey: *const c_char)>) -> ErrorCode {
    // this will be done later
}
```
What you can modify here is function name and params. 

There is only one strict rule for the function name -- it should have prefix `indy_` to avoid collisions with other libraries in system.

Rules for params:
1) First parameter is almost always `command_handle` -- you will need this param if your call will work asynchronously -- you will need it most of the times.
2) After the command handle list all of the params that you need to pass. Please, don't use `Option` on the types other than `extern fn`.
3) The last parameter is almost always `cb` -- it is a callback. Its first param should be `command_handle_` (for your app to identify which call this callback came for), then `err` for the error code and then all the values that you call is going to return.
4) Please, use the types that were defined in `libindy/src/api/mod.rs` such as `WalletHandle`, `PoolHandle` and so on for the types of your parameters.

#### Parameter checks:

```rust
trace!("indy_create_and_store_my_did: >>> wallet_handle: {:?}, did_json: {:?}", wallet_handle, did_info);

check_useful_json!(did_info, ErrorCode::CommonInvalidParam3, MyDidInfo); // redefine to MyDidInfo if valid
check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

trace!("indy_create_and_store_my_did: entities >>> wallet_handle: {:?}, did_json: {:?}", wallet_handle, secret!(&did_info));
```

For most C-types that need to be converted to Rust types Libindy has a set of macros that will check and convert it if everything is good or throw specified error code otherwise. You can see these macros in [`libindy\src\utils\ctypes.rs`](libindy\src\utils\ctypes.rs)

### Passing command to Command Thread

#### Adding new command

For that you should go to [`libindy/src/commands`](libindy/src/commands) directory and choose a module that fits your call. Most of the times it matches by the name to the module from the previous section. In our case it is `did.rs`.

Here you will find an enum named like `SomeCommand`, `DidCommand` in our case. Here you have to add a new variant. In our case it will look like this:

```rust
CreateAndStoreMyDid(
    WalletHandle,
    MyDidInfo, // my did info
    Box<Fn(IndyResult<(String, String)>) + Send>),
```

It has params that you have passed to the function and a callback to return values.

#### Sending the command

In our API call we need to add some code to pass the command we just created:
```rust
let result = CommandExecutor::instance()
    .send(Command::Did(DidCommand::CreateAndStoreMyDid(
        wallet_handle,
        did_info,
        Box::new(move |result| {
            // here goes a call to the callback that you have passed
        }),
    )));
```
`CommandExecutor` is a holder of a channel to communicate with the Command Thread.

#### Receiving the command

In a module with commands you will find a struct named like `SomeCommandExecutor`. In our case it will be `DidCommandExecutor`. 
Here you should add a new function for your business logic and a new match clause to the function `execute`. In our case it will look like this:

```rust
    pub fn execute(&self, command: DidCommand) {
        match command {
            // some other clauses
            DidCommand::CreateAndStoreMyDid(wallet_handle, my_did_info, cb) => {
                info!("CreateAndStoreMyDid command received");
                cb(self.create_and_store_my_did(wallet_handle, &my_did_info));
            }
        }
    }
    // some other functions
    fn create_and_store_my_did(&self,
                               wallet_handle: WalletHandle,
                               my_did_info: &MyDidInfo) -> IndyResult<(String, String)> {
        debug!("create_and_store_my_did >>> wallet_handle: {:?}, my_did_info_json: {:?}", wallet_handle, secret!(my_did_info));

        let (did, key) = self.crypto_service.create_my_did(&my_did_info)?;

        if self.wallet_service.record_exists::<Did>(wallet_handle, &did.did)? {
            return Err(err_msg(IndyErrorKind::DIDAlreadyExists, did.did));
        };

        self.wallet_service.add_indy_object(wallet_handle, &did.did, &did, &HashMap::new())?;
        self.wallet_service.add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new())?;

        let res = (did.did, did.verkey);

        debug!("create_and_store_my_did <<< res: {:?}", res);

        Ok(res)
    }
```
In the function you should put business logic of your call.
Notice, that if you have some functionality that can be reused later by other commands, you should put it into the service and execute service call in the function.

Services should stay independent from each other. You can include Services into CommandExecutors.  