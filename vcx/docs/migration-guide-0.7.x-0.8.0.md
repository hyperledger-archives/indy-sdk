# LibVCX migration guide from 0.7.x to 0.8.0
## A Developer Guide for LibVCX migration

This document is written for developers using LibVCX to provide necessary information and
to simplify their transition to LibVCX 0.8.x from LibVCX 0.7.x.

* [API](#api)
    * [Protocols Compatibility](#protocols-compatibility)

#### Protocols compatibility

* Supported `protocol_version`: `3.0` which actually is an alternative to combination of settings: `protocol_version`: `2.0` and `communication_method`: `aries`.

* Fixed compatibility between proprietary (`protocol_version`: `2.0`/`1.0`) and aries communication protocols (`protocol_version`: `3.0`).

    Added a new enum variant `Pending` for IssuerCredentials/Credentials/Proofs/DisclosedProofs objects.
    Initially create `Pending` objects and convert them to V1/V3 after receiving the connection handle.
    `Pending` objects have `3.0` versions during serialization.
