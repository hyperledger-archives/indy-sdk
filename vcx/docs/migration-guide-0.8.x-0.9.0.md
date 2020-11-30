# LibVCX migration guide from 0.8.x to 0.9.0
## A Developer Guide for LibVCX migration

This document is written for developers using LibVCX to provide necessary information and
to simplify their transition to LibVCX 0.9.x from LibVCX 0.8.x.

* [API](#api)
    * [Protocols Compatibility](#protocols-compatibility)

#### Protocols compatibility

* Supported `protocol_version`: `4.0`.
 When VCX is set to use  `protocol_version`: `4.0` for all VCX API functions the following met:
    * all inputs are expected to be in the Aries message format.
    * all outputs will be in the Aries message format.

* Added a new function `vcx_delete_credential` to delete credential from the wallet.


* Changed behavior of `vcx_*_update_state_with_message` functions for Aries protocols handling.

In previous Libvcx versions was possible situation when 
1. VCX processed some message 
2. VCX automatically update message status on the Agency 
3. VCX return result to the app 
4. An application failed before storing new VCX object state. 
5. That makes the message unavailable anymore.

The behavior of `vcx_*_update_state_with_message` functions was changed to not update the status of messages on the Agency internally.
Instead, the application using VCX should care about changing of message status using `vcx_messages_update_status` function.

* Bugfixes

