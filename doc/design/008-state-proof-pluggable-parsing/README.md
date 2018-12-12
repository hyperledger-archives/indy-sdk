## Legend
There are some types of requests to Nodes in the Pool allowing to use StateProof optimization in Client-Node communication.
Instead of sending requests to all nodes in the Pool, a client can send a request to a single Node and expect StateProof signed by BLS multi-signature.

BLS multi-signature (BLS MS) guaranties that there was a consensus of Nodes which signed some State identified by State RootHash.
StateProof (SP) is small amount of data which allows to verify particular values against RootHash.
Combination of BLS MS and SP allows clients to be sure about response of single node is a part of State signed by enough Nodes. 

## Goals
Libindy allows to extend building of supported requests via plugged interface and send them.
It is nice to have a way to support BLS MS and SP verification for these plugged transactions.

Implementation of math for SP verification is a bit complicate for plugin logic.
Therefore libindy should perform all math calculation inside.
A plugin should provide handler to parse custom reply to fixed data structure.

## API
The signature of the handler is described below together with custom `free` call to deallocate result data.

```rust
extern fn CustomTransactionParser(reply_from_node: *const c_char, parsed_sp: *mut *const c_char) -> ErrorCode;
extern fn CustomFree(data: *mut c_char) -> ErrorCode;
``` 

Libindy API will contain call to register handler for specific transaction type:
```rust
extern fn indy_register_transaction_parser_for_sp(command_handle: i32,
                                                  txn_type: *const c_char,
                                                  parser: CustomTransactionParser,
                                                  free: CustomFree,
                                                  cb: extern fn(command_handle_: i32, err: ErrorCode)) -> ErrorCode;
```

### Parsed Data structure

A plugin should parse `reply_from_node` and return back to libindy parsed data as JSON string.
Actually this data is array of entities each of them is describe SP Trie and set of key-value pairs to verify against this trie.
It can be represented as `Vec<ParsedSP>` serialized to JSON.


```rust
/**
 Single item to verification:
 - SP Trie with RootHash
 - BLS MS
 - set of key-value to verify
*/
struct ParsedSP {
    /// encoded SP Trie transferred from Node to Client
    proof_nodes: String,
    /// RootHash of the Trie, start point for verification. Should be same with appropriate filed in BLS MS data
    root_hash: String,
    /// entities to verification against current SP Trie
    kvs_to_verify: KeyValuesInSP,
    /// BLS MS data for verification
    multi_signature: serde_json::Value,
}

/**
 Variants of representation for items to verify against SP Trie
 Right now 2 options are specified:
 - simple array of key-value pair
 - whole subtrie
*/
enum KeyValuesInSP {
    Simple(KeyValueSimpleData),
    SubTrie(KeyValuesSubTrieData),
}

/**
 Simple variant of `KeyValuesInSP`.

 All required data already present in parent SP Trie (built from `proof_nodes`).
 `kvs` can be verified directly in parent trie
*/
struct KeyValueSimpleData {
    kvs: Vec<(String /* b64-encoded key */, Option<String /* val */>)>
}

/**
 Subtrie variant of `KeyValuesInSP`.

 In this case Client (libindy) should construct subtrie and append it into trie based on `proof_nodes`.
 After this preparation each kv pair can be checked.
*/
struct KeyValuesSubTrieData {
    /// base64-encoded common prefix of each pair in `kvs`. Should be used to correct merging initial trie and subtrie
    sub_trie_prefix: Option<String>,
    kvs: Vec<(String /* b64-encoded key_suffix */, Option<String /* val */>)>,
}
```

Expected libindy and plugin workflow is the following:
1. Libindy receive reply from a Node, perform initial processing and pass raw reply to plugin.
1. Plugin parse reply from the Node and specify one or more SP Trie with metadata and items for verification.
1. Each SP Trie described by plugin as `ParsedSP`:
    1. Set of encoded nodes of the SP Trie, received from Node - `proof_nodes`. May be fetched from response "as is".
    1. RootHash of this Trie. May be fetched from the response "as is" also.
    1. BLS MS data. Again may be fetched from the response "as is".
    1. Key-value items to verification. Here plugin should define correct keys (path in the trie) and corresponded values.
1. Plugin return serialized as JSON array of `ParsedSP`
1. For each `ParsedSP` libindy:
    1. build base trie from `proof_nodes`
    1. if items to verify is `SubTrie`, construct this subtrie from (key-suffix, value) pairs and merge it with trie from clause above
    1. iterate other key-value pairs and verify that trie (with signed `root_hash`) contains `value` at specified `key`
    1. verify multi-signature
1. If any verification is failed, libindy ignore particular SP + BLS MS and try to request same data from another node,
or collect consensus of same replies from enough count of Nodes.


Below is JSON structure for `Simple` case.
```json
[
 {
   "proof_nodes": "string with serialized SP tree",
   "root_hash": "string with root hash",
   "kvs_to_verify": {
     "type": "simple",
     "kvs": [["key1", "value1"], ["key2", "value2"]]
   },
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