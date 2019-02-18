# Getting Started with Libvcx

## A Developer Guide for Building Indy Clients Using Libvcx

![logo](https://raw.githubusercontent.com/hyperledger/indy-node/master/collateral/logos/indy-logo.png)

* [Getting Started with Libvcx](#getting-started-with-libvcx)
  * [What Indy, Libindy and Libvcx are and Why They Matter](#what-indy-libindy-and-libvcx-are-and-why-they-matter)
  * [What We'll Cover](#what-well-cover)
  * [About Alice](#about-alice)
  * [Infrastructure Preparation](#infrastructure-preparation)
      * [Step 1: Getting Trust Anchor Credentials for Faber, Acme, Thrift and Government](#step-1-getting-trust-anchor-credentials-for-faber-acme-thrift-and-government)
      * [Step 2: Connecting to the Indy Nodes Pool](#step-2-connecting-to-the-indy-nodes-pool)
      * [Step 3: Getting the Ownership for Stewards's Verinym](#step-3-getting-the-ownership-for-stewardss-verinym)
      * [Step 4: Onboarding Faber, Acme, Thrift and Government by the Steward](#step-4-onboarding-faber-acme-thrift-and-government-by-steward)
        * [Connecting the Establishment](#connecting-the-establishment)
        * [Getting the Verinym](#getting-verinym)
      * [Step 5: Credential Schemas Setup](#step-5-credential-schemas-setup)
      * [Step 6: Credential Definition Setup](#step-6-credential-definition-setup)
  * [Alice Gets a Transcript](#alice-gets-a-transcript)
  * [Apply for a Job](#apply-for-a-job)
  * [Apply for a Loan](#apply-for-a-loan)
  * [Explore the Code](#explore-the-code)

## What Indy, Libindy and Libvcx are and Why They Matter

Indy provides a software ecosystem for private, secure, and powerful identity, and libindy enables clients for it. Indy puts people — not the organizations that traditionally centralize identity — in charge of decisions about their own privacy and disclosure. Libindy is a low level library that provides fine configuration, Libvcx is higher-level library on top of libindy which simplifies credential exchange. Libvcx is concentrated on hiding low-level details and increasing application development efficiency. This enables all kinds of rich innovation: connection contracts, revocation, novel payment workflows, asset and document management features, creative forms of escrow, curated reputation, integrations with other cool technologies, and so on.

Indy uses open-source, distributed ledger technology. These ledgers are a form of database that is provided cooperatively by a pool of participants, instead of by a giant database with a central admin. Data lives redundantly in many places, and it accrues in transactions orchestrated by many machines. Strong, industry-standard cryptography protects it. Best practices in key management and cybersecurity pervade its design. The result is a reliable, public source of truth under no single entity’s control, robust to system failure, resilient to hacking, and highly immune to subversion by hostile entities.

If the concepts of cryptography and blockchain details feel mysterious, fear not: this guide will help introduce you to key concepts within Indy. You’re starting in the right place.

## What We’ll Cover

Our goal is to introduce you to many of the concepts of Indy and give you some idea of what happens behind the scenes to make it all work.

We're going to frame the exploration with a story. Alice, a graduate of the fictional Faber College, wants to apply for a job at the fictional company Acme Corp. As soon as she has the job, she wants to apply for a loan in Thrift Bank so she can buy a car. She would like to use her college transcript as proof of her education on the job application and once hired, Alice would like to use the fact of employment as evidence of her creditworthiness for the loan.

The sorts of identity and trust interactions required to pull this off are messy in the world today; they are slow, they violate privacy, and they are susceptible to fraud. We’ll show you how Indy is a quantum leap forward.

Ready?

## About Alice

As a graduate of Faber College, Alice receives an alumni newsletter where she learns that her alma mater is offering digital transcripts. She logs in to the college alumni website and requests her transcript by clicking **Get Transcript**.  (Other ways to initiate this request might include scanning a QR code, downloading a transcript package from a published URL, etc.)

Alice doesn’t realize it yet, but to use this digital transcript she will need a new type of identity — not the traditional identity that Faber College has built for her in its on-campus database, but a new and portable one that belongs to her, independent of all past and future relationships, that nobody can revoke or co-opt or correlate without her permission. This is a **_self-sovereign identity_** and it is the core feature of Indy.

In normal contexts, managing a self-sovereign identity will require a tool such as a desktop or mobile application. It might be a standalone app or it might leverage a third party service provider that the ledger calls an **agency**. The Sovrin Foundation publishes reference versions of such tools. Faber College will have studied these requirements and will recommend an **_Indy app_** to Alice if she doesn’t already have one. This app will install as part of the workflow from the **Get Transcript** button.

When Alice clicks **Get Transcript**, she will download a file that holds an Indy **connection request**. This connection request file, having an .indy extension and associated with her Indy app, will allow her to establish a secure channel of communication with another party in the ledger ecosystem — Faber College.

So when Alice clicks **Get Transcript**, she will normally end up installing an app (if needed), launching it, and then being asked by the app whether she wants to accept a request to connect with Faber.

For this guide, however, we’ll be using an **VCX SDK API** (as provided by libvcx) instead of an app, so we can see what happens behind the scenes. We will pretend to be a particularly curious and technically adventurous Alice…

## Infrastructure Preparation

### Step 1: Getting Trust Anchor Credentials for Faber, Acme, Thrift and Government

Faber College and other actors have done some preparation to offer this service to Alice. To understand these steps let's start with some definitions.

The ledger is intended to store **Identity Records** that describe a **Ledger Entity**. Identity Records are public data and may include Public Keys, Service Endpoints, Credential Schemas, and Credential Definitions. Every **Identity Record** is associated with exactly one **DID** (Decentralized Identifier) that is globally unique and resolvable (via a ledger) without requiring any centralized resolution authority. To maintain privacy each **Identity Owner** can own multiple DIDs.

In this tutorial we will use two types of DIDs. The first one is a **Verinym**. A **Verinym** is associated with the **Legal Identity** of the **Identity Owner**. For example, all parties should be able to verify that some DID is used by a Government to publish schemas for some document type. The second type is a **Pseudonym** - a **Blinded Identifier** used to maintain privacy in the context of an ongoing digital relationship (**Connection**). If the Pseudonym is used to maintain only one digital relationship we will call it a Pairwise-Unique Identifier. We will use Pairwise-Unique Identifiers to maintain secure connections between actors in this tutorial.

The creation of a DID known to the Ledger is an **Identity Record** itself (NYM transaction). The NYM transaction can be used for creation of new DIDs that is known to that ledger, the setting and rotation of a verification key, and the setting and changing of roles. The most important fields of this transaction are `dest` (target DID), `role` (role of a user NYM record being created for) and the `verkey` (target verification key). See [Requests](https://github.com/hyperledger/indy-node/blob/master/docs/requests.md) to get more information about supported ledger transactions.

Publishing with a DID verification key allows a person, organization or thing, to verify that someone owns this DID as that person, organization or thing is the only one who knows the corresponding signing key and any DID-related operations requiring signing with this key.

Our ledger is public permissioned and anyone who wants to publish DIDs needs to get the role of **Trust Anchor** on the ledger. A **Trust Anchor** is a person or organization that the ledger already knows about, that is able to help bootstrap others. (It is *not* the same as what cybersecurity experts call a "trusted third party"; think of it more like a facilitator). See [Roles](https://github.com/hyperledger/indy-node/blob/master/docs/auth_rules.md) to get more information about roles.

**The first step towards being able to place transactions on the ledger involves getting the role of Trust Anchor on the ledger. Faber College, Acme Corp and Thrift Bank will need to get the role of Trust Anchor on the ledger so they can create Verinyms and Pairwise-Unique Identifiers to provide the service to Alice.**

Becoming a **Trust Anchor** requires contacting a person or organization who already has the **Trust Anchor** role on the ledger. For the sake of the demo, in our empty test ledger we have only NYMs with the **Steward** role, but all **Stewards** are automatically **Trust Anchors**.

#### Step 2: Connecting to the Indy Nodes Pool and initializing libvcx

We are ready to start writing the code that will cover Alice's use case from start to finish. It is important to note that for demo purposes it will be a single test that will contain the code intended to be executed on different agents. We will always point to what Agent is intended to execute each code part. Also we will use different wallets to store the DID and keys of different Agents. Let's begin.

The first code block will contain the code of the **Steward's** agent.

**To write and read the ledger's transactions after gaining the proper role, you'll need to make a connection to the Indy nodes pool. To make a connection to the different pools that exist, like the Sovrin pool or the [local pool we started by ourselves](../../README.md#how-to-start-local-nodes-pool-with-docker) as part of this tutorial, you'll need to set up a pool configuration.**

The list of nodes in the pool is stored in the ledger as NODE transactions. Libindy allows you to restore the actual list of NODE transactions by a few known transactions that we call genesis transactions. Each **Pool Configuration** is defined as a pair of pool configuration name and pool configuration JSON. The most important field in pool configuration json is the path to the file with the list of genesis transactions. Make sure this path is correct.

The code block below contains each of these items. Note how the comments denote that this is the code for the "Steward Agent."

```python
# Steward Agent

provisionConfig = {
  'agency_url':'http://localhost:8080',
  'agency_did':'VsKV7grR1BUE29mG2Fm2kX',
  'agency_verkey':'Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR',
  'wallet_name':'alice_wallet',
  'wallet_key':'123',
  'enterprise_seed':'000000000000000000000000Trustee1'
}

# Provision an agent and wallet, get back configuration details
config = await vcx_agent_provision(json.dumps(provisionConfig))
config = json.loads(config)

config['institution_name'] = 'alice'
config['institution_logo_url'] = 'http://robohash.org/456'
config['genesis_path'] = 'docker.txn'

# Initialize libvcx with new configuration
await vcx_init_with_config(json.dumps(config))

```

#### Step 3: Onboarding Faber, Acme, Thrift and Government by Steward

**Faber, Acme, Thrift and Government should now establish a Connection with the Steward.**

Each connection is actually a pair of Pairwise-Unique Identifiers (DIDs). The one DID is owned by one party to the connection and the second by another.

Both parties know both DIDs and understand what connection this pair describes.

The relationship between them is not shareable with others; it is unique to those two parties in that each pairwise relationship uses different DIDs.

We call the process of establish a connection **Onboarding**.

In this tutorial we will describe the simple version of onboarding process.
In our case, one party will always be the Trust Anchor. Real enterprise scenarios can use a more complex version.

##### Connecting the Establishment
Let's look the process of connection establishment between **Steward** and **Faber College**.

1. **Faber** and **Steward** contact in some way to initiate onboarding process.
   It can be filling the form on web site or a phone call.

2. **Faber** creates a connection to **Steward** and print out the invite details
    ```python
    connection_to_steward = await Connection.create('Steward')
    await connection_to_steward.connect(None)
    await connection_to_steward.update_state()
    details = await connection_to_steward.invite_details(False)
    invite_details = json.dumps(details)
    ```
    **Faber** sends invite details to **Steward** (eg. by e-mail).

3. **Steward** accepts connection request from **Faber**
    ```python
    connection_to_faber = await Connection.create_with_details('faber', invite_details)
    await connection_to_faber.connect(None)
    await connection_to_faber.update_state()
    ```

At this point **Faber** is connected to the **Steward** and can interact in a secure peer-to-peer way. **Faber** can trust the response is from **Steward** because:

* it connects to the current endpoint
* no replay - attack is possible, due to her random challenge
* it knows the verification key used to verify **Steward** digital signature is the correct one because it just confirmed it on the ledger

**Note:** All parties must not use the same DID's to establish other relationships.
By having independent pairwise relationships, you're reducing the ability for others to correlate your activities across multiple interactions.

**Acme**, **Thrift Bank**, and **Government** must pass the same Onboarding process connection establishment with **Steward**.

#### Step 4: Credential Schemas Setup

**Credential Schema** is the base semantic structure that describes the list of attributes which one particular Credential can contain.

**Note:** It's not possible to update an existing Schema. So, if the Schema needs to be evolved, a new Schema with a new version or name needs to be created.

A **Credential Schema** can be created and saved in the Ledger by any **Trust Anchor**.

Here is where the **Government** creates and publishes the **Transcript** Credential Schema to the Ledger:

```python
transcript_schema = await Schema.create(government_did, 'Transcript', '1.2', ['first_name', 'last_name', 'degree', 'status', 'year', 'average', 'ssn'], 0)
transcript_schema_id = await transcript_schema.get_schema_id()
```

In the same way **Government** creates and publishes the **Job-Certificate** Credential Schema to the Ledger:
```python
job_certificate_schema = await Schema.create(government_did, 'Job-Certificate', '0.2', ['first_name', 'last_name', 'salary', 'employee_status', 'experience'], 0)
job_certificate_schema_id = await job_certificate_schema.get_schema_id()
```

At this point we have the **Transcript** and the **Job-Certificate** Credential Schemas published by **Government** to the Ledger.

#### Step 5: Credential Definition Setup

**Credential Definition** is similar in that the keys that the Issuer uses for the signing of Credentials also satisfies a specific Credential Schema.

**Note** It's not possible to update data in an existing Credential Definition. So, if a `CredDef` needs to be evolved (for example, a key needs to be rotated), then a new Credential Definition needs to be created by a new Issuer DID.

A Credential Definition can be created and saved in the Ledger by any **Trust Anchor**. Here **Faber** creates and publishes a Credential Definition for the known **Transcript** Credential Schema to the Ledger.

```python
# Faber Agent
faber_transcript_cred_def = await CredentialDef.create(faber_did, 'Faber-Transcript', transcript_schema_id, 0)
faber_transcript_cred_def_id = await faber_transcript_cred_def.get_cred_def_id()
```

The same way **Acme** creates and publishes a **Credential Definition** for the known **Job-Certificate** Credential Schema to the Ledger.
```python
# Acme Agent
acme_job_certificate_cred_def = await CredentialDef.create(acme_did, 'ACME-Job-Certificate', job_certificate_schema_id, 0)
acme_job_certificate_cred_def_id = await acme_job_certificate_cred_def.get_cred_def_id()
```

At this point we have a **Credential Definition** for the **Job-Certificate** Credential Schema published by **Acme** and a
 **Credential Definition** for the **Transcript** Credential Schema published by **Faber**.

## Alice Gets a Transcript

A credential is a piece of information about an identity — a name, an age, a credit score… It is information claimed to be true. In this case, the credential is named, "Transcript".

Credentials are offered by an issuer.

An issuer may be any identity owner known to the Ledger and any issuer may issue a credential about any identity owner it can identify.

The usefulness and reliability of a credential are tied to the reputation of the issuer with respect to the credential at hand.
For Alice to self-issue a credential that she likes chocolate ice cream may be perfectly reasonable, but for her to self-issue a credential that she graduated from Faber College should not impress anyone.


As we mentioned in [About Alice](#about-alice), **Alice** graduated from **Faber College**.
After **Faber College** had established a connection with Alice, it created for her a Credential Offer about the issuance of the **Transcript** Credential.
```python
# Faber Agent

alice_degree_attrs = {
    'first_name': 'Alice',
    'last_name': 'Garcia',
    'degree': 'Bachelor of Science, Marketing',
    'status': 'graduated',
    'year': '2015',
    'average': '5',
    'ssn': '123-45-6789'
}

# Create an IssuerCredential object using the schema and credential definition
alice_degree = await IssuerCredential.create('alice_degree', alice_degree_attrs, faber_transcript_cred_def_id, 'cred', 1)

# Issue credential offer to alice
await alice_degree.send_offer(connection_to_alice)
await alice_degree.update_state()
```

**Note:** All messages sent between actors are encrypted using `Authenticated-encryption` scheme.

The value of this **Transcript** Credential is that it is provably issued by **Faber College**.

However, the **Transcript** Credential has not been delivered to Alice yet in a usable form. Alice wants to use that Credential.
To get it, Alice needs to request it.

```python
# Alice Agent

# Get credential offers from faber
offers = await Credential.get_offers(connection_to_faber)

# Create a credential object from the credential offer
credential = await Credential.create('credential', offers[0])

# Send credential request
await credential.send_request(connection_to_faber, 0)
```

**Faber** waits for **Alice** to send a credential request. Then issues the credential and sends it.
```python
# Faber Agent
await credential.send_credential(connection_to_alice)
```

Now the **Transcript** Credential has been issued. Alice waits for it and stores it in her wallet.
```python
# Alice Agent
while await credential.get_state() != State.Accepted:
    sleep(2)
    await credential.update_state()
```

Alice has it in her possession, in much the same way that she would hold a physical transcript that had been mailed to her.

## Apply for a Job

At some time in the future, Alice would like to work for the fictional company, Acme Corp.
Normally she would browse to their website, where she would click on a hyperlink to apply for a job.
Her browser would download a connection request in which her Indy app would open; this would trigger a prompt to Alice, asking her to accept the connection with Acme Corp.
Because we’re using an VCX, the process is different, but the steps are the same.
The process of the connection establishment is the same as when Faber was accepting the Steward connection request.

After Alice had established connection with Acme, she got the **Job-Application** Proof Request.
A proof request is a request made by the party who needs verifiable proof of having certain attributes and the solving of predicates that can be provided by other verified credentials.

In this case, Acme Corp is requesting that Alice provide a **Job Application**.
The Job Application requires a name, degree, status, SSN and also the satisfaction of the condition about the average mark or grades.

In this case, **Job-Application** Proof Request looks like:

```python
# Acme Agent

proof_attrs = [
    {'name': 'first_name', 'restrictions': []},
    {'name': 'last_name', 'restrictions': []},
    {'name': 'phone_number', 'restrictions': []},
    {'name': 'degree', 'restrictions': [{'cred_def_id': faber_transcript_cred_def_id}]},
    {'name': 'status', 'restrictions': [{'cred_def_id': faber_transcript_cred_def_id}]},
    {'name': 'ssn', 'restrictions': [{'cred_def_id': faber_transcript_cred_def_id}]},
    {'name': 'average', 'p_type': 'GE', 'p_value': 4, 'restrictions': [{'cred_def_id': faber_transcript_cred_def_id}]},
]

# Create a Proof object
proof = await Proof.create('alice_proof','Job-Application', proof_attrs)

# Request proof of degree from alice
await proof.request_proof(connection_to_alice)

# Poll agency and wait for alice to provide proof
while await proof.get_state() != State.Accepted:
    sleep(2)
    await proof.update_state()
```

Notice that some attributes are verifiable and some are not.

The proof request says that SSN, degree, and graduation status in the Credential must be formally asserted by an issuer and schema_key. Notice also that the first_name, last_name and phone_number are not required to be verifiable.
By not tagging these credentials with a verifiable status, Acme’s credential request is saying it will accept Alice’s own credential about her names and phone numbers.

```python
# Alice Agent

# Poll agency for a proof request
requests = await DisclosedProof.get_requests(connection_to_acme)

# Create a Disclosed proof object from proof request
proof = await DisclosedProof.create('proof', requests[0])

# Query for credentials in the wallet that satisfy the proof request
credentials = await proof.get_creds()

# Use the first available credentials to satisfy the proof request
for attr in credentials['attrs']:
    credentials['attrs'][attr] = credentials['attrs'][attr][0]

# Generate the proof
await proof.generate_proof(credentials,{})

# Send the proof to acme
await proof.send_proof(connection_to_acme)
```

**Acme** got all the requested attributes. Now **Acme** wants to check the Validity Proof.
```python
# Acme Agent

# Process the proof provided by alice
await proof.get_proof(connection_to_alice)

# Check if proof is valid
if proof.proof_state == ProofState.Verified:
    print("proof is verified!")
else:
    print("could not verify proof :(")
```

Here, we’ll assume the application is accepted and Alice ends up getting the job.
**Acme** creates new Credential Offer for Alice.
```python
# Acme Agent

# Create an IssuerCredential object using the schema and credential definition
credential = await IssuerCredential.create('alice_job_certificate', schema_attrs, acme_job_certificate_cred_def_id,'cred', 0)

# Issue credential offer to alice
await credential.send_offer(connection_to_alice)
await credential.update_state()
```

When Alice inspects her connection with Acme, she sees that a new Credential Offer is available.

## Apply for a Loan

Now that Alice has a job, she’d like to apply for a loan. That will require a proof of employment.
She can get this from the **Job-Certificate** credential offered by Acme.
Alice goes through a familiar sequence of interactions.

1. First **Acme** creates a Credential Offer.
```python  
# Acme Agent

alice_job_certificate_attrs = {
    'first_name': 'Alice',
    'last_name': 'Garcia',
    'employee_status': 'Permanent',
    'salary': '2400',
    'experience': '10',
}

# Create an IssuerCredential object using the schema and credential definition
alice_job_certificate = await IssuerCredential.create('alice_job_certificate', alice_job_certificate_attrs, acme_job_certificate_cred_def_id, 'cred', 0)

# Issue credential offer to alice
await alice_job_certificate.send_offer(connection_to_alice)
await alice_job_certificate.update_state()
```

2. **Alice** requests the credential.
```python
# Alice Agent

# Get credential offers from Acme
offers = await Credential.get_offers(connection_to_acme)

# Create a credential object from the credential offer
credential = await Credential.create('credential', offers[0])

# Send credential request
await credential.send_request(connection_to_acme, 0)
```

3. Acme waits for Alice to send a credential request. Then issues the credential and sends it.
```python
# Acme Agent
await credential.send_credential(connection_to_alice)
```

Now the **Job-Certificate** Credential has been issued and Alice now has it in her possession. Alice stores **Job-Certificate** Credential in her wallet.
```python
# Alice Agent
while await credential.get_state() != State.Accepted:
    sleep(2)
    await credential.update_state()
```

She can use it when she applies for her loan, in much the same way that she used her transcript when applying for a job.

There is a disadvantage in this approach to data sharing though, — it may disclose more data than what is strictly necessary. If all Alice needs to do is provide proof of employment, this can be done with an anonymous credential instead. Anonymous credentials may prove certain predicates without disclosing actual values (e.g., Alice is employed full-time, with a salary greater than X, along with her hire date, but her actually salary remains hidden). A compound proof can be created, drawing from credentials from both Faber College and Acme Corp, that discloses only what is necessary.

Alice now establishes connection with Thrift Bank.

Alice gets a **Loan-Application-Basic** Proof Request from Thrift Bank that looks like:
```python
# Thrift Agent

proof_attrs = [
    {'name': 'employee_status', 'restrictions': [{'cred_def_id': acme_job_certificate_cred_def_id}]},
    {'name': 'salary', 'p_type': 'GE', 'p_value': 2000, 'restrictions': [{'cred_def_id': acme_job_certificate_cred_def_id}]},
    {'name': 'experience', 'p_type': 'GE', 'p_value': 1, 'restrictions': [{'cred_def_id': acme_job_certificate_cred_def_id}]},
]

# Create a Proof object
proof = await Proof.create('alice_proof','Job-Application', proof_attrs)

# Request proof of degree from alice
await proof.request_proof(connection_to_alice)
```

Alice has only one credential that meets the proof requirements for this **Loan-Application-Basic** Proof Request.
```python
# Alice Agent

# Poll agency for a proof request
requests = await DisclosedProof.get_requests(connection_to_thrift)

# Create a Disclosed proof object from proof request
proof = await DisclosedProof.create('proof', requests[0])

#Query for credentials in the wallet that satisfy the proof request
credentials = await proof.get_creds()

# Use the first available credentials to satisfy the proof request
for attr in credentials['attrs']:
    credentials['attrs'][attr] = credentials['attrs'][attr][0]
```

Alice sends just the **Loan-Application-Basic** proof to the bank.
This allows her to minimize the PII (personally identifiable information) that she has to share when all she's trying to do right now is prove basic eligibility.
```python
# Alice Agent

await proof.generate_proof(credentials, {})

await proof.send_proof(connection_to_thrift)
```

When **Thrift** inspects the received Proof he will see the following structure:
```
# Thrift Agent
{
    'requested_proof': {
        'revealed_attrs': {
            'attr1_referent': {'sub_proof_index': 0, 'raw':'Permanent', 'encoded':'2143135425425143112321314321'},
        },
        'self_attested_attrs': {},
        'unrevealed_attrs': {},
        'predicates': {
            'predicate1_referent': {'sub_proof_index': 0},
            'predicate2_referent': {'sub_proof_index': 0}
        }
    },
    'proof' : { ... } # Validity Proof that Thrift can check
    'identifiers' : [ # Identifiers of credentials were used for Proof building
            {
            'schema_id': job_certificate_schema_id,
                'cred_def_id': acme_job_certificate_cred_def_id,
                'revoc_reg_seq_no': None,
                'timestamp': None
        }
    ]
}
```

**Thrift Bank** successfully verified the **Loan-Application-Basic** Proof from Alice.
```python
# Thrift Agent

# Process the proof provided by alice
await proof.get_proof(connection_to_alice)

# Check if proof is valid
if proof.proof_state == ProofState.Verified:
    print("proof is verified!")
else:
    print("could not verify proof :(")
```

Thrift Bank sends the second Proof Request where Alice needs to share her personal information with the bank.
```python
# Thrift Agent

proof_attrs = [
    {'name': 'first_name', 'restrictions': []},
    {'name': 'last_name', 'restrictions': []},
    {'name': 'ssn', 'restrictions': []},
]

# Create a Proof object
proof = await Proof.create('alice_proof','Job-Application', proof_attrs)

# Request proof of degree from alice
await proof.request_proof(connection_to_alice)

```

Alice has two credentials that meets the proof requirements for this **Loan-Application-KYC** Proof Request.
```python
# Alice Agent
{
  'referent': 'Transcript Credential Referent',
  'schema_id': transcript_schema_id,
  'cred_def_id': faber_transcript_cred_def_id,
  'attrs': {
      'first_name': 'Alice',
      'last_name': 'Garcia',
      'status': 'graduated',
      'degree': 'Bachelor of Science, Marketing',
      'ssn': '123-45-6789',
      'year': '2015',
      'average': '5'
  },
  'rev_reg_id': None,
  'cred_rev_id': None
},
{
    'referent': 'Job-Certificate Credential Referent',
    'schema_key': job_certificate_schema_id,
    'cred_def_id': acme_job_certificate_cred_def_id,
    'attrs': {
        'employee_status': 'Permanent',
        'last_name': 'Garcia',
        'experience': '10',
        'first_name': 'Alice',
        'salary': '2400'
    },
    'rev_reg_id': None,
    'revoc_reg_seq_no': None
}
```

Alice creates the Proof for **Loan-Application-KYC** Proof Request.
```python
# Alice Agent

# Poll agency for a proof request
requests = await DisclosedProof.get_requests(connection_to_thrift)

# Create a Disclosed proof object from proof request
proof = await DisclosedProof.create('proof', requests[0])

#Query for credentials in the wallet that satisfy the proof request
credentials = await proof.get_creds()

# Use the first available credentials to satisfy the proof request
for attr in credentials['attrs']:
    credentials['attrs'][attr] = credentials['attrs'][attr][0]

# Generate the proof
await proof.generate_proof(credentials, {})

# Send the proof
await proof.send_proof(connection_to_thrift)
```

When **Thrift** inspects the received Proof he will see the following structure:
```
  # Thrift Agent
  {
      'requested_proof': {
          'revealed_attrs': {
              'attr1_referent': {'sub_proof_index': 0, 'raw':'123-45-6789', 'encoded':'3124141231422543541'},
              'attr1_referent': {'sub_proof_index': 1, 'raw':'Alice', 'encoded':'245712572474217942457235975012103335'},
              'attr1_referent': {'sub_proof_index': 1, 'raw':'Garcia', 'encoded':'312643218496194691632153761283356127'},
          },
          'self_attested_attrs': {},
          'unrevealed_attrs': {},
          'predicates': {}
      },
      'proof' : { ... } # Validity Proof that Thrift can check
      'identifiers' : [ # Identifiers of credentials were used for Proof building
          {
            'schema_id': transcript_schema_id,
            'cred_def_id': faber_transcript_cred_def_id,
            'rev_reg_id': None,
            'timestamp': None
          },
          {
            'schema_key': job_certificate_schema_id,
            'cred_def_id': acme_job_certificate_cred_def_id,
            'rev_reg_id': None,
            'timestamp': None
          }
      ]
  }
```

**Thrift Bank** has successfully validated the **Loan-Application-KYC** Proof from Alice.
```python
# Thrift Agent

# Process the proof provided by Alice
await proof.get_proof(connection_to_alice)

# Check if proof is valid
if proof.proof_state == ProofState.Verified:
    print("proof is verified!")
else:
    print("could not verify proof :(")
```

Both of Alice's Proofs have been successfully verified and she got loan from **Thrift Bank**.
