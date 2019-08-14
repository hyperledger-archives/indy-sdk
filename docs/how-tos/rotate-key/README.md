# Rotate a Key

This shows how to change the verkey that the ledger associates with
a DID. It builds on ["Write DID and Query Its Verkey"](../write-did-and-query-verkey/README.md).

In case of troubles running the how-to, please read the [trouble shooting](../trouble-shooting.md) section.

## Prerequisites

Setup your workstation with an indy development virtual machine (VM). See [prerequisites](../prerequisites.md).

## Steps

### Step 1

In your normal workstation operating system (not the VM), open a text editor of your
choice and paste the *template* code of one of the available in the list bellow into 
a new file and saved it as `rotate_key.EXT`, replacing *EXT* with the proper file 
extension (e.g for python: `rotate_key.py`, for nodejs: `rotate_key.js`, and so on). 
We will be modifying this code in later steps.

[ [Python template](python/template.py) | [Java template](java/template.java) | [Node.js template](nodejs/template.js) | [Rust template](rust/src/template.rs)]

### Step 2

This how-to builds on the work in
["Write DID and Query Verkey"](../write-did-and-query-verkey/README.md).
Rather than duplicate our explanation of those steps here, we will simply
copy that code as our starting point.

Copy the contents of the correspondent *step2* file below into your `rotate_key` file 
instead of the `Step 2 code goes here` placeholder comment, and save it.

[ [Python step2](python/step2.py) | [Java step2](java/step2.java) | [Node.js step2](nodejs/step2.js) | [Rust step2](rust/src/step2.rs)]

### Step 3

Once we have an identity on the ledger, we can rotate its key pair.

Copy the contents of the correspondent *step3* file below into your `rotate_key` file instead of the `Step 3 code goes here` placeholder comment.

[ [Python step3](python/step3.py) | [Java step3](java/step3.java) | [Node.js step3](nodejs/step3.js) | [Rust step3](rust/src/step3.rs)]

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

Copy the contents of *step4* file below into your `rotate_key` file instead of 
the `Step 4 code goes here` placeholder comment.

[ [Python step4](python/step4.py) | [Java step4](java/step4.java) | [Node.js step4](nodejs/step4.js) | [Rust step4](rust/src/step4.rs)]

Only a handful of lines of code matter to our goal here; the rest of this
block is comments and boilerplate cleanup **(which you should not omit!)**.

### Step 5

Run the completed demo and observe the whole sequence.

[ [Python complete](python/rotate_key.py) | [Java complete](java/RotateKey.java) | [Node.js complete](nodejs/rotateKey.js) | [Rust complete](rust/src/rotate-key.rs)]

## More experiments

You might try the ["Save a Schema and Cred Def"](../save-schema-and-cred-def/README.md)
how-to.
