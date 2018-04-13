# Guidelines for Libindy Wrappers
>These are more like constructive suggestions and not strict guidelines, if a contributor/community have strong reasons to not follow these instructions, they can choose to implement wrappers as they seem fit.

These are the guidelines generally followed for developing the wrappers for LibIndy.

## 1. The wrapper should be idiomatic for the language in question.

For example, in object-oriented languages, 
- It might be expected all functions that take a handle to a wallet to turn into methods of a wallet class. 
- It would be expected to have the "indy_" prefix on C functions to turn into an "indy" namespace. 
- Function names and parameters in the wrapper to use javaCamelCase in java, and class names to use TitleCase in java. 
- Languages that support exceptions may (or may not) want to throw exceptions instead of raising errors. 
- Languages that support closures and futures might want to use those instead of callbacks like the C-callable API. And so forth. The overall effect for using the wrapper in a given language should be that it feels like a wonderful API, customized for the language in question.

## 2. The wrapper should keep certain things invariant.

The reason to keep things invariant is to make wrappers similar to one another, and to avoid unnecessary documentation burden or mental model translation between wrappers (which may have light doc) and libindy (where the investment in doc will be substantial).

If someone learns how to use a wrapper of the API in language 1, and then go look at the wrapper of the API in language 2, one should see the same basic concepts, and be able to predict the function names one should call and the sequence of calls one should use.

Each wrapper should preserve all the terminology of the main interface; do not decide in a wrapper that you didn't like the original API's term "wallet" and change it to "keychain" in a wrapper. All function names and parameters should be the same (except transformed to an idiomatic convention for the language, so create_and_store_proof() might become CreateAndStoreProof()). All error codes should be identical. (If you throw exceptions instead of raising errors, the original error numbers should be preserved.) The original API is threadsafe; the wrappers should be, too.

All preconditions on parameters, all data formats (e.g., json, config files, etc), all constraints on input should be identical in all wrappers and in the base API.

The "level" of the wrapper API can be generally "higher" than the C-callable functions, but it should expose the same amount of functionality and flexibility as the C-callable functions, so the higher level functions, if they exist, should be an addition to lower-level ones, not substitutes for them. In other words, helpers and convenience methods are great, but don’t allow them to entirely hide the core functions which provide granular control.

Important architectural choices should be preserved. For example, the original C-callable API separates preparation of a transaction from transmission of a transaction, to support the use case where transmission of a txn and preparation of it need to take place on different machines. This should still be possible in a wrapper. Likewise, the design choice that private keys remain inside wallets instead of passing into user code where they can be mishandled should be preserved. Likewise, the design choice that functions call a wallet automatically instead of expecting the user to know when to call the wallet. Likewise, choices about how state is managed and where it is necessary. Likewise, wrappers should not attempt to change timeout behaviors built into libindy.

## 3. Things that the wrapper adds should be identified as separate from what the core API provides.

It's fine for a wrapper to add convenience methods and extra ideas. However, these should be identified as extensions, separate from the core wrapper. For example, they could be put in a different (sub)namespace, or in different classes, or in functions that have an "_ex" suffix or an "Easy" prefix, etc. This will help people understand what is core, so the consumer of a wrapper knows which functions are likely to be identical in semantics to what libindy documents.

## 4. Versioning and applicability

The wrapper should document the earliest and latest version of libindy that it knows to be compatible. Likewise, it should document what platforms it targets, what use cases it's built for, etc. A wrapper should be able to find libindy using default OS methods (e.g., in the system PATH), but should also provide a way for a specific path to libindy to be specified, such that the wrapper can work either from an OS-wide install of libindy or a version in a particular directory.

