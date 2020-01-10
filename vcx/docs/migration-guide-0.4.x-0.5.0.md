# LibVCX migration guide from 0.4.x to 0.5.0

## A Developer Guide for LibVCX migration

This document is written for developers using LibVCX to provide necessary information and
to simplify their transition to LibVCX 0.5 from LibVCX 0.4.x.

* [API]()
    * [Vcx API](#vcx-api)

#### Vcx API

We introduced support for Aries protocols:
 * Connection (https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol)
 * Credential Issuance (https://github.com/hyperledger/aries-rfcs/tree/master/features/0036-issue-credential)
 * Credential Presentation (https://github.com/hyperledger/aries-rfcs/tree/master/features/0037-present-proof)

In general, if you were using old versions protocols in LibVCX and want to continue using them you don't need to do anything -- you can just take the new version and use it as is -- no formats or workflows were changed.

If you need to use newer versions of protocols and you are not getting any information from messages -- you need to just set "communication_protocol" value to "aries" and create new connection with agent who supports Aries. You future interaction will be defined by Aries protocols.

If you need to parse some information from messages, you need to update parsers for the new formats of messages. You can find updated message formats in Aries protocol descriptions.