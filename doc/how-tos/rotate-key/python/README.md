# Rotate a Key

Indy-SDK Developer Walkthrough #2, Python Edition

[ [Java](../java/README.md) | [.NET](../../not-yet-written.md) | [Node.js](../../not-yet-written.md) | [Objective C](../../not-yet-written.md) ]


## Prerequisites

Setup your workstation with an indy development virtual machine (VM). See [prerequisites](../../prerequisites.md).

## Steps

### Step 1

In your normal workstation operating system (not the VM), open a python editor of your
choice and paste the code from [template.py](template.py)
into a new doc. We will be modifying this code in later steps.

Save the doc as `rotate_key.py`.

### Step 2

This how-to builds on the work in
["Write DID and Query Verkey"](../write-did-and-query-verkey/python/README.md).
Rather than duplicate our explanation of those steps here, we will simply
copy that code as our starting point.

Copy the contents of [step2.py](step2.py) into
`rotate_key.py` on top of the `Step 2 code goes here` placeholder comment.

Save the updated version of `rotate_key.py`.

### Step 3

Once we have an identity on the ledger, we can rotate its key pair.

Copy the contents of [step3.py](step3.py) into
`rotate_key.py` on top of the `Step 3 code goes here` placeholder comment.

Save the updated version of `rotate_key.py`.

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

Copy the contents of [step4.py](step4.py) into
`rotate_key.py` on top of the `Step 4 code goes here` placeholder comment.

Save the updated version of `rotate_key.py`.

Only a handful of lines of code matter to our goal here; the rest of this
block is comments and boilerplate cleanup **(which you should not omit!)**.

### Step 5

Run the completed demo and observe the whole sequence.

## More experiments

TBD
