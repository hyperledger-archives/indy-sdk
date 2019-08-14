# Save Schema and Credential Definition

This shows how to save a schema and credential definition on the ledger, which is
a prerequisite for ["Issue Credential"](../issue-credential/README.md).

In case of troubles running the how-to, please read the [trouble shooting](../trouble-shooting.md) section.

## Prerequisites

Setup your workstation with an indy development virtual machine (VM). See [prerequisites](../prerequisites.md).

## Steps

### Step 1

In your normal workstation operating system (not the VM), open a text editor of your
choice and paste the *template* code of one of the available in the list bellow into 
a new file and saved it as `save_schema_and_cred_def.EXT`, replacing *EXT* with the proper file 
extension (e.g for python: `save_schema_and_cred_def.py`, for nodejs: `save_schema_and_cred_def.js`, and so on). 
We will be modifying this code in later steps.

[ [Python template](python/template.py) | [Java template](java/template.java) | [Rust template](rust/src/template.rs)]

This is a very simple app framework into which you'll plug the code you'll be writing.

### Step 2

Now we need to give the SDK some context that it will need
to deal with an indy ledger. This requires us to point the SDK at some
*genesis transactions* that tell the SDK how to contact the ledger on
the network, and how to trust that the nodes it contacts possess
appropriate keys.
We also need to create a wallet so the SDK can store
DIDs and the key material we're going to use. Also, we need
to create a trust anchor identity that has privileges to create schemas
and credential definitions.

All of these steps are similar to those in simpler how-tos, such as
["Write a DID and Query Its Verkey"](../write-did-and-query-verkey/README.md).
We'll get this housekeeping out of the way in a single step here, 
rather than dwelling on its details.

Copy the contents of the correspondent *step2* file below into your `save_schema_and_cred_def` file instead of the `Step 2 code goes here` 
placeholder comment, and save it.

[ [Python step2](python/step2.py) | [Java step2](java/step2.java) | [Rust step2](rust/src/step2.rs)]

Study the changes. Scaffolding code like this is likely to appear in anything
that uses indy.

### Step 3

Now we need to create and define a schema. Schemas in indy are very simple
JSON documents that specify their name and version, and that list attributes
that will appear in a credential. Today, they do not describe data type,
recurrence rules, nesting, and other elaborate constructs. There is work
underway to make them fancier; visit
[#indy-sdk on Rocket.Chat](https://chat.hyperledger.org/channel/indy-sdk) to learn
more.

A sample schema might look like this:

```json
{
  "id": "1",
  "name": "gvt",
  "version": "1.0",
  "ver": "1.0",
  "attrNames": ["age", "sex", "height", "name"]
}
```

Copy the contents of the correspondent *step3* file below into your `save_schema_and_cred_def` file instead of the `Step 3 code goes here` placeholder comment.

[ [Python step3](python/step3.py) | [Java step3](java/step3.java) | [Rust step3](rust/src/step3.rs)]

Notice how this schema is submitted to the ledger by the steward
identity we created previously.

### Step 4

Next, we create a *credential definition*. This references the schema
that we just added, and announces who is going to be issuing credentials
with that schema (our trust anchor identity, in this case), what type of
signature method they plan to use ("CL" = "Camenisch Lysyanskya", the
default method used for zero-knowledge proofs by indy), how they
plan to handle revocation, and so forth.

Copy the contents of *step4* file below into your `save_schema_and_cred_def` file instead of the `Step 4 code goes here` placeholder comment.

[ [Python step4](python/step4.py) | [Java step4](java/step4.java) | [Rust step4](rust/src/step4.rs)]

### Step 5

Run the completed demo and observe the whole sequence.

[ [Python complete](python/save_schema_and_cred_def.py) | [Java complete](java/SaveSchemaAndCredDef.java) | [Rust complete](rust/src/save-schema-and-cred-def.rs)]

## More experiments

You might try the ["Issue a Credential"](../issue-credential/README.md)
how-to, which can be done in only one step once you complete this one.
