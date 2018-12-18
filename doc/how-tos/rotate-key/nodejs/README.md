# Rotate a Key

Indy-SDK Developer Walkthrough #2, NodeJS Edition

[ [Python](../python/README.md) | [Java](../java/README.md) | [.NET](../../not-yet-written.md) | [Objective C](../../not-yet-written.md) | [Rust](../rust/README.md)]


## Prerequisites

Setup your workstation with an indy development virtual machine (VM). See [prerequisites](../../prerequisites.md).

Install all dependencies running `npm install`.

## Steps

### Step 1

In your normal workstation operating system (not the VM), open a python editor of your
choice and paste the code from [template.js](template.js)
into a new file. We will be modifying this code in later steps.

Save the file as `rotateKeyOnTheLedger.js`.

### Step 2

This how-to builds on the work in
["Write DID and Query Verkey"](../../write-did-and-query-verkey/nodejs/README.md).
Rather than duplicate our explanation of those steps here, we will simply
copy that code as our starting point.

Copy the contents of [step2.js](step2.js) into
`rotateKeyOnTheLedger.js` instead of the `Step 2 code goes here` placeholder comment.

Save the updated version of `rotateKeyOnTheLedger.js`.

### Step 3

Once we have an identity on the ledger, we can rotate its key pair.

Copy the contents of [step3.js](step3.js) into
`rotateKeyOnTheLedger.js` instead of the `Step 3 code goes here` placeholder comment.

Save the updated version of `rotateKeyOnTheLedger.js`.

Most of the logic here should be self-explanatory. However, it's worth
explaining the paired functions `replace_keys_start` and `replace_keys_apply`.
When we submit the update transaction to the ledger, we have to sign it
using the current signing key; the ledger will verify this using the
verkey that it recognizes. Yet we have to specify the new verkey value
in the transaction itself. The `replace_keys_start` method tells the wallet
that an update is pending, and that it should track both the new and old keypairs
for the identity. The `replace_keys_apply` resolves the pending status
so the new value becomes canonical in the local wallet (after it has
already become canonical on the ledger).

### Step 4

Now we can query the ledger to see which verkey it has on record for the
identity.

Copy the contents of [step4.js](step4.js) into
`rotateKeyOnTheLedger.js` instead of the `Step 4 code goes here` placeholder comment.

Save the updated version of `rotateKeyOnTheLedger.js`.

Only a handful of lines of code matter to our goal here; the rest of this
block is comments and boilerplate cleanup **(which you should not omit!)**.

### Step 5

Run the completed demo and observe the whole sequence.

## More experiments

You might try the ["Save a Schema and Cred Def"](../../save-schema-and-cred-def/nodejs/README.md)
how-to.
