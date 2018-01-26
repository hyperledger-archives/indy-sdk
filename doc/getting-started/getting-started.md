# Getting Started with Indy

## A Developer Guide for an Implementation of the Indy Code Base

![logo](https://raw.githubusercontent.com/hyperledger/indy-node/master/collateral/logos/indy-logo.png)

* [Getting Started with Indy](#getting-started-with-indy)
  * [What Indy Is, and Why it Matters](#what-indy-is-and-why-it-matters)
  * [What We'll Cover](#what-well-cover)
  * [Alice Gets a Transcript](#alice-gets-a-transcript)
  * [Install Indy](#install-indy)
  * [Evaluate a Connection Request](#evaluate-a-connection-request)
  * [Accept a Connection Request](#accept-a-connection-request)
  * [Test Secure Interaction](#test-secure-interaction)
  * [Apply for a Job](#apply-for-a-job)
  * [Apply for a Loan](#apply-for-a-loan)
  * [Explore the Code](#explore-the-code)
* [Appendix](#appendix)
  * [Faber College Configures Transcripts](#faber-college-configures-transcripts)
  * [Acme Corp Defines a Job-Application](#acme-corp-defines-a-job-application)

## What Indy is, and Why it Matters

The Indy code base (Indy) is a software ecosystem for private, secure, and powerful identity. Once it is implemented, it puts people — not the organizations that traditionally centralize identity — in charge of decisions about their own privacy and disclosure. This enables all kinds of rich innovation: connection contracts, revocation, novel payment workflows, asset and document management features, creative forms of escrow, curated reputation, integrations with other cool technologies, and so on.

Indy uses open-source, distributed ledger technology. These ledgers are a form of database that is provided cooperatively by a pool of participants, instead of by a giant database with a central admin. Data lives redundantly in many places, and it accrues in transactions orchestrated by many machines. Strong, industry-standard cryptography protects it. Best practices in key management and cybersecurity pervade its design. The result is a reliable, public source of truth under no single entity’s control, robust to system failure, resilient to hacking, and highly immune to subversion by hostile entities.

If the cryptography and blockchain details feel mysterious, fear not: this guide will help introduce you to key concepts within Indy. You’re starting in the right place.

## What We’ll Cover

Our goal is to introduce you to many of the concepts of Indy, and give you some idea of what happens behind the scenes to make it all work.

We are going to frame the exploration with a story. Alice, a graduate of the fictional Faber College, wants to apply for a job at the fictional company Acme Corp. As soon as she has the job, she wants to apply for a loan in Thrift Bank so she can buy a car. She would like to use her college transcript as proof of her education on the job application; once hired, Alice would like to use the fact of employment as evidence of her creditworthiness for the loan.

The sorts of identity and trust interactions required to pull this off are messy in the world today; they are slow, they violate privacy, and they are susceptible to fraud. We’ll show you how Indy is a quantum leap forward.

Ready?

## Alice Gets a Transcript

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

On the next step Faber, Acme, Thrift and Government should establish **Connection** with Steward. Each connection is actually a pair of Pairwise-Unique Identifiers (DIDs). The one DID is owned by one connection party and second by another. Both parties are know both DIDs and understand what connection this pair describes. We call process of establish connection **Onboarding**. In this tutorial we will describe the simple version of onboarding process. In our case one party will be always Trust Real enterprise scenarios can use more complex version. Our onboarding process will include the following steps:

1. Someone and **Trustor Anchor** contact in a some way to initiate onboarding process. It can be filling the form on web site or phone call.
1. Trust Anctor creates DID that he will use for this connection and sends corresponded NYM transaction to the ledger.