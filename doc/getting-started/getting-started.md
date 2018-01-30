# Getting Started with Indy

## A Developer Guide for an Implementation of the Indy Code Base

![logo](https://raw.githubusercontent.com/hyperledger/indy-node/master/collateral/logos/indy-logo.png)

* [Getting Started with Indy](#getting-started-with-indy)
  * [What Indy Is, and Why it Matters](#what-indy-is-and-why-it-matters)
  * [What We'll Cover](#what-well-cover)
  * [Involving of Alice](#alice-gets-a-transcript)
  * [Infrastructure preparation](#alice-gets-a-transcript)
      * [Getting Trust Anchor credentials for Faber, Acme, Thrift and Government](#alice-gets-a-transcript)
      * [Connecting to Indy nodes pool](#alice-gets-a-transcript)
      * [Getting the ownership for Stewards's Verinym](#alice-gets-a-transcript)
      * [Onboarding Faber, Acme, Thrift and Government by Steward](#alice-gets-a-transcript)
        * [Connection establishment](#alice-gets-a-transcript)
        * [Getting self-sovereign identity](#alice-gets-a-transcript)
      * [Claim Schemas Setup](#alice-gets-a-transcript)
      * [Claim Definition Setup](#alice-gets-a-transcript)
  * [Alice Gets a Transcript Claim](#accept-a-connection-request)
  * [Apply for a Job](#apply-for-a-job)
  * [Alice Gets a Job-Certificate Claim](#accept-a-connection-request)
  * [Apply for a Loan](#apply-for-a-loan)
  * [Explore the Code](#explore-the-code)

## What Indy is, and Why it Matters

The Indy code base (Indy) is a software ecosystem for private, secure, and powerful identity. Once it is implemented, it puts people — not the organizations that traditionally centralize identity — in charge of decisions about their own privacy and disclosure. This enables all kinds of rich innovation: connection contracts, revocation, novel payment workflows, asset and document management features, creative forms of escrow, curated reputation, integrations with other cool technologies, and so on.

Indy uses open-source, distributed ledger technology. These ledgers are a form of database that is provided cooperatively by a pool of participants, instead of by a giant database with a central admin. Data lives redundantly in many places, and it accrues in transactions orchestrated by many machines. Strong, industry-standard cryptography protects it. Best practices in key management and cybersecurity pervade its design. The result is a reliable, public source of truth under no single entity’s control, robust to system failure, resilient to hacking, and highly immune to subversion by hostile entities.

If the cryptography and blockchain details feel mysterious, fear not: this guide will help introduce you to key concepts within Indy. You’re starting in the right place.

## What We’ll Cover

Our goal is to introduce you to many of the concepts of Indy, and give you some idea of what happens behind the scenes to make it all work.

We are going to frame the exploration with a story. Alice, a graduate of the fictional Faber College, wants to apply for a job at the fictional company Acme Corp. As soon as she has the job, she wants to apply for a loan in Thrift Bank so she can buy a car. She would like to use her college transcript as proof of her education on the job application; once hired, Alice would like to use the fact of employment as evidence of her creditworthiness for the loan.

The sorts of identity and trust interactions required to pull this off are messy in the world today; they are slow, they violate privacy, and they are susceptible to fraud. We’ll show you how Indy is a quantum leap forward.

Ready?

## Involving of Alice

As a graduate of Faber College, Alice receives an alumni newsletter where she learns that her alma mater is offering digital transcripts. She logs in to the college alumni website and requests her transcript by clicking **Get Transcript**.  (Other ways to initiate this request might include scanning a QR code, downloading a transcript package from a published URL, etc.)

Alice doesn’t realize it yet, but in order to use this digital transcript she will need a new type of identity -- not the traditional identity that Faber College has built for her in its on-campus database, but a new and portable one that belongs to her, independent of all past and future relationships, that nobody can revoke or co-opt or correlate without her permission. This is a **_self-sovereign identity_** and it is the core feature of the ledger.

In normal contexts, managing a self-sovereign identity will require a tool such as a desktop or mobile application. It might be a standalone app, or it might leverage a third party service provider that the ledger calls an **agency**. For example, leaders in this technology such as the Sovrin Foundation and companies like Evernym, publish reference versions of such tools. Faber College will have studied these requirements and will recommend a **_Indy app_** to Alice if she doesn’t already have one; this app will install as part of the workflow from the **Get Transcript** button.

When Alice clicks **Get Transcript**, she will download a file that holds an Indy **connection request**. This connection request file, having a .indy extension and associated with her Indy app, will allow her to establish a secure channel of communication with another party in the ledger ecosystem -- Faber College.

So when Alice clicks **Get Transcript**, she will normally end up installing an app (if needed), launching it, and then being asked by the app whether she wants to accept a request to connect with Faber.

For this guide, however, we’ll be using a **Indy SDK API** instead of an app, so we can see what happens behind the scenes. We will pretend to be a particularly curious and technically adventurous Alice…

## Infrastructure preparation

### Getting Trust Anchor credentials for Faber, Acme, Thrift and Government

Faber College and another actors have done some prep steps to offer this service to Alice. To understand these steps let's start with some definitions.

The ledger is intended to store **Identity Records** that describe a **Ledger Entity**. Identity Records are public data and may include Public Keys, Service Endpoints, Claim Schemas, Claim Definitions. Every **Identity Record** is associated with exactly one **DID** (Decentralized Identifier) that is globally unique and resolvable (via a ledger) without requiring any centralized resolution authority. To maintain privacy each **Identity Owner** can own multiple DIDs.

In this tutorial we will use 2 type of DIDs. The first one is **Verinym**. **Verinym** is associated with the **Legal Identity** of the **Identity Owner**. For example, all parties should be able to verify that some DID is used by Government to publish schemas for some document type. The second type is **Pseudonym** - a **Blinded Identifier** used to maintain privacy in the context on an ongoing digital relationship (**Connection**). if Pseudonym is used to maintain only one digital relationship we will call it Pairwise-Unique Identifier. We will use Pairwise-Unique Identifiers to maintain secure connections between actors in this tutorial.

Creation of DID known to the ledger is **Identity Record** itself (NYM transaction). NYM transaction can be used for creation of new DIDs that ledger known, setting and rotation of verification key, setting and changing of roles. The most important fields of this transaction are dest (target DID), role (role of a user NYM record being created for) and verkey (target verification key). See [Requests](https://github.com/hyperledger/indy-node/blob/master/docs/requests.md) to get more information about supported ledger transactions.

Published with DID verification key allows to verify that someone owns this DID as he is only one who known corresponded sign key and any DID-related operation require signing with this key.

Our ledger is public permissioned and anyone who wants to publish DIDs need to get the role of **Trust Anchor** on the ledger. A **Trust Anchor** is a person or organization that the ledger already knows about, that is able to help bootstrap others. (It is *not* the same as what cybersecurity experts call a "trusted third party"; think of it more like a facilitator). See [Roles](https://docs.google.com/spreadsheets/d/1TWXF7NtBjSOaUIBeIH77SyZnawfo91cJ_ns4TR-wsq4/edit#gid=0) to get more information about roles.

On the first step Faber College, Acme Corp and Thrift Bank need to get the role of **Trust Anchor** on the ledger as they will need to create Verinyms and Pairwise-Unique Identifiers to provide the service to Alice. Becoming a **Trust Anchor** requires contacting a person or organization who already has **Trust Anchor** role on the ledger. In our empty test ledger we have only NYMs with **Steward** role, but all **Stewards** are automatically **Trust Anchors**.

#### Connecting to Indy nodes pool

We are ready to start writing the code that will cover full Alice's use case. Important note is that for demo purposes it will be single test that will contain the code intended to be executed on different agents. We will always point what Agent is intended to execute each code part. Also we will use different wallets to store DID and keys of different Agents. Let's start.

First code block will contain the code of **Steward's** agent. To write and read ledger's transactions we need to make connection to Indy nodes pool. Libindy allows to work with different pools like Sovrin pool or local pool we started by ourself as part of this tutorial. To work with different pools libindy has concept of **Pool Configuration**. The list of nodes in the pool is also stored in the ledger as NODE transactions. Libindy allows to restore actual list of NODE transactions by few known transactions that we call genesis transactions. Each **Pool Configuration** are defined as pair of pool configuration name and pool configuration json. The most important field in pool configuration json is path to file with the list if genesis transactions. ``pool.create_pool_ledger_config`` call allows to create named pool configuration. After pool configuration is created we can connect to the nodes pool that this configuration describes by calling ``pool.open_pool_ledger``. This call returns pool handle that can be used to reference this opened connection in future libindy calls.

```python
  " Steward Agent
  pool_name = 'pool1'
  pool_genesis_txn_path = get_pool_genesis_txn_path(pool_name)
  pool_config = json.dumps({"genesis_txn": str(pool_genesis_txn_path)})
  await pool.create_pool_ledger_config(pool_name, pool_config)
  pool_handle = await pool.open_pool_ledger(pool_name, None)
```

#### Getting the ownership for Stewards's Verinym

On the next step **Steward's** agent should get the ownership for DID that has corresponded NYM transaction with **Steward** role on the ledger. The test ledger we use was pre-configured to store some known **Steward** NYMs. Also we know **seed** values for random number generator that were used to generate keys for this NYMs. These **seed** values allow us to restore sign keys for these DIDs on **Steward's** agent side and as result get DID ownership.

Libindy has conception of the **Wallet**. Wallet is secure storage for crypto materials like DIDs, keys and etc... To store **Steward's** DID and corresponded signkey agent should create named wallet first by calling ``pool.create_wallet``. After this wallet can be opened by calling ``pool.open_wallet``. This call returns wallet handle that can be used to reference this opened wallet in future libindy calls.

After wallet is opened we can create DID record in this wallet by calling ``did.create_and_store_my_did`` that returns generated DID and verkey part of generated key. Signkey part for this DID will be stored in the wallet too, but it is impossible to read it directly.

```python
  " Steward Agent
  steward_wallet_name = 'sovrin_steward_wallet'
  await wallet.create_wallet(pool_name, steward_wallet_name, None, None, None)
  steward_wallet = await wallet.open_wallet(steward_wallet_name, None, None)

  steward_did_info = {'seed': '000000000000000000000000Steward1'}
  (steward_did, steward_key) = await did.create_and_store_my_did(steward_wallet, json.dumps(steward_did_info))
```

Please note that we provided only information about seed to ``did.create_and_store_my_did``, but no any information about Steward's DID. The reason is that by default DID's are generated as fist 16 bytes of verkey. For such DID's for operations that require both DID and verkey we can use verkey in abbreviated form. In this form verkey starts with a tilde '~' followed by 22 or 23 characters. The tilde indicates that the DID itself represents the first 16 bytes of the verkey and the string following the tilde represents the second 16 bytes of the verkey, both using base58Check encoding.

#### Onboarding Faber, Acme, Thrift and Government by Steward

On the next step Faber, Acme, Thrift and Government should establish **Connection** with Steward. 
Each connection is actually a pair of Pairwise-Unique Identifiers (DIDs). 
The one DID is owned by one connection party and second by another.
Both parties are know both DIDs and understand what connection this pair describes.
The relationship between them is not shareable with others; it is unique to those two parties in that each pairwise relationship uses different DIDs. 
We call process of establish connection **Onboarding**. 
In this tutorial we will describe the simple version of onboarding process. 
In our case one party will be always Trust Real enterprise scenarios can use more complex version. 

##### Connection establishment
Let's look the process of connection establishment between Steward and Faber.

1. **Faber** and **Sovrin Steward** contact in a some way to initiate onboarding process. It can be filling the form on web site or phone call.
1. **Sovrin Steward** creates DID record that he will use for this secure interactions with **Faber** 
1. **Sovrin Steward** sends corresponding `NYM` transaction to the ledger by calling consistently ``build_nym_request`` to build NYM request and ``sign_and_submit_request`` to send the created request.
1. **Sovrin Steward** creates connection request which contains created `DID` and `Nonce`. This nonce is just a big random number generated to track the unique connection request. A nonce is a random arbitrary number that can only be used one time.  When a connection request is accepted, the invitee digitally signs the nonce such that the inviter can match the response with a prior request.
1. **Sovrin Steward** sends connection request to **Faber**.
1. **Faber** accepts and stores connection request from **Sovrin Steward**
1. **Faber** creates wallet if he does not have yet.
1. **Faber** creates DID record in his wallet that he will use for secure interactions with **Sovrin Steward**.
1. **Faber** creates connection response which contains created `DID`, `Verkey` and `Nonce` got from received connection request.
1. **Faber** asks ledger for Verification key of **Sovrin Steward** by calling ``key_for_did``.
1. **Faber** anonymous encrypt connection response by calling ``anon_crypt`` using **Sovrin Steward** Verkey.
Anonymous-encryption schema is designed for sending of messages to a Recipient given its public key. 
Only the Recipient can decrypt these messages, using its private key. 
While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
1. **Faber** sends anonymous encrypted connection response to **Sovrin Steward**.
1. **Sovrin Steward** anonymous decrypt connection response by calling ``anon_decrypt`` and store **Faber** `DID` and `Verkey`.

```python
    (steward_faber_did, steward_faber_key) = await did.create_and_store_my_did(steward_wallet, "{}")
    nym_request = await ledger.build_nym_request(steward_did, steward_faber_did, steward_faber_key, None, role)
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, nym_request)
    
    connection_request = {
        'did': steward_faber_did,
        'nonce': 123456789
    }
    
    await wallet.create_wallet(pool_name, 'faber_wallet', None, None, None)
    faber_wallet = await wallet.open_wallet('faber_wallet', None, None)
    
    (faber_steward_did, faber_steward_key) = await did.create_and_store_my_did(faber_wallet, "{}")
    steward_faber_verkey = await did.key_for_did(pool_handle, faber_wallet, connection_request['did'])
    
    connection_response = json.dumps({
        'did': faber_steward_did,
        'verkey': faber_steward_key,
        'nonce': connection_request['nonce']
    })
    anoncrypted_connection_response = await crypto.anon_crypt(steward_faber_key, connection_response.encode('utf-8'))
    
    decrypted_connection_response = \
        (await crypto.anon_decrypt(steward_wallet, steward_faber_key, anoncrypted_connection_response)).decode("utf-8")
```

At this point **Faber** is connected to **Sovrin Steward** and can interact in a secure way and can trust the response from **Sovrin Steward** because 
(1) he connects to the current endpoint, 
(2) no replay - attack is possible, due to her random challenge, 
(3) he knows the verification key used to verify **Sovrin Steward** digital signature is the correct one because he just confirmed it on the ledger.

Note: **Every Side** must not use same DID's to establish other relationships.
By having independent pairwise relationships, we reduces the ability for others to correlate your activities across multiple interactions.

##### Getting self-sovereign identity

It is important to understand that created early **Faber** DID is not, in and of itself, the same thing as self-sovereign identity.
Its DID must be used only for secure communication with **Sovrin Steward**.
After connection is established **Faber** must create new DID that he will use as Verinym in the Ledger.
1. **Faber** creates new DID record in his wallet by calling ``did.create_and_store_my_did``.
1. **Faber** prepares message which contains created DID and Verkey.
1. **Faber** authenticated encrypt message by calling ``auth_crypt`` using Verkeys for secure communication with **Sovrin Steward**. 
Authenticated-encryption schema is designed for sending of a confidential message specifically for Recipient, using Sender's public key.
Using Recipient's public key, Sender can compute a shared secret key. Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
That shared secret key can be used to verify that the encrypted message was not tampered with, before eventually decrypting it.
1. **Faber** sends encrypted message to **Sovrin Steward**.
1. **Sovrin Steward** decrypts received message by calling ``auth_decrypt`` and sends corresponded NYM transaction to the ledger with `TRUST ANCHOR` role.

```python
    (faber_did, faber_key) = await did.create_and_store_my_did(faber_wallet, "{}")
    
    faber_did_info_json = json.dumps({
        'did': faber_did,
        'verkey': faber_key
    })
    authcrypted_faber_did_info_json = \
        await crypto.auth_crypt(faber_wallet, faber_steward_key, steward_faber_key, faber_did_info_json.encode('utf-8'))
        
    _, decrypted_faber_did_info_json = \
        await crypto.auth_decrypt(steward_handle, steward_faber_key, authcrypted_faber_did_info_json)
    faber_did_info = json.loads(decrypted_faber_did_info_json)
        
    nym_request = \
        await ledger.build_nym_request(steward_did, faber_did_info['did'], faber_did_info['verkey'], None, 'TRUST_ANCHOR')
    await ledger.sign_and_submit_request(pool_handle, steward_handle, steward_did, nym_request)
```
At this point **Faber** has DID related to his identity in the Ledger. 

Acme, Thrift and Government must pass the same Onboarding process to establish connection with Steward.

#### Claim Schemas Setup
Claim Schema - is the base semantic structure that describes list of attributes which one particular Claim may contain. 
Note: It's not possible to update existing Schema. So, if the Schema needs to be evolved, a new Schema with a new version or name needs to be created.
Claim Schema can be created and saved in the Ledger by any **Trust Anchor** by passing following steps:
1. **Trust Anchor** optionally creates new DID record in his wallet and sends corresponded NYM transaction to the ledger.
1. **Trust Anchor** creates **Schema**.
1. **Trust Anchor** sends corresponded Schema transaction to the ledger by calling consistently ``build_schema_request`` to build Schema request and ``sign_and_submit_request`` to send created request.

Here is **Government** creates and publishes **Employment History** Schema to the Ledger:
```python
(government_issuer_did, government_issuer_key) = await did.create_and_store_my_did(government_wallet, "{}")

employment_history_schema = {
    'name': 'Employment History',
    'version': '1.0',
    'attr_names': ['first_name', 'last_name', 'salary', 'employee_status', 'experience']
}
schema_request = await ledger.build_schema_request(government_issuer_did, json.dumps(employment_history_schema))
await ledger.sign_and_submit_request(pool_handle, government_wallet, government_issuer_did, schema_request)
```
The same way **Government** creates and publishes **HE Diploma** Schema to the Ledger:
```python
he_diploma_schema = {
    'name': 'HE Diploma',
    'version': '1.0',
    'attr_names': ['first_name', 'last_name', 'phone_number', 'degree', 'status', 'ssn', 'average']
}
schema_request = await ledger.build_schema_request(government_issuer_did, json.dumps(he_diploma_schema))
await ledger.sign_and_submit_request(pool_handle, government_wallet, government_issuer_did, schema_request)
```

At this point we have **Employment History** and **HE Diploma** Claim Schema published to the Ledger by **Government**.

#### Claim Definition Setup
Claim Definition - is a machine-readable definition of the particular Claim Schema.
Claim Definition is similar to keys that Issuer use for signing of Claims satisfied specific Claim Schema. 
Note: One Issuer DID can create just one Claim Definition for specific Claim Schema, but different Issuer DIDs can create numerous Claim Definitions for the same Claim Schema.
Note: It's not possible to update data in existing Claim Def. So, if a Claim Def needs to be evolved (for example, a key needs to be rotated), then a new Claim Def needs to be created by a new Issuer DID
Claim Definition can be created and saved in the Ledger by any **Trust Anchor** by following the next steps:
1. **Trust Anchor** optionally creates new DID record in his wallet and sends corresponded NYM transaction to the ledger.
1. **Trust Anchor** gets specific Schema from the Ledger by calling consistently ``build_get_schema_request`` to build GetSchema request and ``sign_and_submit_request`` to send created request.
1. **Trust Anchor** creates **Claim Definition** related to received Schema by calling ``issuer_create_and_store_claim_def``that returns generated public Claim Definition. Private Claim Definition part for this Schema will be stored in the wallet too, but it is impossible to read it directly. 
1. **Trust Anchor** sends corresponded ClaimDef transaction to the ledger by calling consistently ``build_claim_def_txn`` to build ClaimDef request and ``sign_and_submit_request`` to send created request.

Here is **Faber** creates and publishes Claim Definition for known **HE Diploma** Claim Schema to the Ledger:
```python
    (faber_issuer_did, faber_issuer_key) = await did.create_and_store_my_did(faber_wallet, "{}")
    await send_nym(pool_handle, faber_wallet, faber_did, faber_issuer_did, faber_issuer_key, None)
    
    he_diploma_schema_key = {
        'name': he_diploma_schema['name'],
        'version': he_diploma_schema['version'],
        'did': government_issuer_did
    }
    get_schema_data = json.dumps({
        'name': he_diploma_schema_key['name'],
        'version': he_diploma_schema_key['version']
    })
    get_schema_request = await ledger.build_get_schema_request(faber_issuer_did, he_diploma_schema_key['did'], get_schema_data)
    get_schema_response = await ledger.submit_request(pool_handle, get_schema_request)
    he_diploma_schema = json.loads(get_schema_response)['result']
    
    faber_he_diploma_claim_def_json = \
        await anoncreds.issuer_create_and_store_claim_def(faber_wallet, faber_issuer_did, json.dumps(received_he_diploma_schema), 'CL', False)
    faber_he_diploma_claim_def = json.loads(faber_he_diploma_claim_def_json)
    
    claim_def_request = await ledger.build_claim_def_txn(faber_issuer_did, claim_def['ref'], claim_def['signature_type'],
                                                         json.dumps(claim_def['data']))
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, faber_issuer_did, claim_def_request)
```

The same way **Acme** creates and publishes Claim Definition for **Employment History** Claim Schema to the Ledger.

At this point we have **Claim Definition** for **Employment History** Claim Schema published by Acme and **Claim Definition** for **HE Diploma** Claim Schema published by Faber. 

## Alice Gets a Transcript

At this point Alice is connected to Faber College and can interact in a secure way. 
