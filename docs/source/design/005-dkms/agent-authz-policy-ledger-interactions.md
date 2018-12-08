# Agent Authz policy (changes for ledger)
**Objective**: Prove agents are authorized to provide proof of claims and authorize and de-authorize other agents

## Assumptions
1. The ledger maintains a global accumulator that holds commitments sent by the agents.
1. The global accumulator is maintained by the each node so every node knows the accumulator private key
1. Agent auth policy txns are stored at the identity ledger.
1. Each auth policy is uniquely identified by a policy address `I`.
1. One agent can belong to several authz policies, thus several different `I`'s.
1. An agent can have several authorizations. Following are the list of authorizations:
- PROVE
- PROVE\_GRANT
- PROVE\_REVOKE
- ADMIN

## Transactions
### AGENT\_AUTHZ
An authz policy is created/updated by an `AGENT_AUTHZ` transaction. A transaction creating a new authz policy:
```
{
    identifier: <transaction sender's verification key>
    signature: <signature created by the sender's public key>,
    req_id: <a nonce>,
    operation: {
        type: AGENT_AUTHZ,
        address: <policy address, I>,
        verkey: <optional, verification key of the agent>,
        authorization: <optional, a bitset>,
        commitment: <optional>
    }
} 
```
**address**: The policy address, this is a unique identifier of an authz policy. Is a large number (size/range TBD). If the ledger has never seen the provided policy address, it considers the transaction a creation of a new authz policy else it is considered an update of an existing policy identifier by the address.  
**verkey**: An ed25519 verkey of the agent to which the `authorization` corresponds. This is optional when a new policy is being created as `identifier` is sufficient. This verkey should be kept different from any DID verkey to avoid correlation.   
**authorization**: A bitset indicating which authorizations are being given to the agent, it is ignored when creating a new policy (the ledger does not know `I`). The various bits indicate different authorizations:

```
0 None (revoked)
1 ADMIN (all)
2 PROVE
3 PROVE_GRANT
4 PROVE_REVOKE
5 "Reserved for future"
6 "Reserved for future"
7  ... 
   ... 
```

While creating a new policy, this field's value is ignored and the creator agent has all authorizations. For any subsequent policy transactions, the ledger checks if the sender (author to be precise, since anyone can send a transaction once a signature has been done) of transaction has the authorization to make the transaction, eg. The author of txn has `PROVE_GRANT` if it is giving a `PROVE` authorization to another agent.  
**Future Work**: When we support `m-of-n` authorization, `verkey` would be a map stating the policy and the verkeys

**commitment**: This is a number (size/range TBD) given by the agent when it is being given a ``PROVE`` authorization. Thus this field is only needed when a policy is being created or an agent is being given the `PROVE` authorization. The ledger upon receiving this commitment checks if the commitment is prime and if it is then it updates the global accumulator with this commitment. Efficient primality testing algorithms like BPSW or ECPP can be used but the exact algorithm is yet to be decided.  If the commitment is not prime (in case of creation or update of policy address) then the transaction is rejected. The ledger rejects the transaction if it has already seen the commitment as part of another transaction. 
In case of creation of new policy or an agent being given `PROVE` authorization, the ledger responds with the accumulator value after the update with this commitment.


### GET\_AGENT\_AUTHZ
This query is sent by any client to check what the authz policy of any address `I` is
```
{
	...,
	operation: {
		type: GET_AGENT_AUTHZ,
		address: <policy address, I>,
	}
} 
```

The ledger replies with all the agents, their associated authorizations and the commitments of the address `I`.

### GET\_AGENT\_AUTHZ\_ACCUM
This query is sent by anyone to get the value of the accumulator. 
```
{
	...,
	operation: {
		type: GET_AGENT_AUTHZ_ACCUM,
    accum_id: <id of either the provisioned agents accumulator or the revoked agent accumulator>
	}
} 
```
The ledger returns the global accumulator with the id. Both accumulators are add only; the client checks that commitment is present in one accumulator AND not present in other accumulator.


## Data structures
### Ledger
Each authz transaction goes in the identity ledger.

### State trie.
The state stores:
1. Accumulator: The accumulator is stored in the trie at name `<special byte denoting an authz prove accumulator>` with value as the value of accumulator. 
2. Policies: The state stores one name for each policy, the name is `<special byte denoting an authz policy>:<policy address>`, the value at this name is a hash. The hash is determined deterministically serializing (RLP encoding from ethereum, we already use this) this data structure: 

```
[
  [<agent verkey1>, <authorization bitset>, [commitment>]],
  [<agent verkey2>, <authorization bitset>, [commitment>]],
  [<agent verkey3>, <authorization bitset>, [commitment>]],
]
```

The hash of above can then be used to lookup (it is not, more on this later) the exact authorization policy in a separate name-value store. This is done to keep the database backing the state (trie) smaller.

### Caches
There is an agent\_authz cache used for optimisations are:
The cache is a name-value store (leveldb) and offers a constant lookup time for lookup by name.
1. Policy values: The authorization of each agent per policy. The values for the keys are rlp encoding of the list of at most 2 items, `authorization bitset` with each bit respresenting a different auth, `commitment` is optional and relevant only when agent has the `PROVE` authorization.

```
{
  <policy address 1><delimiter><agent verkey 1>: <authorization bitset>:<commitment>,
  <policy address 1><delimiter><agent verkey 2>: <authorization bitset>:<commitment>,
  <policy address 1><delimiter><agent verkey 3>: <authorization bitset>:<commitment>,
  <policy address 2><delimiter><agent verkey 1>: <authorization bitset>:<commitment>,
  <policy address 2><delimiter><agent verkey 2>: <authorization bitset>:<commitment>,
  ....
}
```
These names are used by the nodes during processing any transaction.

2. Accumulator value: Value of each accumulator is stored corresponding to the special byte indicating the global accumulator.
```
{
  <special_byte>: <accumulator value>,
}
```

During processing of any write transaction, the node updates the ledger, state and caches after the txn is successful but for querying (client as well as its own like validation, etc) it only uses caches since caches are more efficient than state trie. The state trie is only used for state proofs.  

3. TODO: Maintaining a set of commitments: Each node maintains a set of see commitments and does not allow duplicate commitments. Its kept in key value store with constant lookup time for commitment lookup.


## Code organisation.
These changes would be implemented as a separate plugin. The plugin will not introduce new ledger or state but will introduce the cache described above. The plugin will introduce a new request handler which will subclass the `DomainRequestHandler`. The plugin's new request handler will introduce 1 `write_type` and 2 `query_types` and methods to handle those.
