# How to add a new API call (code walkthrough)

In this guide you will see how to add a new call to Libindy. As an example we will take `indy_create_and_store_my_did` call.

Code will be splitted to the following layers:
* API layer - enter point to the library 
* Commands layer - split complex operation into multiply atomic ones, call various services for atomic actions and join results from them
* Service layer - isolated against each-other services, process atomic operations

For more details please see [Layers of libindy description](../architecture/libindy_layers.md)

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
                debug!("CreateAndStoreMyDid command received");
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

### Testing

One of the general principles of development within the core Indy team is to use [TDD](http://www.agiledata.org/essays/tdd.html)

##### Unit test

All functions within `services` and `utils` modules should be covered with Unit tests (as we mentioned above these functions must be atomic operations). 
So, if we consider function `create_and_store_my_did` the following service functions should be covered with Unit tests:
* `self.crypto_service.create_my_did`
* `self.wallet_service.record_exists`
* `self.wallet_service.add_indy_object`

We don't cover the functions within the `commands` module with Unit tests because:
*  `CommandExecutor` is a complex structure which has multiple dependencies. 
*  These functions only join results from multiple atomic service-related functions.
*  These functions totally correspond to an API level function which sends associated command to the `CommandExecutor`.

##### Integration test

Each Libindy external API function (within `api` module) should be covered with Integration tests.
* These tests live within the `tests` directory. 
* There are two usage types:
    * use C function definition (like tests located at `demo` file).
    * use Rust wrapper to avoid boilerplate like `channel` preparation and casting to C types.
* Integration tests based on Rust wrapper is divided on High level and Medium level.
    * High cases - typical positive scenarios or a strongly specific error related to a function.
    * Medium cases - tricky positive scenarios or general errors like invalid input data or invalid handles. 
* there are the set of integration tests devoted to one specific API function.
* there are the set of integration tests which covers different complex scenarios like `interaction` or `anoncreds_demos`.

These integration tests can be a good example of how a function should be used.

If we consider function `indy_create_and_store_my_did`:
 * There are multiple High and Medium tests cases file devoted to that function within `did.rs`.
 * There is also `ledger_demo_works` test in the `demo.rs` file which uses C definition of `indy_create_and_store_my_did` function.

For more details around testing, checkout out this [doc](/docs/contributors/test-design.md).