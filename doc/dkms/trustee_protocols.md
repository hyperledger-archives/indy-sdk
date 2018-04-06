# Trustee Setup Protocol
**Objective**: Provide the messages and data formats so an identity owner can choose, update, remove trustees and their delegated capabilities.

## Assumptions

1. An identity owner selects a connection to become a trustee
1. Trustees can be granted various capabilities by identity owners
    1. Safeguarding a recovery share. This will be the most common
    1. Revoke an authorized agent on behalf of an identity owner
    1. Provision a new agent on behalf of an identity owner
    1. Be an administrator for managing identity owner agents
1. Trustees agree to any new specified capabilities before any action is taken
1. Trustees will safeguard recovery shares. Their app will encrypt the share and not expose it to anyone else
1. Trustees authenticate out-of-band an identity owner when a recovery event occurs
1. The Trustees' app should only send a recovery share to an identity owner after they have been authenticated
1. All messages will use [Message Packaging](https://github.com/evernym/protocol/blob/master/message-packaging.md).

## Messages and Structures
Messages are formatted as JSON. All binary encodings use base64url.
All messages include the following fields:

1. *version* \<string\>: The semantic version of the message data format.
1. *type* \<string\>: The message type.

### CAPABILTY\_OFFER
Informs a connection that the identity owner wishes to make them a trustee. The message includes information about what
capabilities the identity owner has chosen to grant a trustee and how long the offer is valid.
This message adds the following fields

*expires* \<string\>: 64-bit unsigned big-endian integer. The number of seconds elapsed between January 1, 1970 UTC and the time the offer will expire if no request message is received. This value is purely informative.\
*capabilities* \<list\[string\]\>: A list of capabilities that the trustee will be granted. They can include

1. RECOVERY\_SHARE: The trustee will be given a recovery share
1. REVOKE\_AUTHZ: The trustee can revoke agents
1. PROVISION\_AUTHZ: The trustee can provision new agents
1. ADMIN\_AUTHZ: The trustee is an administrator of agents

```
{
  "version": "0.1",
  "type": "CAPABILITY_OFFER",
  "capabilities": ["RECOVERY_SHARE", "REVOKE_AUTHZ", "PROVISION_AUTHZ"]
  "expires": 1517428815
}
```

### CAPABILTY\_REQUEST
Sent to an identity owner in response to a TRUSTEE\_OFFER message. The message includes includes information
for which capabilities the trustee has agreed. This message adds the following fields

*for\_id* \<string\>: The nonce sent in the TRUSTEE\_OFFER message.\
*capabilities* \<object\[string,string\]\>: A name value object that contains the trustee's response for each privilege.\
*authorizationKeys* \<list\[string\]\>: The public keys that the trustee will use to verify her actions with the authz policy registry on behalf of the identity owner.

```
{
  "version": "0.1",
  "type": "CAPABILITY_REQUEST",
  "authorizationKeys": ["Rtna123KPuQWEcxzbNMjkb"]
  "capabilities": ["RECOVERY_SHARE", "REVOKE_AUTHZ"]
}
```

### CAPABILITY\_RESPONSE
Sends the identity owner policy address and/or recovery data and metadata to a recovery trustee. A trustee should send a confirmation message that this message was received.

*address* \<string\>: The identity owner's policy address. Only required if the trustee has a key in the authz policy registry.\
*share* \<object\>: The actual recovery share data in the format given in the next section. Only required if the trustee has the RECOVERY\_SHARE privilege.

```
{
  "version": "0.1",
  "type": "CAPABILITY_RESPONSE",
  "address": "b3AFkei98bf3R2s"
  "share": {
    ...
  }
}
```

### TRUST\_PING
Authenticates a party to the identity owner for out of band communication.

*challenge* \<object\>: A message that a party should respond to so the identity owner can be authenticated. Contains a *question* field for the other party to answer
and a list of *valid\_responses*.

```
{
  "version": "0.1",
  "type": "TRUST_PING",
  "challenge": {
    ...
  }
}
```

*challenge* will look like the example below but allows for future changes as needed.\
*question* \<string\>: The question for the other party to answer.\
*valid\_responses* \<list\[string\]\>: A list of valid responses that the party can give in return.

```
{
    "question": "Are you on a call with CULedger?",
    "valid_responses": ["Yes", "No"]
}
```

### TRUST\_PONG
The response message for the TRUST\_PING message.

```
  "version": "0.1",
  "type": "TRUST_PONG",
  "answer": {
    "answerValue": "Yes"
  }
```

### KEY\_HEARTBEAT\_REQUEST
Future\_Work: Verifies a trustee/agent has and is using the public keys that were given to the identity owner. These keys

*authorizationKeys* \<list\[string\]\>: Public keys the identity owner knows that belong to the trustee/agent.

```
{
  "version": "0.1",
  "type": "KEY_HEARTBEAT_REQUEST",
  "authorizationKeys": ["Rtna123KPuQWEcxzbNMjkb"]
}
```

### KEY\_HEARTBEAT\_RESPONSE
Future\_Work: The updated keys sent back from the trustee/agent 


### RECOVERY\_SHARE\_RESPONSE
Future\_Work: After an identity owner receives a challenge from a trustee, an application prompts her to complete the challenge. This message contains her response.

*for\_id* \<string\>: The nonce sent in the RECOVERY\_SHARE\_CHALLENGE message.\
*response* \<object\>: The response from the identity owner.

```
{
  "version": "0.1",
  "type": "RECOVERY_SHARE_RESPONSE",
  "response": {
    ...
  }
}
```

*response* will look like the example below but allows for future changes as needed.

```
{
  "pin": "3qA5h7"
}
```

## Recovery Share Data Structure
Recovery shares are formated in JSON with the following fields:

1. *version* \<string\>: The semantic version of the recovery share data format.
1. *source\_did*: The identity owner DID that sent this share to the trustee
1. *tag* \<string\>: A value used to verify that all the shares are for the same secret. The identity owner compares this to every share to make sure they are the same.
1. *shareValue* \<string\>: The share binary value.
1. *hint* \<object\>: Hint data that contains the following fields:
    1. *trustees* \<list\[string\]\>: A list of all the recovery trustee names associated with this share. These names are only significant to the identity owner. Helps to aid in recovery by providing some metadata for the identity owner and the application.
    1. *threshold* \<integer\>: The minimum number of shares needed to recover the key. Helps to aid in recovery by providing some metadata for the identity owner and the application.

```
{
  "version": "0.1",
  "source_did": "did:sov:asbdfa32135"
  "tag": "ze4152Bsxo90",
  "shareValue": "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ123456789"
  "hint": {
    "theshold": 3,
    "trustees": ["Mike L", "Lovesh", "Corin", "Devin", "Drummond"]
  }
}
```
