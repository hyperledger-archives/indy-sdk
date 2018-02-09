# Getting Started with Libindy

## A Developer Guide for an Implementation of the Libindy Code Base

![logo](https://raw.githubusercontent.com/hyperledger/indy-node/master/collateral/logos/indy-logo.png)

* [Getting Started with Indy](#getting-started-with-indy)
  * [What Indy Is, and Why it Matters](#what-indy-is-and-why-it-matters)
  * [What We'll Cover](#what-well-cover)
  * [Involving of Alice](#involving-of-alice)
  * [Infrastructure preparation](#infrastructure-preparation)
      * [Getting Trust Anchor credentials for Faber, Acme, Thrift and Government](#getting-trust-anchor-credentials-for-faber-acme-thrift-and-government)
      * [Connecting to Indy nodes pool](#connecting-to-indy-nodes-pool)
      * [Getting the ownership for Stewards's Verinym](#getting-the-ownership-for-stewardss-verinym)
      * [Onboarding Faber, Acme, Thrift and Government by Steward](#onboarding-faber-acme-thrift-and-government-by-steward)
        * [Connection establishment](#connection-establishment)
        * [Getting Verinym](#getting-verinym)
      * [Claim Schemas Setup](#claim-schemas-setup)
      * [Claim Definition Setup](#claim-definition-setup)
  * [Alice Gets a Transcript](#alice-gets-a-transcript)
  * [Apply for a Job](#apply-for-a-job)
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

Creation of DID known to the Ledger is **Identity Record** itself (NYM transaction). NYM transaction can be used for creation of new DIDs that ledger known, setting and rotation of verification key, setting and changing of roles. The most important fields of this transaction are dest (target DID), role (role of a user NYM record being created for) and verkey (target verification key). See [Requests](https://github.com/hyperledger/indy-node/blob/master/docs/requests.md) to get more information about supported ledger transactions.

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
  # Steward Agent
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
In our case one party will be always Trust Anchor. Real enterprise scenarios can use more complex version. 

##### Connection establishment
Let's look the process of connection establishment between **Steward** and **Faber College**.

1. **Faber** and **Steward** contact in a some way to initiate onboarding process. 
   It can be filling the form on web site or phone call.
1. **Steward** creates new DID record in wallet by calling ``did.create_and_store_my_did`` that he will use for secure interactions with **Faber**.
    ```python 
    # Steward Agent
    (steward_faber_did, steward_faber_key) = await did.create_and_store_my_did(steward_wallet, "{}")
    ```
1. **Steward** sends corresponding `NYM` transaction to the Ledger by calling consistently ``ledger.build_nym_request`` to build NYM request and ``ledger.sign_and_submit_request`` to send the created request.
    ```python 
    # Steward Agent
    nym_request = await ledger.build_nym_request(steward_did, steward_faber_did, steward_faber_key, None, role)
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, nym_request)
    ```
1. **Steward** creates connection request which contains created `DID` and `Nonce`. 
   This nonce is just a big random number generated to track the unique connection request. 
   A nonce is a random arbitrary number that can only be used one time.  
   When a connection request is accepted, the invitee digitally signs the nonce such that the inviter can match the response with a prior request.
    ```python 
    # Steward Agent
    connection_request = {
        'did': steward_faber_did,
        'nonce': 123456789
    }
    ```
1. **Steward** sends connection request to **Faber**.
1. **Faber** accepts connection request from **Steward**.
1. **Faber** creates wallet if it does not exist yet.
    ```python 
    # Faber Agent
    await wallet.create_wallet(pool_name, 'faber_wallet', None, None, None)
    faber_wallet = await wallet.open_wallet('faber_wallet', None, None)
    ```
1. **Faber** creates new DID record in his wallet by calling ``did.create_and_store_my_did`` that he will use for secure interactions with **Steward**.
    ```python 
    # Faber Agent
    (faber_steward_did, faber_steward_key) = await did.create_and_store_my_did(faber_wallet, "{}")
    ```
1. **Faber** creates connection response which contains created `DID`, `Verkey` and `Nonce` from received connection request.
    ```python 
    # Faber Agent
    connection_response = json.dumps({
        'did': faber_steward_did,
        'verkey': faber_steward_key,
        'nonce': connection_request['nonce']
    })
    ```
1. **Faber** asks ledger for Verification key of **Steward's** DID by calling ``did.key_for_did``.
    ```python 
    # Faber Agent
    steward_faber_verkey = await did.key_for_did(pool_handle, faber_wallet, connection_request['did'])
    ```
1. **Faber** anonymous encrypts connection response by calling ``crypto.anon_crypt`` with **Steward** Verkey.
   Anonymous-encryption schema is designed for sending of messages to a Recipient given its public key. 
   Only the Recipient can decrypt these messages, using its private key. 
   While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
    ```python 
    # Faber Agent
    anoncrypted_connection_response = await crypto.anon_crypt(steward_faber_verkey, connection_response.encode('utf-8'))
    ```
1. **Faber** sends anonymous encrypted connection response to **Steward**.
1. **Steward** anonymous decrypts connection response by calling ``crypto.anon_decrypt``.
    ```python 
    # Steward Agent
    decrypted_connection_response = \
        (await crypto.anon_decrypt(steward_wallet, steward_faber_key, anoncrypted_connection_response)).decode("utf-8")
    ```
1. **Steward** authenticates **Faber** by comparision of Nonce.
    ```python 
    # Steward Agent
    assert connection_request['nonce'] == decrypted_connection_response['nonce']
    ```
1. **Steward** sends `NYM` transaction for **Faber's** DID to the Ledger. 
Please note that despite Steward is sender of this transaction the owner of DID will be Faber as it uses Verkey provided by Faber.
    ```python        
    # Steward Agent 
    nym_request = await ledger.build_nym_request(steward_did, decrypted_connection_response['did'], decrypted_connection_response['verkey'], None, role)
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, nym_request)
    ```

At this point **Faber** is connected to **Steward** and can interact in a secure way and can trust the response from **Steward** because 
(1) he connects to the current endpoint, 
(2) no replay - attack is possible, due to her random challenge, 
(3) he knows the verification key used to verify **Steward** digital signature is the correct one because he just confirmed it on the ledger.

Note: **Every Side** must not use same DID's to establish other relationships.
By having independent pairwise relationships, we reduces the ability for others to correlate your activities across multiple interactions.

##### Getting Verinym

It is important to understand that created early **Faber** DID is not, in and of itself, the same thing as self-sovereign identity.
This DID must be used only for secure interaction with **Steward**.
After the connection is established **Faber** must create new DID record that he will use as Verinym in the Ledger.
1. **Faber** creates new DID in his wallet by calling ``did.create_and_store_my_did``.
    ```python        
    # Faber Agent 
    (faber_did, faber_key) = await did.create_and_store_my_did(faber_wallet, "{}")
    ```
1. **Faber** prepares the message that will contain created DID and Verkey.
    ```python        
    # Faber Agent 
    faber_did_info_json = json.dumps({
        'did': faber_did,
        'verkey': faber_key
    })
    ```
1. **Faber** authenticated encrypts the message by calling ``crypto.auth_crypt`` using Verkeys created for secure communication with **Steward**. 
   Authenticated-encryption schema is designed for sending of a confidential message specifically for Recipient, using Sender's public key.
   Using Recipient's public key, Sender can compute a shared secret key. Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
   That shared secret key can be used to verify that the encrypted message was not tampered with, before eventually decrypting it.
    ```python        
    # Faber Agent 
    authcrypted_faber_did_info_json = \
        await crypto.auth_crypt(faber_wallet, faber_steward_key, steward_faber_key, faber_did_info_json.encode('utf-8'))
    ```
1. **Faber** sends encrypted message to **Steward**.
1. **Steward** decrypts received message by calling ``crypto.auth_decrypt``.
    ```python        
    # Steward Agent    
    sender_verkey, authdecrypted_faber_did_info_json = \
        await crypto.auth_decrypt(steward_handle, steward_faber_key, authcrypted_faber_did_info_json)
    faber_did_info = json.loads(authdecrypted_faber_did_info_json)
    ```
1. **Steward** asks ledger for Verification key of **Faber's** DID by calling ``did.key_for_did``.
    ```python        
    # Steward Agent    
    faber_verkey = await did.key_for_did(pool_handle, from_wallet, faber_did_info['did'])
    ```
1. **Steward** authenticates **Faber** by comparision of Message Sender Verkey and **Faber** Verkey received from the Ledger.
    ```python        
    # Steward Agent    
    assert sender_verkey == faber_verkey
    ```
1. **Steward** sends corresponded NYM transaction to the Ledger with `TRUST ANCHOR` role.
Please note that despite Steward is sender of this transaction the owner of DID will be Faber as it uses Verkey provided by Faber.
    ```python    
    # Steward Agen
    nym_request = await ledger.build_nym_request(steward_did, decrypted_faber_did_info_json['did'],
                                                 decrypted_faber_did_info_json['verkey'], None, 'TRUST_ANCHOR')
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, nym_request)
    ```
At this point **Faber** has DID related to his identity in the Ledger. 

**Acme**, **Thrift Bank**, and **Government** must pass the same Onboarding process connection establishment with **Steward**.

#### Claim Schemas Setup
**Claim Schema** - is the base semantic structure that describes the list of attributes which one particular Claim can contain. 

Note: It's not possible to update existing Schema. So, if the Schema needs to be evolved, a new Schema with a new version or name needs to be created.

**Claim Schema** can be created and saved in the Ledger by any **Trust Anchor**.
Here is **Government** creates and publishes **Transcript** Claim Schema to the Ledger:
1. **Trust Anchor** optionally creates new DID record in his wallet and sends corresponded NYM transaction to the Ledger.
    ```python
    # Government Agent 
    (government_issuer_did, government_issuer_key) = await did.create_and_store_my_did(government_wallet, "{}")
    nym_request = await ledger.build_nym_request(government_did, government_issuer_did, government_issuer_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, government_handle, government_did, nym_request)
    ```
1. **Trust Anchor** prepares **Claim Schema**.
    ```python
    # Government Agent 
    transcript_schema = {
        'name': 'Transcript',
        'version': '1.2',
        'attr_names': ['first_name', 'last_name', 'degree', 'status', 'year', 'average', 'ssn']
    }
    ```
1. **Trust Anchor** sends corresponded Schema transaction to the Ledger by calling consistently ``ledger.build_schema_request`` to build Schema request and ``ledger.sign_and_submit_request`` to send created request.
    ```python
    # Government Agent 
    schema_request = await ledger.build_schema_request(government_issuer_did, json.dumps(transcript_schema))
    await ledger.sign_and_submit_request(pool_handle, government_wallet, government_issuer_did, schema_request)
    ```

The same way **Government** creates and publishes **Job-Certificate** Claim Schema to the Ledger:
```python    
  # Government Agent 
  job_certificate_schema = {
      'name': 'Job-Certificate',
      'version': '0.2',
      'attr_names': ['first_name', 'last_name', 'salary', 'employee_status', 'experience']
  }
  schema_request = await ledger.build_schema_request(government_issuer_did, json.dumps(to the Ledger))
  await ledger.sign_and_submit_request(pool_handle, government_wallet, government_issuer_did, schema_request)
```

At this point we have **Transcript** and **Job-Certificate** Claim Schemas published by **Government** to the Ledger.

#### Claim Definition Setup
Claim Definition is similar to keys that Issuer use for signing of Claims satisfied specific Claim Schema. 

Note: It's not possible to update data in existing Claim Def. So, if a ClaimDef needs to be evolved (for example, a key needs to be rotated), then a new Claim Def needs to be created by a new Issuer DID.

**Claim Definition** can be created and saved in the Ledger by any **Trust Anchor**.
Here is **Faber** creates and publishes Claim Definition for known **Transcript** Claim Schema to the Ledger.
1. **Trust Anchor** optionally creates new DID record in his wallet and sends corresponded NYM transaction to the Ledger.
    ```python
    # Faber Agent 
    (faber_issuer_did, faber_issuer_key) = await did.create_and_store_my_did(faber_wallet, "{}")
    nym_request = await ledger.build_nym_request(faber_did, faber_issuer_did, faber_issuer_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, faber_wallet, faber_did, nym_request)  
    ```
1. **Trust Anchor** gets specific **Claim Schema** from the Ledger by calling consistently ``ledger.build_get_schema_request`` to build GetSchema request and ``ledger.sign_and_submit_request`` to send created request.
    ```python
    # Faber Agent 
    get_schema_data = json.dumps({
        'name': 'Transcript',
        'version': '1.2'
    })
    get_schema_request = await ledger.build_get_schema_request(faber_issuer_did, government_issuer_did, get_schema_data)
    get_schema_response = await ledger.submit_request(pool_handle, get_schema_request)
    transcript_schema = json.loads(get_schema_response)['result']
    ```
1. **Trust Anchor** creates **Claim Definition** related to received **Claim Schema** by calling ``anoncreds.issuer_create_and_store_claim_def``that returns generated public Claim Definition. 
   Private Claim Definition part for this **Claim Schema** will be stored in the wallet too, but it is impossible to read it directly. 
    ```python
    # Faber Agent 
    faber_transcript_claim_def_json = \
        await anoncreds.issuer_create_and_store_claim_def(faber_wallet, faber_issuer_did, json.dumps(transcript_schema), 'CL', False)
    faber_transcript_claim_def = json.loads(faber_transcript_claim_def_json)
    ```
1. **Trust Anchor** sends corresponded ClaimDef transaction to the Ledger by calling consistently ``ledger.build_claim_def_txn`` to build ClaimDef request and ``ledger.sign_and_submit_request`` to send created request.
    ```python
    # Faber Agent     
    claim_def_request = \
        await ledger.build_claim_def_txn(faber_issuer_did, faber_transcript_claim_def['ref'], 
                                         faber_transcript_claim_def['signature_type'], json.faber_transcript_claim_def(claim_def['data']))
    await ledger.sign_and_submit_request(pool_handle, faber_wallet, faber_issuer_did, claim_def_request)
    ```
    
The same way **Acme** creates and publishes Claim Definition for known **Job-Certificate** Claim Schema to the Ledger.
```python
  # Acme Agent 
  (acme_issuer_did, acme_issuer_key) = await did.create_and_store_my_did(acme_wallet, "{}")
  await send_nym(pool_handle, acme_wallet, acme_did, acme_issuer_did, acme_issuer_key, None)
    
  get_schema_data = json.dumps({
      'name': 'Job-Certificate',
      'version': '0.2'
  })
  get_schema_request = await ledger.build_get_schema_request(acme_issuer_did, government_issuer_did, get_schema_data)
  get_schema_response = await ledger.submit_request(pool_handle, get_schema_request)
  job_certificate_schema = json.loads(get_schema_response)['result']
    
  acme_job_certificate_claim_def_json = \
      await anoncreds.issuer_create_and_store_claim_def(acme_wallet, acme_issuer_did, json.dumps(job_certificate_schema), 'CL', False)
  acme_transcript_claim_def = json.loads(acme_job_certificate_claim_def_json)
    
  claim_def_request = \
      await ledger.build_claim_def_txn(acme_issuer_did, acme_transcript_claim_def['ref'], 
                                       acme_transcript_claim_def['signature_type'], acme_transcript_claim_def.dumps(claim_def['data']))
  await ledger.sign_and_submit_request(pool_handle, acme_wallet, acme_issuer_did, claim_def_request)
```

At this point we have **Claim Definition** for **Job-Certificate** Claim Schema published by **Acme** and
 **Claim Definition** for **Transcript** Claim Schema published by **Faber**. 

## Alice Gets a Transcript

A claim is a piece of information about an identity -- a name, an age, a credit score… It is information claimed to be true.
In this case, the claim is named, "Transcript".

Claims are offered by an issuer. 
An issuer may be any identity owner known to the Ledger and any issuer may issue a claim about any identity owner it can identify.
The usefulness and reliability of a claim are tied to the reputation of the issuer with respect to the claim at hand. 
For Alice to self-issue a claim that she likes chocolate ice cream may be perfectly reasonable, but for her to self-issue a claim that she graduated from Faber College should not impress anyone. 

As we mentioned in [Involving of Alice](#involving-of-alice) **Alice** graduate **Faber College**.
After Alice had established connection with **Faber College**, she got Claim Offer about the issuance of **Transcript** Claim.
Alice stores it in her wallet.
```python
  # Alice Agent 
  transcript_claim_offer = {
      'issuer_did': faber_issuer_did,
      'schema_key': transcript_schema_key
  }
  await anoncreds.prover_store_claim_offer(alice_wallet, json.dumps(transcript_claim_offer))
```
Note: All messages sent between actors are encrypted using `Authenticated-encryption` scheme.

The value of this **Transcript** Claim is that it is provably issued by **Faber College**.
 
**Alice** wants to see the attributes the **Transcript** Claim contains. 
These attributes are known because a Claim Schema for **Transcript** has been written to the Ledger.
```python
  # Alice Agent 
  get_schema_data = json.dumps({
      'name': transcript_claim_offer['transcript_claim_offer']['name'],
      'version': transcript_claim_offer['transcript_claim_offer']['version']
  })
  get_schema_request = await ledger.build_get_schema_request(alice_faber_did, transcript_claim_offer['transcript_claim_offer']['did'], get_schema_data)
  get_schema_response = await ledger.submit_request(pool_handle, get_schema_request)
  transcript_schema = json.loads(get_schema_response)['result']
  
  print(transcript_schema['data'])
  # Transcript Schema:
  {
      'name': 'Transcript',
      'version': '1.2',
      'attr_names': ['first_name', 'last_name', 'degree', 'status', 'year', 'average', 'ssn']
  }
```

However, **Transcript** Claim has not been delivered to Alice yet in a usable form.
Alice wants to use that Claim. 
To get it, Alice needs to request it, but first she must create Master Secret.
   
Note: Master Secret is an item of Private Data used by a Prover to guarantee that a claim uniquely applies to them. 
The Master Secret is an input that combine data from multiple Claims in order to prove that the Claims have a common subject (the Prover). 
A Master Secret should be known only to the Prover. 
Alice creates Master Secret in her wallet.
```python
  # Alice Agent 
  alice_master_secret_name = 'alice_master_secret'
  await anoncreds.prover_create_master_secret(alice_wallet, alice_master_secret_name)
```

Also Alice needs to get Claim Definition corresponded to issuer_did and schema_key in **Transcript** Claim Offer.
```python
  # Alice Agent 
  get_claim_def_request = await ledger.build_get_claim_def_txn(alice_faber_did, transcript_schema['seqNo'], 'CL', faber_issuer_did)
  get_claim_def_response = await ledger.submit_request(pool_handle, get_claim_def_request)
  transcript_claim_def = json.loads(get_claim_def_response)['result']
```

Now Alice has everything to create Claim Request of issuance of **Faber Transcript** Claim.
```python   
  # Alice Agent 
  transcript_claim_request_json = \
          await anoncreds.prover_create_and_store_claim_req(alice_wallet, alice_faber_did, transcript_claim_offer,
                                                            json.dumps(transcript_claim_def), alice_master_secret)
```

**Faber** prepares Raw and Encoded values for each attribute in **Transcript** Claim Schema.
**Faber** creates **Transcript** Claim for Alice.
```python
  # Faber Agent 
  transcript_claim_values = json.dumps({
      'first_name': ['Alice', '1139481716457488690172217916278103335'], # 
      'last_name': ['Garcia', '5321642780241790123587902456789123452'],
      'degree': ['Bachelor of Science, Marketing', '12434523576212321'],
      'status': ['graduated', '2213454313412354'],
      'ssn': ['123-45-6789', '3124141231422543541'],
      'year': ['2015', '2015'],
      'average': ['5', '5']
  })
    
  (_, transcript_claim_json) = \
      await anoncreds.issuer_create_claim(faber_wallet, transcript_claim_request_json, transcript_claim_values, -1)
```

Now **Transcript** Claim has been issued. Alice stores it in her wallet.
```python
  # Alice Agent 
  await anoncreds.prover_store_claim(alice_wallet, transcript_claim_json, None)
```

Alice has it in her possession, in much the same way that she would hold a physical transcript that had been mailed to her.

## Apply for a Job
At some time in the future, Alice would like to work for the fictional company, Acme Corp. 
Normally she would browse to their website, where she would click on a hyperlink to apply for a job. 
Her browser would download a connection request which her Indy app would open; this would trigger a prompt to Alice, asking her to accept the connection with Acme Corp. 
Because we’re using a Indy-SDK, the process is different, but the steps are the same. 
The process of connection establishment is the same as when Faber was accepting Steward connection request.

After Alice had established connection with Acme, she got **Job-Application** Proof Request.
A proof request is a request made by the party who needs verifiable proof of having certain attributes and solving of predicates that can be provided by other verified claims.

In this case, Acme Corp is requesting that Alice provide a **Job Application**. 
The Job Application requires a name, degree, status, ssn and also the satisfaction of the condition about the average mark.
In this case, **Job-Application** Proof Request looks like:
```
  # Acme Agent 
  job_application_proof_request_json = json.dumps({
      'nonce': '1432422343242122312411212',
      'name': 'Job-Application',
      'version': '0.1',
      'requested_attrs': {
        'attr1_referent': {
              'name': 'first_name'
          },
          'attr2_referent': {
              'name': 'last_name'
          },
          'attr3_referent': {
              'name': 'degree',
              'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': transcript_schema_key}]
          },
          'attr4_referent': {
              'name': 'status',
              'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': transcript_schema_key}]
          },
          'attr5_referent': {
              'name': 'ssn',
              'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': transcript_schema_key}]
          },
          'attr6_referent': {
              'name': 'phone_number'
          }
      },
      'requested_predicates': {
          'predicate1_referent': {
              'attr_name': 'average',
              'p_type': '>=',
              'value': 4,
              'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': transcript_schema_key}]
          }
      }
  })
```

Notice that some attributes are verifiable and some are not. 
The proof request says that SSN, degree, and graduation status in the Claim must be formally asserted by an issuer and schema_key. 
Notice also that the first_name, last_name and phone_number are not required to be verifiable.
By not tagging these claims with a verifiable status, Acme’s claim request is saying it will accept any Alice’s own claim about her names and phone numbers.

To show Claims that Alice can use for creating of Proof for **Job-Application** Proof Request Alice calls `anoncreds.prover_get_claims_for_proof_req`.
```python
  # Alice Agent 
  claims_for_proof_request = \
      json.loads(await anoncreds.prover_get_claims_for_proof_req(alice_wallet, job_application_proof_request_json))
```

Alice has only one claim that meets proof requirements for this **Job Application**.
```python
  # Alice Agent 
  {
    'referent': 'Transcript Claim Referent',
    'schema_key': transcript_schema_key, 
    'attrs': {
        'first_name': 'Alice', 
        'last_name': 'Garcia', 
        'status': 'graduated', 
        'degree': 'Bachelor of Science, Marketing', 
        'ssn': '123-45-6789',
        'year': '2015', 
        'average': '5'
    }, 
    'issuer_did': faber_issuer_did,
    'revoc_reg_seq_no': None, 
  }
```

Now Alice can divide attributes into the three groups:
  1. attributes values of which will be revealed
  1. attributes values of which will be unrevealed
  1. attributes for which creating of verifiable proof is not required

For **Job-Application** Proof Request Alice divided attributes as follows: 
```python
  # Alice Agent 
  job_application_requested_claims_json = json.dumps({
      'self_attested_attributes': {
          'attr1_referent': 'Alice',
          'attr2_referent': 'Garcia',
          'attr6_referent': '123-45-6789'
      },
      'requested_attrs': {
          'attr3_referent': ['Transcript Claim Referent', True],
          'attr4_referent': ['Transcript Claim Referent', True],
          'attr5_referent': ['Transcript Claim Referent', True]
      },
      'requested_predicates': {'predicate1_referent': 'Transcript Claim Referent'}
  })
```

In addition, Alice must get Claim Schema and corresponded Claim Definition for each used Claim, the same way, as on the step creation of Claim Request.

Now Alice has everything to create Proof for **Acme Job-Application** Proof Request.
```python
  # Alice Agent 
  apply_job_proof_json = \
      await anoncreds.prover_create_proof(alice_wallet, job_application_proof_request_json, job_application_requested_claims_json,
                                          schemas_json, alice_master_secret_name, claim_defs_json, revoc_regs_json)
```

When **Acme** inspects received Proof he will see following structure:
```
  # Acme Agent 
  {
      'requested_proof': {
          'revealed_attrs': {
              'attr4_referent': ['Transcript Claim Referent', 'graduated', '2213454313412354'], 
              'attr5_referent': ['Transcript Claim Referent', '123-45-6789', '3124141231422543541'], 
              'attr3_referent': ['Transcript Claim Referent', 'Bachelor of Science, Marketing', '12434523576212321']
          },
          'self_attested_attrs': {
              'attr1_referent': 'Alice', 
              'attr2_referent': 'Garcia',
              'attr6_referent': '123-45-6789'
          }, 
          'unrevealed_attrs': {},
          'predicates': {
              'predicate1_referent': 'Transcript Claim Referent'
          }
      },
      'proof' : {} # Validity Proof that Acme can check
      'identifiers' : { # Identifiers of claims were used for Proof building
          'Transcript Claim Referent': {
              'issuer_did': faber_issuer_did, 
              'rev_reg_seq_no': None, 
              'schema_key': transcript_schema_key
          }
      } 
  }
```

**Acme** got all requested attributes.
Now **Acme** wants to check Validity Proof.
To do it **Acme** firstly must get every Claim Schema and corresponded Claim Definition for each identifier presented in Proof, the same way, as it was doing Alice. 
Now **Acme** has everything to check **Job-Application** Proof from Alice.
 ```python
  # Acme Agent 
  assert await anoncreds.verifier_verify_proof(job_application_proof_request_json, apply_job_proof_json, 
                                               schemas_json, claim_defs_json, revoc_regs_json)
```

Here, we’ll assume the application is accepted, and Alice ends up getting the job.
When Alice inspects her connection with Acme a week later, she sees that a new Claim Offer is available.
```python
  # Alice Agent 
  job_certificate_claim_offer = {
      "issuer_did": acme_issuer_did,
      "schema_key": job_certificate_schema_key
  }
  await anoncreds.prover_store_claim_offer(alice_wallet, job_certificate_claim_offer)
```

## Apply for a Loan 

Now that Alice has a job, she’d like to apply for a loan. That will require proof of employment. 
She can get this from the **Job-Certificate** Claim offered by Acme. 
Alice goes through a familiar sequence of interactions. 

First she creates Claim Request.
 ```python  
  # Alice Agent 
  job_certificate_claim_request_json = \
      await anoncreds.prover_create_and_store_claim_req(alice_wallet, alice_acme_did, job_certificate_claim_offer,
                                                        json.dumps(job_certificate_claim_def), alice_master_secret)
 ```
 
 Acme issues **Job-Certificate** Claim for Alice.
 ```python
  # Acme Agent 
  job_certificate_claim_values_json = json.dumps({
      'first_name': ['Alice', '245712572474217942457235975012103335'],
      'last_name': ['Garcia', '312643218496194691632153761283356127'],
      'employee_status': ['Permanent', '2143135425425143112321314321'],
      'salary': ['2400', '2400'],
      'experience': ['10', '10']
  })
  _, job_certificate_claim_json = \
      await anoncreds.issuer_create_claim(acme_wallet, job_certificate_claim_request_json, job_certificate_claim_values_json, -1)
```

Now the **Job-Certificate** Claim has been issued, and Alice now has it in her possession. 
Alice stores **Job-Certificate** Claim in her wallet.
```python
  # Alice Agent 
  await anoncreds.prover_store_claim(alice_wallet, job_certificate_claim_json, None)
```
 
She can use it when she applies for her loan, in much the same way that she used her transcript when applying for a job.
 
There is a disadvantage in this approach to data sharing though, -- it may disclose more data than what is strictly necessary. If all Alice needs to do is provide proof of employment, this can be done with an anonymous credential instead. Anonymous credentials may prove certain predicates without disclosing actual values (e.g., Alice is employed full-time, with a salary greater than X, along with her hire date, but her actually salary remains hidden). A compound proof can be created, drawing from claims from both Faber College and Acme Corp, that discloses only what is necessary.
 
Alice now establishes connection with Thrift Bank.
 
Alice gets **Loan-Application-Basic** Proof Request from Thrift Bank that looks like:
```python
  # Thrift Agent 
  apply_loan_proof_request_json = json.dumps({
      'nonce': '123432421212',
      'name': 'Loan-Application-Basic',
      'version': '0.1',
      'requested_attrs': {
          'attr1_referent': {
              'name': 'employee_status',
              'restrictions': [{'issuer_did': acme_issuer_did, 'schema_key': job_certificate_schema_key}]
          }
      },
      'requested_predicates': {
          'predicate1_referent': {
              'attr_name': 'salary',
              'p_type': '>=',
              'value': 2000,
              'restrictions': [{'issuer_did': acme_issuer_did, 'schema_key': job_certificate_schema_key}]
          },
          'predicate2_referent': {
              'attr_name': 'experience',
              'p_type': '>=',
              'value': 1,
              'restrictions': [{'issuer_did': acme_issuer_did, 'schema_key': job_certificate_schema_key}]
          }
      }
  })
```

Alice has only one claim that meets proof requirements for this **Loan-Application-Basic** Proof Request.
```python
  # Alice Agent 
  {
      'referent': 'Job-Certificate Claim Referent',
      'revoc_reg_seq_no': None, 
      'schema_key': job_certificate_schema_key, 
      'attrs': {
          'employee_status': 'Permanent', 
          'last_name': 'Garcia', 
          'experience': '10', 
          'first_name': 'Alice',
           'salary': '2400'
      }, 
      'issuer_did': acme_issuer_did
  }
```

For **Loan-Application-Basic** Proof Request Alice divided attributes as follows: 
```python
  # Alice Agent 
  apply_loan_requested_claims_json = json.dumps({
      'self_attested_attributes': {},
      'requested_attrs': {
          'attr1_referent': ['Job-Certificate Claim Referent', True]
      },
      'requested_predicates': {
          'predicate1_referent': 'Job-Certificate Claim Referent',
          'predicate2_referent': 'Job-Certificate Claim Referent'
      }
  })
```

Alice creates Proof for **Loan-Application-Basic** Proof Request.
```python
  # Alice Agent 
  apply_loan_proof_json = \
      await anoncreds.prover_create_proof(alice_wallet, apply_loan_proof_request_json, apply_loan_requested_claims_json, 
                                          schemas_json, alice_master_secret_name, claim_defs_json, revoc_regs_json)
```

Alice sends just the **Loan-Application-Basic** proof to the bank. 
This allows her to minimize the PII (personally identifiable information) that she has to share when all she's trying to do right now is prove basic eligibility.

When **Thrift** inspects received Proof he will see following structure:
```
  # Thrift Agent 
  {
      'requested_proof': {
          'revealed_attrs': {
              'attr1_referent': ['Job-Certificate Claim Referent', 'Permanent', '2143135425425143112321314321'], 
          },
          'self_attested_attrs': {}, 
          'unrevealed_attrs': {},
          'predicates': {
              'predicate1_referent': 'Job-Certificate Claim Referent',
              'predicate2_referent': 'Job-Certificate Claim Referent'
          }
      },
      'proof' : {} # Validity Proof that Thrift can check
      'identifiers' : { # Identifiers of claims were used for Proof building
          'Transcript Claim Referent': {
              'issuer_did': faber_issuer_did, 
              'rev_reg_seq_no': None, 
              'schema_key': transcript_schema_key
          }
      } 
  }
```

**Thrift Bank** successfully verified **Loan-Application-Basic** Proof from Alice.
```python
  # Thrift Agent 
    assert await anoncreds.verifier_verify_proof(apply_loan_proof_request_json, apply_loan_proof_json,
                                                 schemas_json, claim_defs_json, revoc_regs_json)
```

Thrift Bank sends the second Proof Request where Alice needs to share her personal information with the bank.
```python
  # Thrift Agent 
  apply_loan_kyc_proof_request_json = json.dumps({
      'nonce': '123432421212',
      'name': 'Loan-Application-KYC',
      'version': '0.1',
      'requested_attrs': {
          'attr1_referent': {'name': 'first_name'},
          'attr2_referent': {'name': 'last_name'},
          'attr3_referent': {'name': 'ssn'}
      },
      'requested_predicates': {}
  })
```

Alice has two claim that meets proof requirements for this **Loan-Application-KYC** Proof Request.
```python
  # Alice Agent 
  {
    'referent': 'Transcript Claim Referent',
    'schema_key': transcript_schema_key, 
    'attrs': {
        'first_name': 'Alice', 
        'last_name': 'Garcia', 
        'status': 'graduated', 
        'degree': 'Bachelor of Science, Marketing', 
        'ssn': '123-45-6789',
        'year': '2015', 
        'average': '5'
    }, 
    'issuer_did': faber_issuer_did,
    'revoc_reg_seq_no': None, 
  },
  {
      'referent': 'Job-Certificate Claim Referent',
      'revoc_reg_seq_no': None, 
      'schema_key': job_certificate_schema_key, 
      'attrs': {
          'employee_status': 'Permanent', 
          'last_name': 'Garcia', 
          'experience': '10', 
          'first_name': 'Alice',
          'salary': '2400'
      }, 
      'issuer_did': acme_issuer_did
  }
```

For **Loan-Application-KYC** Proof Request Alice divided attributes as follows: 
```python
  # Alice Agent 
  apply_loan_kyc_requested_claims_json = json.dumps({
      'self_attested_attributes': {},
      'requested_attrs': {
          'attr1_referent': ['Job-Certificate Claim Referent', True],
          'attr2_referent': ['Job-Certificate Claim Referent', True],
          'attr3_referent': ['Transcript Claim Referent', True]
      },
      'requested_predicates': {}
  })
```

Alice creates Proof for **Loan-Application-KYC** Proof Request.
```python
  # Alice Agent 
  apply_loan_kyc_proof_json = \
      await anoncreds.prover_create_proof(alice_wallet, apply_loan_kyc_proof_request_json, apply_loan_kyc_requested_claims_json, 
                                          schemas_json, alice_master_secret_name, claim_defs_json, revoc_regs_json)
```

When **Thrift** inspects received Proof he will see following structure:
```
  # Thrift Agent 
  {
      'requested_proof': {
          'revealed_attrs': {
              'attr1_referent': ['Transcript Claim Referent', '123-45-6789', '3124141231422543541'], 
              'attr1_referent': ['Job-Certificate Claim Referent', 'Alice', '245712572474217942457235975012103335'], 
              'attr1_referent': ['Job-Certificate Claim Referent', 'Garcia', '312643218496194691632153761283356127'], 
          },
          'self_attested_attrs': {}, 
          'unrevealed_attrs': {},
          'predicates': {}
      },
      'proof' : {} # Validity Proof that Thrift can check
      'identifiers' : { # Identifiers of claims were used for Proof building
          'Transcript Claim Referent': {
              'issuer_did': faber_issuer_did, 
              'rev_reg_seq_no': None, 
              'schema_key': transcript_schema_key
          },
          'Job-Certificate Claim Referent': {
              'issuer_did': acme_issuer_did, 
              'rev_reg_seq_no': None, 
              'schema_key': job_certificate_schema_key
          }
      } 
  }
```

**Thrift Bank** successfully validate **Loan-Application-KYC** Proof from Alice.
```python
  # Thrift Agent 
  assert await anoncreds.verifier_verify_proof(apply_loan_kyc_proof_request_json, apply_loan_kyc_proof_json,
                                               schemas_json, claim_defs_json, revoc_regs_json)
```

Both Alice Proofs have been successfully verified and she got loan from **Thrift Bank**.
     
## Explore the Code
Now that you've had a chance to see how the Libindy implementation works from the outside, perhaps you'd like to see how it works underneath, from code? 
If so, please run [Simulating Getting Started in the Jupiter](run-getting-started.md). 
You may need to be signed into GitHub to view this link. 
Also you can found source code [here](https://github.com/hyperledger/indy-sdk/blob/master/samples/python/src/getting_started.py)