## Advanced getting fees for a request

This document contains the list of approaches to implementing a new function for getting request fee value.

Approaches:

1) The user prepares all data stored on the ledger and calls a function that returns an action fee amount.
Pros: 
    * explicit, 
    * single-purpose,
    * flexible, 
    * follows to the common Indy pattern - separate function for one independent action.  
    * allow caching
Cons: 
    * user need to prepare data stored on the ledger.

```
/// Gets request price correspondent to specific auth rule and
/// in case the requester can perform this action.
///
/// If the requester does not match to transaction auth rule, `TransactionNotAllowed` error will be thrown.
///
/// # Params
/// command_handle: Command handle to map callback to caller context.
/// get_auth_rule_response_json: response on GET_AUTH_RULE request.
/// requester_info_json: {
///     "role": string - role of a user which can sign transaction.
///     "count": string - count of users.
///     "is_owner": bool - if user is an owner of transaction.
/// }
/// fees_json: fees are set on the ledger.
///
/// # Return
/// price: tokens amount required for action performingю
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn indy_get_txn_price(command_handle: CommandHandle,
                                 get_auth_rule_response_json: *const c_char,
                                 requester_info_json: *const c_char,
                                 fees_json: *const c_char,
                                 cb: Option<extern fn(command_handle_: CommandHandle,
                                                      err: ErrorCode,
                                                      price: u64)>) -> ErrorCode {
.......
```
Example of usage:
```
get_auth_rule_req = indy_build_get_auth_rule_request(..)
get_auth_rule_res = indy_submit_request(get_auth_rule_req, ...)
get_fees_req = indy_build_payment_req(...)
get_fees_res = indy_submit_request(get_fees_req, ...)
fees = indy_parse_payment_response(get_fees_res, ...)
price = indy_parse_request_info(get_auth_rule_res, requester_info_json, fees)
```

2) Similar to the 1 option but returns json which contains action requirements in addition to the fee amount.
Pros: 
    * explicit
    * single-purpose
    * flexible
    * follows to the common Indy pattern - separate function for one independent action.  
Cons: 
    * user need to prepare data stored on the ledger.
    
Note: This approach is implemented in Libindy.

```
/// Gets request requirements (with minimal price) correspondent to specific auth rule and
/// in case the requester can perform this action.
///
/// If the requester does not match to transaction auth rule, `TransactionNotAllowed` error will be thrown.
///
/// # Params
/// command_handle: Command handle to map callback to caller context.
/// get_auth_rule_response_json: response on GET_AUTH_RULE request.
/// requester_info_json: {
///     "role": string - role of a user which can sign transaction.
///     "count": u64 - count of users.
///     "is_owner": bool - if user is an owner of transaction.
/// }
/// fees_json: fees are set on the ledger.
///
/// # Return
/// request_info_json: request info if a requester match to the action auth rule.
/// {
///     "price": u64 - tokens amount required for action performing,
///     "requirements": [{
///         "role": string - role of users who should sign,
///         "sig_count": u64 - count of signers,
///         "need_to_be_owner": bool - if requester need to be owner,
///     }]
/// }
///
#[no_mangle]
pub extern fn indy_get_request_info(command_handle: CommandHandle,
                                    get_auth_rule_response_json: *const c_char,
                                    requester_info_json: *const c_char,
                                    fees_json: *const c_char,
                                    cb: Option<extern fn(command_handle_: CommandHandle,
                                                       err: ErrorCode,
                                                       request_info_json: *const c_char)>) -> ErrorCode {
.......
```
Example of usage:
```
get_auth_rule_req = indy_build_get_auth_rule_request(..)
get_auth_rule_res = indy_submit_request(get_auth_rule_req, ...)
get_fees_req = indy_build_payment_req(...)
get_fees_res = indy_submit_request(get_fees_req, ...)
fees = indy_parse_payment_response(get_fees_res, ...)
req_info_json = indy_parse_request_info(get_auth_rule_res, requester_info_json, fees)
```

3) The user calls a function that automatically fetches the required information from the ledger (auth rule and fees):
Pros: 
    * automatically fetches the required information from the ledger.  
Cons: 
    * user need to define action
    * does a lot of intermediate steps internally that contradicts to Indy functions pattern
    * many input parameters
    
Note: This approach is implemented in Libvcx. The function definition is much simpler because Libvcx has the majority of these params in the context.  
    
```
/// Gets request price correspondent to specific auth rule and
/// in case the requester can perform this action.
///
/// # Params
/// command_handle: Command handle to map callback to caller context.
/// pool_handle: pool handle.
/// wallet_handle: wallet handle.
/// submitter_did: (Optional) DID of request sender
/// payment_method: payment method to use
/// action_json: {
///     "auth_type": ledger transaction alias or associated value,
///     "auth_action": type of an action.,
///     "field": transaction field,
///     "old_value": (Optional) old value of a field, which can be changed to a new_value (mandatory for EDIT action),
///     "new_value": (Optional) new value that can be used to fill the field,
/// }
/// requester_info_json: {
///     "role": string - role of a user which can sign transaction.
///     "count": string - count of users.
///     "is_owner": bool - if user is an owner of transaction.
/// }
///
/// # Return
/// "price": u64 - tokens amount required for action performing,
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn indy_get_request_price(command_handle: CommandHandle,
                                     pool_handle: PoolHandle,
                                     wallet_handle: WalletHandle,
                                     submitter_did: *const c_char,
                                     payment_method: *const c_char,
                                     action_json: *const c_char,
                                     requester_info_json: *const c_char,
                                     cb: Option<extern fn(command_handle_: CommandHandle,
                                                          err: ErrorCode,
                                                          price: u64)>) -> ErrorCode {
.......
```

4) Similar to the 3 option but introduces replacement of action json by some aliases like `add_new_trustee`.
The function will fetch the required information from the ledger automatically.

Pros: 
    * API definition is simpler than for 2 option. 
    * User doesn't need to know auth rule parts.  
Cons: 
    * It requires changes in AUTH_RULE related functions to follow the same pattern. 
    * Probably on the both side: sdk and ledger.
    
```
/// Gets request price correspondent to specific auth rule and
/// in case the requester can perform this action.
///
/// # Params
/// command_handle: Command handle to map callback to caller context.
/// pool_handle: pool handle.
/// wallet_handle: wallet handle.
/// submitter_did: (Optional) DID of request sender
/// payment_method: payment method to use
/// action: action alias
/// requester_info_json: {
///     "role": string - role of a user which can sign transaction.
///     "count": string - count of users.
///     "is_owner": bool - if user is an owner of transaction.
/// }
///
/// # Return
/// "price": u64 - tokens amount required for action performing,
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn indy_get_request_price(command_handle: CommandHandle,
                                     pool_handle: PoolHandle,
                                     wallet_handle: WalletHandle,
                                     submitter_did: *const c_char,
                                     payment_method: *const c_char,
                                     action: *const c_char,
                                     requester_info_json: *const c_char,
                                     cb: Option<extern fn(command_handle_: CommandHandle,
                                                          err: ErrorCode,
                                                          price: u64)>) -> ErrorCode {
.......
```