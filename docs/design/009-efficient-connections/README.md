# Connections optimization design

## Summary

This design proposes enhancements of pool connection logic in libindy for
reducing of pool load and better pool DDoS protection.

## Motivation

For current moment ```indy_pool_open``` endpoint performs CatchUp process and after this
creates zmq sockets for each pool node and keep sockets connected until calling
of ```indy_pool_close``` endpoint.

This behavior causes the most of clients connected for the most of the time. As result
we have obvious problem as each node can open only limited amount of sockets for
the same time. Only first n clients can connect because n+1 connection will just
cause error about limit of opened file descriptors.

Additionally it makes easy to perform DDoS attack by justs calling ```indy_pool_open```
in a cycle.

Note that problem is complex and requires corresponded solution in Node codebase and
pool network infrastructure, but this proposal is focused on libindy (client) side.

## Proposed changes

The main idea of proposal is force libindy to close sockets as soon as possible, but
still provide very limited keep-alive ability to avoid unnecessary CurveCP and TCP
handshakes:

1. Change ```indy_pool_open``` endpoint behavior. Instead keep sockets connected it will only perform CatchUp
   and keep only information about nodes of this pool. Sockets creation will be performed when application
   sends request with some limitations and optimizations (see details below).
1. Persist ```indy_pool_open``` CatchUp results to start from updated pool ledger next time.
1. Introduce internal "pool connection" entity. Each "pool connection" is intended to be used with a specific pool only.
1. "Pool connection" owns sockets and can be used to execute one or multiple requests.
1. After creation "pool connection" becomes "active" for some pre-defined timeout (5 sec) or until some pre-defined amount
   of requests (5) were started through this "pool connection".
1. "Active" means that context can be used to start execution of     new requests to pool.
1. When application tries to send request to pool libindy checks for already exists "pool connection" for target pool in "active" state.
1. If there is no active "pool connection" then libindy creates a new one and uses it for sending request.
1. If there is active "pool connection" then libindy re-uses it for sending request.
1. "pool connection" opens sockets only "by request". if requests execution requires connection to only one node than only one socket will be created.
1. "pool connection" determines node to perform new connection with round robin. If connection is already established than sockets will be re-used.
1. libindy keep opened sockets connected until "pool connection" is active (5s from "pool connection" creation or 5 requests started) or there is request    that waits for response on this socket. As soon as "pool connection" is switched from active state and sockets isn't needed for request it will be
   immediately closed.
1. libindy waits for response on the socket only limited amount of time. There are 2 pre-defined timeouts. First one for getting of "Ack" message
   is short (10s). Second one for getting "Reply" message is significantly longer (100s). If node is health in a 99.9% cases it will
   send "ACK" message in a short period of time (the reason for small timeout).