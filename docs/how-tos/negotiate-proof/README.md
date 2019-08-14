# Negotiate Proof

After an issuer has completed the ["Save Schema and Cred Def"](../save-schema-and-cred-def/README.md)
and ["Issue Credential"](../issue-credential/README.md) how-tos, you have
all the context for a credential holder and a relying party (verifier)
to generate a zero-knowledge proof based on the credential.

In case of troubles running the how-to, please read the [trouble shooting](../trouble-shooting.md) section.

## Prerequisites

Setup your workstation with an indy development virtual machine (VM). See [prerequisites](../prerequisites.md).

## Steps

### Step 1

In your normal workstation operating system (not the VM), open a text editor of your
choice and paste the *template* code of one of the available in the list bellow into 
a new file and saved it as `negotiate_proof.EXT`, replacing *EXT* with the proper file 
extension (e.g for python: `negotiate_proof.py`, for nodejs: `negotiate_proof.js`, and so on). 
We will be modifying this code in later steps.

[ [Python template](python/template.py) | [Java template](java/README.md) | [Node.js template](nodejs/template.js) | [Rust template](rust/src/template.rs)]

This is a very simple app framework into which you'll plug the code
you'll be writing.

### Step 2

This how-to builds on the work in ["Issue Credential"](../issue-credential/README.md).
Rather than duplicate our explanation of those steps here, we will simply
copy that code as our starting point.

Copy the contents of the correspondent *step2* file below into your `negotiate_proof` file 
instead of the `Step 2 code goes here` placeholder comment, and save it.

[ [Python step2](python/step2.py) | [Node.js step2](nodejs/step2.js) | [Rust step2](rust/src/step2.rs)]

### Step 3

Proof negotiation typically begins when a *verifier* (also called a *relying party*)
requests proof. (As with credential issuance, the process has three logical
phases, but it is rare to begin with a proof offer. However, if an initial
proof request is met with a counter-offer, the offering phase of the
sequence becomes relevant.)

![3 phases of proof negotiation; first phase is uncommon](3-phases.png)

A proof request is a JSON file that describes what sort of
proof would satisfy the relying party.

Once the proof request is received, a holder of credentials must scan their
*identity wallet* to find out which credentials could be used to satisfy
the request. (Wallet scanning is inefficient, but this does not cause
problems for dozens or hundreds of credentials. At higher scale, a new
mechanism is needed.
[Work is underway](https://docs.google.com/presentation/d/1X6F9QVG8M4PqQQLLL_5I6aQ5z7CCpYyYHBNKYMlsqXc/edit#slide=id.g31e3a419cd_0_67)
to add index-driven search to indy wallets. Visit
[#indy-sdk on Rocket.Chat](https://chat.hyperledger.org/channel/indy-sdk)
to learn more.)

Copy the contents of the correspondent *step3* file below into your `negotiate_proof` 
file instead of the `Step 3 code goes here` placeholder comment.

[ [Python step3](python/step3.py) | [Node.js step3](nodejs/step3.js) | [Rust step3](rust/src/step3.rs)]

### Step 4

At this point, the holder becomes a *prover* by generating and presenting
a proof. This is done by building some JSON that selects the credentials
(out of those identified as valid candidates in the previous step),
that the prover wishes to use to satisfy the request. The prover calls
`prover_create_proof` function with appropriate parameters, and the
proof is created.

Copy the contents of *step4* file below into your `negotiate_proof` file instead of 
the `Step 4 code goes here` placeholder comment and save it.

[ [Python step4](python/step4.py) | [Node.js step4](nodejs/step4.js) | [Rust step4](rust/src/step4.rs)]

### Step 5

Finally, the verifier needs to check to be sure the proof that's presented
satisfies their criteria. This is easy; just call `verifier_verify_proof` function.

Copy the contents of *step5* file below into `negotiate_proof` file instead of 
the `Step 5 code goes here` placeholder comment and save it.

[ [Python step5](python/step5.py) | [Node.js step5](nodejs/step5.js) | [Rust step5](rust/src/step5.rs)]

### Step 6

Run the completed demo and observe the whole sequence.

[ [Python complete](python/negotiate_proof.py) | [Node.js complete](nodejs/negotiateProof.js) | [Rust complete](rust/src/negotiate-proof.rs)]

## More experiments

You might try the ["Send a Secure Message"](../send-secure-msg/README.md) how-to.
