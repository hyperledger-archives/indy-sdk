## Wallet Query Language

This language will be used to define queries in Non-secrets, Anoncreds search APIs.

```Rust
query = {subquery}
subquery = {subquery, ..., subquery} - WHERE subquery AND ... AND subquery
subquery = $or: [{subquery},..., {subquery}] - WHERE subquery OR ... OR subquery
subquery = $not: {subquery} - Where NOT (subquery)
subquery = "tagName": tagValue - WHERE tagName == tagValue
subquery = "tagName": {$neq: tagValue} - WHERE tagName != tagValue
subquery = "tagName": {$gt: tagValue} - WHERE tagName > tagValue
subquery = "tagName": {$gte: tagValue} - WHERE tagName >= tagValue
subquery = "tagName": {$lt: tagValue} - WHERE tagName < tagValue
subquery = "tagName": {$lte: tagValue} - WHERE tagName <= tagValue
subquery = "tagName": {$like: tagValue} - WHERE tagName LIKE tagValue
subquery = "tagName": {$in: [tagValue, ..., tagValue]} - WHERE tagName IN (tagValue, ..., tagValue)
```

#### Tag types
There are two types of tags:
* Un-encrypted - Tag name starts with "~". That tag will be stored un-encrypted that will allow usage of this tag in complex search queries (comparison, predicates).
* Encrypted - That tag will be stored encrypted. The tag can be searched only for exact matching.

NOTE: Combinators $or, $and, $not can be used with both tag types.