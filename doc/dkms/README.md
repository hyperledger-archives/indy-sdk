# Decentralized Key Management

### Introduction
A decentralized key management system (DKMS) is an approach to cryptographic key
management where there is no central authority. DKMS leverages the security,
immutability, availability, and resiliency properties of distributed ledgers
to provide highly scalable key distribution, verification, and recovery.

### Key Types
DKMS uses the following key types:
1. **Master keys**: Keys that are not cryptographically protected. They are distributed manually or
initially installed and protected by procedural controls and physical or electronic isolation.
2. **Key encrypting keys**: Symmetric or public keys used for key transport or storage of other keys.
3. **Data keys**: Used to provide cryptographic operations on user data (e.g., encryption, authentication).

The keys at one level are used to protect items at a lower level. Consequently, special measures
are used to protect master keys, including severely limiting access and use, hardware protection,
and providing access to the key only under shared control.

### Key Loss
Key loss means the owner no longer controls the key and it can assume there is no further risk of compromise. For example devices unable to function due to water, electricity, breaking, fire, hardware failure, acts of God, etc.
### Compromise
Key compromise means that private keys and/or master keys have become or can become known either passively or actively.

### Recovery
In decentralized identity management, recovery is important since identity owners have no “higher authority”
to turn to for recovery.
1. Offline recovery uses physical media or removable digital media to store recovery keys.
2. Social recovery employs entities trusted by the identity owner called "trustees" who store recovery data on an identity owners behalf—typically
in the trustees own agent(s).

These methods are not exclusive and should be combined with key rotation and revocation for proper security.

1. [Design and architecture](DKMS%20Design%20and%20Architecture%20V3.md)
2. **Public Registry for Agent Authorization Policy**. An identity owner create a policy on the ledger that defines its agents and their authorizations. 
   Agents while acting on the behalf of the identity owner need to prove that they are authorised. [More details](https://github.com/hyperledger/indy-sdk/blob/master/doc/dkms/Agent Authorization Policy.pdf)  
   