# Write a DID and Query Its Verkey

This is the most basic operation in indy; it establishes a new DID and
then proves that information about it can be queried from the ledger.
A good step after this how-to would be ["Rotate Key"](../rotate-key/README.md).

In case of troubles running the how-to, please read the [trouble shooting](../trouble-shooting.md) section.

## Prerequisites

Setup your workstation with an indy development virtual machine (VM). See [prerequisites](../prerequisites.md).

## Steps

### Step 1

In your normal workstation operating system (not the VM), open a text editor of your
choice and paste the *template* code of one of the available in the list bellow into 
a new file and saved it as `write_did.EXT`, replacing *EXT* with the proper file 
extension (e.g for python: `write_did.py`, for nodejs: `write_did.js`, and so on). 
We will be modifying this code in later steps.

[ [Python template](python/template.py) | [Java template](java/template.java) | [.NET template](cs/Template.cs) | [Node.js template](nodejs/template.js) | [Rust template](rust/src/template.rs)]

This is a very simple app framework into which you'll plug the code you'll be writing.

### Step 2

Now we need to give the SDK some context that it will need
to deal with an indy ledger. This requires us to point the SDK at some
*genesis transactions* that tell the SDK how to contact the ledger on
the network, and how to trust that the nodes it contacts possess
appropriate keys.

We also need to create an *[identity wallet](https://docs.google.com/presentation/d/1X6F9QVG8M4PqQQLLL_5I6aQ5z7CCpYyYHBNKYMlsqXc/edit#slide=id.g32295399e3_0_73)*, so the SDK can store the DID and key
material generated during the tutorial.

![more info on wallets](wallet-slide.png)

Copy the contents of the correspondent *step2* file below into your `write_did` file 
instead of the `Step 2 code goes here` placeholder comment, and save it.

[ [Python step2](python/step2.py) | [Java step2](java/step2.java) | [.NET step2](../not-yet-written.md) | [Node.js step2](nodejs/step2.js) | [Rust step2](rust/src/step2.rs)]

Study the changes. Scaffolding code like this is likely to appear in anything
that uses indy.

### Step 3

Now we need to put some DIDs and keys in our identity wallet. Copy the contents of 
the correspondent *step3* file below into your `write_did` file instead of the `Step 3 code goes here` placeholder comment.

[ [Python step3](python/step3.py) | [Java step3](java/step3.java) | [.NET step3](../not-yet-written.md) | [Node.js step3](nodejs/step3.js) | [Rust step3](rust/src/step3.rs)]

Study the changes.

A few operations in indy [can only be done by identities (DIDs) with
special roles](https://github.com/hyperledger/indy-node/blob/master/docs/source/auth_rules.md). For example, a DID that is a *steward* can add a node (the one
they own) to the validator pool, and can create DIDs with a *trust anchor*
role. A trust anchor DID can add arbitrary DIDs to the ledger.

Here, we are populating our identity wallet with DID+keypair material for
one steward identity and one trust anchor identity. The steward identity is
a bootstrapping step, whereas the trust anchor DID is the one we will query
later.

Notice that the steward DID is created with a seed, but the trust anchor DID is not.
This is because the steward DID and its verkey is already in the ledger;
they were part of the genesis transactions we told the SDK to start with
in the previous step. But we have to also put the DID, verkey, and *private*
signing key (which the ledger doesn't know) into our wallet, so we can use
the signing key to submit an acceptably signed transaction to the ledger.
We will use this steward's signing key to create our *next* DID--the
one for our trust anchor, which is truly new. This is why we use a hard-coded seed
when creating the steward DID--it guarantees that the same DID and key
material are created that the genesis transactions expect. In a production indy pool
such as the Sovrin "live" network, the bootstrapping steward identities
would not have known the seeds.

### Step 4

Now that preparations are complete, we can finally write the DID and verkey
for our trust anchor identity to the ledger.

Copy the contents of *step4* file below into your `write_did` file instead of 
the `Step 4 code goes here` placeholder comment.

[ [Python step4](python/step4.py) | [Java step4](java/step4.java) | [.NET step4](../not-yet-written.md) | [Node.js step4](nodejs/step4.js) | [Rust step4](rust/src/step4.rs)]

### Step 5

Once we have an identity on the ledger, we can query it.

Copy the contents of *step5* file below into `write_did` file instead of 
the `Step 5 code goes here` placeholder comment.

[ [Python step5](python/step5.py) | [Java step5](java/step5.java) | [.NET step5](../not-yet-written.md) | [Node.js step5](nodejs/step5.js) | [Rust step5](rust/src/step5.rs)]

Only a handful of lines of code matter to our goal here; the rest of
this block is comments and boilerplate cleanup **(which you should not omit!)**.
You should see similarities between the way this query "transaction" and
the preceding write transaction are bundled, sent, and awaited.

### Step 6

Run the completed demo and observe the whole sequence.

[ [Python complete](python/write_did_and_query_verkey.py) | [Java complete](java/WriteDIDAndQueryVerkey.java) | [.NET complete](cs/WriteDIDAndQueryVerkey.cs) | [Node.js complete](nodejs/writeDidAndQueryVerkey.js) | [Rust complete](rust/src/write-did-and-query-verkey.rs)]

## More experiments

Most of the code in this how-to exists to satisfy some preconditions.
Now that you have a trust anchor identity, you can write or query
any number of additional identities to the ledger, with just a handful of
lines of code. Try creating some.

You might try the ["Rotate a Key"](../rotate-key/README.md)
how-to, which can be done in only one step one you complete this one.

You could also try to create a new steward identity without a seed, or
with a different seed, and see what kind of error you get. Only identities
with a trustee role can create stewards.
