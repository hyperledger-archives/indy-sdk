# Issue a Credential
Indy-SDK Developer Walkthrough #4, Java Edition

[ [Python](../python/README.md) | [.NET](../dotnet/README.md) | [Node.js](../node/README.md) | [Objective C](../objectivec/README.md) ]


## Prerequisites

Setup your workstation with an indy development virtual machine(VM). See [prerequisites](../../prerequisites).


## Steps

### Step 1

In your normal workstation operating system (not the VM), open a java editor of your
choice and paste the code from [template.java](template.java)
into a new doc. We will be modifying this code in later steps.

Save the doc as `IssueCredential.java`.

This is a very simple app framework into which you'll plug the code
you'll be writing.

### Step 2

This how-to builds on the work in ["Save Schema and Cred Def"](../save-schema-and-cred-def/java/README.md).
Rather than duplicate our explanation of those steps here, we will simply
copy that code as our starting point.

Copy the contents of [step2.java](step2.java) into
`IssueCredential.java` on top of the `Step 2 code goes here` placeholder comment.

Save the updated version of `IssueCredential.java`.

### Step 3

So far, we have created an identity that can be used to issue credentials.
We need another one, now, that can be used to hold credentials once they're issued.

Copy the contents of [step3.java](step3.java) into
`IssueCredential.java` on top of the `Step 3 code goes here` placeholder comment.

Save the updated version of `IssueCredential.java`.

Notice that this identity creates something called a *link secret* (formerly
called a *master secret*; this older term is now deprecated).
A link secret is a special piece of data that's inserted into
a credential, in blinded form, on issuance; it is used to prove that the
credential in question belongs to a particular holder and not to someone
else. Because Alice's credentials contain her link secret, only she can
use them.

### Step 4

At this point, the issuer and the person who receives the credential
(called the *holder*) engage in an interactive protocol that results
in issuance.

First, the issuer offers a credential. This step is optional; we include
it for completeness. Next, the holder requests the credential, supplying
the blinded link secret that will bind it to them. Finally, the issuer
generates the credential and gives it to the holder.

![3-phase negotiation on issuance](../3-phase-negotiation.png)

These three steps embody a negotiation pattern that is used in many
indy interactions (e.g., in proving). Either party can begin; the other
party acknowledges and accepts--or makes a counter proposal. In the case
of a counter proposal, a new negotiation cycle begins; in the simpler
case, the negotiation is concluded successfully. Negotiation could be used
during credential issuance to negotiate a change to a credential (e.g.,
to correct a typo or to ask an issuer to include or omit a piece of data
that they didn't initially propose); however, we don't cover that
advanced workflow here.

One other note: the sample code in this step uses the word "claim" in
places where you might expect "credential." These used to be synonyms,
but usage has evolved in the [W3C](https://github.com/TechWritingWhiz/indy-sdk/blob/master/doc/how-tos/issue-credential/dotnet/README.md) since the Indy SDK was built. "Credential"
is the newer word, and function and parameter names that refer to "claims"
are now deprecated. Eventually, all usage will show "credential."

Copy the contents of [step4.java](step4.java) into
`IssueCredential.java` on top of the `Step 4 code goes here` placeholder comment.

Save the updated version of `IssueCredential.java`.

### Step 5

Run the [finished code](IssueCredential.java) and observe the whole sequence.

## More experiments

You might try the ["Negotiate a Proof"](../../negotiate-proof/java/README.md)
how-to, which can be done in only one step once you complete this one.
