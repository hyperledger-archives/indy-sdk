## Legend
There are some types of requests to Nodes in the Pool which allow the use of StateProof (SP) optimization in
Client-Node communication. Instead of sending requests to all nodes in the Pool, a client can send a request
to a single Node and expect a StateProof signed by a Boneh–Lynn–Shacham (BLS) multi-signature.

BLS multi-signature (BLS MS) guaranties that there was a consensus of Nodes which signed some State
identified by the State RootHash. StateProof (SP) is small amount of data which allows the verification of
particular values against the RootHash. The combination of BLS MS and SP allows clients to be sure that
the response of single node is a part of the State signed by a sufficient number of Nodes. 

## Goals
Libindy also allows the building and sending of supported requests via a plugable interface.
It is nice to have a way to support BLS MS and SP verification for these plugged transactions.

The implementation of math for SP verification is a bit complicated to include in plugin logic.
Therefore, libindy should perform all of the math calculations inside the SDK.
A plugin should provide a handler to parse the custom reply to a fixed data structure.

## API
The signature of the handler is described below together with the custom `free` call to deallocate result data.

```rust
extern fn CustomTransactionParser(reply_from_node: *const c_char, parsed_sp: *mut *const c_char) -> ErrorCode;
extern fn CustomFree(data: *mut c_char) -> ErrorCode;
``` 

The libindy API will contain a call to register the handler for a specific transaction type:
```rust
extern fn indy_register_transaction_parser_for_sp(command_handle: i32,
                                                  txn_type: *const c_char,
                                                  parser: CustomTransactionParser,
                                                  free: CustomFree,
                                                  cb: extern fn(command_handle_: i32, err: ErrorCode)) -> ErrorCode;
```

### Parsed Data structure

A plugin should parse `reply_from_node` and return back to libindy the parsed data as JSON string.
Actually this data is an array of entities, each of them is described as a SP Trie and a set of key-value pairs to
verify against this trie.
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
1. Libindy receives a reply from a Node, performs initial processing and passes raw reply to plugin.
1. Plugin parses reply from the Node and specifies one (or more) SP Trie with metadata and items for verification.
1. Each SP Trie is described by the plugin as `ParsedSP`:
    1. Set of encoded nodes of the SP Trie, received from Node - `proof_nodes`. May be fetched from response "as is".
    1. RootHash of this Trie. May be fetched from the response "as is" also.
    1. BLS MS data. Again may be fetched from the response "as is".
    1. Key-value items for verification. Here the plugin should define the correct keys (path in the trie) and their corresponding values.
1. Plugin returns serialized as JSON array of `ParsedSP`
1. For each `ParsedSP` libindy:
    1. build base trie from `proof_nodes`
    1. if item to verify is `SubTrie`, construct this subtrie from (key-suffix, value) pairs and merge it with trie from clause above
    1. iterate other key-value pairs and verify that trie (with signed `root_hash`) contains `value` at specified `key`
    1. verify multi-signature
1. If any verification fails, libindy will ignore that particular SP + BLS MS and try to request the same data from another node,
or collect a consensus of the same replies from a sufficient number of Nodes.


Below is the JSON structure for `Simple` case.
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

### Simple and SubTrie Verification

Some use cases require verification of multiple of key-value pairs in one Trie.
Moreover, there is possible situation when a client would like to verify the whole subtrie.
In this case, the amount of data transferred from Node to Client can be significantly reduced.
Instead of including all nodes for SP verification to `proof_nodes`, a Node can include only a prefix path down to
a subtrie.
The entire subtrie to be verified can be restored on the Client side from key-value pairs and combined with the
prefix part.