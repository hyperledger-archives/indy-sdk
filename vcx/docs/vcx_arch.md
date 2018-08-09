libVCX Architecture Choices
=====
No persistent storage
-----
libvcx does not have its own storage. While libindy wallet secrets are stored in the libindy wallet, libvxc objects are not stored by libvcx. Instead, each object can be serialized for storage by the user of libvcx.  When the object is needed it can be deserialized.  This allows users of libvcx to manage objects themselves inside long-running applications. libindy has recently introduced a non-secrets API for general storage and libvcx may use this in the future for object storage.

Must initialize library with configuration
-----
Each process that uses libvcx must first initalize the library with "vcx_init(<configuration>)". Every call after will use <configuration>. <configuration> includes things such as agency urls and wallet name. To switch configurations the user must either exit the process and start over or call "vcx_shutdown()".

Thread each API call with callback
-----
Each call to libvcx creates its own thread. Eventually this could be migrated into a thread pool or something similar to libindy.

MsgPack
-----
The current agent to agent communcation protocol uses MsgPack. This will change in the future.

Only compatible with evernym agency
-----
This will change as the agent to agent protocol is defined and implemented by others.

No explicit contract for libindy objects
-----
libindy objects such as credentials, credential_offers, proofs, proof_requests, etc are represented by libvcx as strings and not objects.  In the future this may change if needed and possible.

libindy overlap
-----
There are some convenience functions that overlap with libindy. These are the creation of schemas and credential definition and wallet non-secrets. In most cases the overlap is a simplification of the libindy API.

Thin wrappers
-----
Wrappers are meant to be as thin as possible, in other words, they should have as little logic as possible and should simply wrap the rust code.  They should be idiomatic and make sense to developers familiar with the specific language of the wrapper.

Error codes
-----
This area will need work once it has been migrated to the indy-sdk. There should be a plan for error-code unification.

"Microledger" architecture
-----
DIDs are not stored on the ledger, neither for the initial invitations nor for the pairwise connections. The ledger is used for schemas, credential definitions and proving credentials.

