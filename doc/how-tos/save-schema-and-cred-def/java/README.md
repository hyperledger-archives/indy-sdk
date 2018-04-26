# Save a Schema and Credential Definition

Indy-SDK Developer Walkthrough #4, Java Edition

[ [Python](../python/README.md) | [.NET](../dotnet/README.md) | [Node.js](../node/README.md) | [Objective C](../objectivec/README.md) ]


## Prerequisites

Setup your workstation with an indy development virtual machine (VM). See [prerequisites](../../prerequisites).


## Steps

### Step 1

In your normal workstation operating system (not the VM), open a java editor of your
choice and paste the code from [template.java](template.java)
into a new doc. We will be modifying this code in later steps.

Save the doc as `SaveSchemaAndCredDef.java`

This is a simple app framework into which you'll plug the code
you'll be writing.

### Step 2

We need to give the SDK some context that it will need
to deal with an indy ledger. This requires us to point the SDK at some
*genesis transactions* that tell the SDK how to contact the ledger on
the network and how to trust that the nodes it contacts possess
appropriate keys. We also need to create a wallet so the SDK can store
DIDs and the key material we're going to use. Also, we need
to create a trust anchor identity that has privileges to create schemas
and credential definitions.

All of these steps are similar to those in simpler how-tos, such as
["Write a DID and Query Its Verkey"](../../write-did-and-query-verkey/java/README.md).
We'll get this housekeeping out of
the way in a single step here, rather than dwelling on its details.

Copy the contents of [step2.java](step2.java) into
`SaveSchemaAndCredDef.java` on top of the `Step 2 code goes here` placeholder comment.

Save the updated version of `SaveSchemaAndCredDef.java`.

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
  "name": "simple_gvt_credential",
  "version": "1.0",
  "attr_names": ["age", "gender", "height", "name"]
}
```

Copy the contents of [step3.java](step3.java) into
`SaveSchemaAndCredDef.java` on top of the `Step 3 code goes here` placeholder comment.

Save the updated version of `SaveSchemaAndCredDef.java`.

Notice how this schema is submitted to the ledger by the steward
identity we created previously.

### Step 4

Next, we create a *credential definition*. This references the schema
that we just added, and announces who is going to be issuing credentials
with that schema (our trust anchor identity, in this case), what type of
signature method they plan to use ("CL" = "Camenisch Lysyanskya", the
default method used for zero-knowledge proofs by indy), how they
plan to handle revocation, and so forth.

Copy the contents of [step4.java](step4.java) into
`SaveSchemaAndCredDef.java` on top of the `Step 4 code goes here` placeholder comment.

Save the updated version of `SaveSchemaAndCredDef.java`.

### Step 5

Run the [finished code](SaveSchemaAndCredDef.java) and observe the whole sequence.

## More experiments

You might try the ["Issue a Credential"](../../issue-cred/java/README.md)
how-to, which can be done in only one step once you complete this one.
