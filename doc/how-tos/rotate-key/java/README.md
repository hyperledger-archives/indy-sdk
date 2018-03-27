# Rotate a Key
Indy-SDK Developer Walkthrough #2, Java Edition

[ [Python](../python/README.md) | [.NET](../dotnet/README.md) | [Node.js](../node/README.md) | [Objective C](../objectivec/README.md) ]


## Prerequisites

Before we can rotate a key, we first need to write a DID and its
verkey to the ledger. This means that the initial steps in this how-to are
identical to the ones in ["Write a DID and Query Its Verkey"](
../../write-did-and-query-verkey/java/README.md). If you've just 
done that how-to, you can skip ahead to step 5.

If not, setup your workstation and indy development virtual machine. See [prerequisites](../prerequisites.md).

## Steps

### Step 1

In your normal workstation OS (not the VM), open a java editor of your
choice and paste the code from [template.java](template.java)
into a new doc. We will be modifying this code in later steps.

Save the doc as `RotateKey.java`.

### Step 2

We need to give the SDK some context that it will use
to deal with an indy ledger. This requires us to point the SDK at some
*genesis transactions* that tell the SDK how to contact the ledger on
the network, and how to trust that the nodes it contacts possess 
appropriate keys.

We also need to create a wallet, so the SDK can store the DID and key
material generated during the tutorial.

Copy the contents of [step2.java](step2.java) into
`RotateKey.java` on top of the `Step 2 code goes here` placeholder comment.

Save the updated version of `RotateKey.java`.

### Step 3

Now we need to put some DIDs and keys in our identity
wallet. Copy the contents of [step4.java](step4.java) into
`RotateKey.java` on top of the `Step 4 code goes here` placeholder comment.

Study the changes.

A few (very few) operations in indy can only be done by identities (DIDs) with
special roles. For example, an DID that is a *steward* can add a node (the one
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
material are created that the genesis txns expect. In a production indy pool
such as the Sovrin "live" network, the bootstrapping steward identities
would not have known seeds.

### Step 4

Now that preparations are complete, we can finally write the DID and verkey
for our trust anchor identity to the ledger.

Copy the contents of [step4.java](step4.java) into
`RotateKey.java` on top of the `Step 4 code goes here` placeholder comment.

### Step 5

Once we have an identity on the ledger, we can rotate its key pair.

Copy the contents of [step5.java](step5.java) into
`RotateKey.java` on top of the `Step 5 code goes here` placeholder comment.

Most of the logic here should be self-explanatory. However, it's worth
explaining the paired functions `replaceKeysStart()` and `replaceKeysApply()`.
When we submit the update transaction to the ledger, we have to sign it
using the current signing key; the ledger will verify this using the
verkey that it recognizes. Yet we have to specify the new verkey value
in the transaction itself. The `replaceKeysStart()` method tells the wallet
that an update is pending, and that it should track both the new and old keypairs
for the identity. The `replaceKeysApply()` resolves the pending status
so the new value becomes canonical in the local wallet (after it has
already become canonical on the ledger).

### Step 6

Now we can query the ledger to see which verkey it has on record for the
identity.

Only a handful of lines of code matter here; the rest of this block is
comments and cleanup. You should see similarities between the way this
query "transaction" and the preceding write transaction are bundled, sent,
and awaited.

### Step 7

Run the completed demo and observe the whole sequence.

## More experiments

TBD