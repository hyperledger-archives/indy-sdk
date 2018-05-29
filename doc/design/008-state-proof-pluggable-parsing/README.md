## Legend
There are some types of requests to Nodes in Pool which allows to use StateProof optimization in Client-Node communication.
Instead of sending request to all nodes in the Pool, client send request to single Node and expect StateProof signed by BLS multi-signature.

BLS multi-signature (BLS MS) guaranties what consensus of Nodes signed some State which can be identified by State RootHash.
StateProof (SP) is small amount of data which allows to verify particular values against RootHash.
Combination of BLS MS and SP allows clients to be sure about response of single node is a part of State signed by enough Nodes. 

## Goals
Libindy allows to create and send custom transaction via plugged interface.
It is nice to have a way to support BLS MS and SP verification for these plugged transactions.

Implementation of math for SP verification is a bit complicate for plugin logic.
Therefore libindy should perform all math calculation inside.
A plugin should provide handler to parse custom reply to fixed data structure.

## API
The signature of the handler is described below together with custom `free` call to deallocate result data.

```rust
extern fn custom_transaction_parser(reply_from_node: *const c_char, parsed_sp: *mut *const c_char) -> ErrCode;
extern fn custom_free(data: *mut c_char) -> ErrCode;
``` 

Libindy API will contain call to register handler for specific transaction type:
```rust
extern fn indy_register_transaction_parser_for_sp(command_handle: i32,
                                                  txn_type: *const c_char,
                                                  parser: custom_transaction_parser,
                                                  free: custom_free,
                                                  cb: extern fn(command_handle_: i32, err: ErrCode)) -> ErrCode;
```

### Parsed Data structure

A plugin should parse `reply_from_node` and return back to libindy parsed data as JSON string.
Actually this data is `Vec<ParsedSP>` serialized to JSON.


```rust 
struct ParsedSP {
    proof_nodes: String,
    root_hash: String,
    kvs_to_verify: KeyValuesInSP,
    multi_signature: JsonObject,
}

enum KeyValuesInSP {
    Simple(Vec<(String /* key */, String /* val */)>),
    SubTrie(KeyValuesSubTrieData),
}

struct KeyValuesSubTrieData {
    sub_trie_prefix: Option<String>,
    kv: Vec<(String /* key_suffix */, String /* val */)>,
}
```

Below is JSON structure for `Simple` case.
```json
[
 {
   "proof_nodes": "string with serialized SP tree",
   "root_hash": "string with root hash",
   "kvs_to_verify": [["key1", "value1"], ["key2", "val2"]],
   "multi_signature": "JSON object from Node`s reply as is"
 }
]
```

### Simple and SubTrie verification

Some use cases require verification multiply pairs of key-value in one Trie.
Moreover there is possible situation when client would like to verify whole subtrie.
In this case, the amount of data transferred from Node to Client can be significantly reduced.
Instead of including all nodes for SP verification to `proof_nodes`, Node can include only prefix path down to subtrie.
The entire subtrie to verification can be restored on Client side from key-value pairs and combined with prefix part.