/**
 * Example demonstrating Proof Verification.
 *
 * First Issuer creates Credential Definition for existing Schema.
 * After that, it issues a Credential to Prover (as in issue_credential.py example)
 *
 * Once Prover has successfully stored its Credential, it uses Proof Request that he
 * received, to get Credentials which satisfy the Proof Request from his wallet.
 * Prover uses the output to create Proof, using its Master Secret.
 * After that, Proof is verified against the Proof Request
 */

const indy = require('indy-sdk')
const util = require('./util')
const colors = require('./colors')

const log = console.log

function logValue() {
    log(colors.CYAN, ...arguments, colors.NONE)
}


async function run() {

    const issuerDid = "NcYxiDXkpYi6ov5FcYDi1e"
    const proverDid = "VsKV7grR1BUE29mG2Fm2kX"

    log("Set protocol version 2 to work with Indy Node 1.4")
    await indy.setProtocolVersion(2)

    // Step 2 code goes here.

    // Step 3 code goes here.

    // Step 4 code goes here.

    // Step 5 code goes here.
}

try {
    run()
} catch (e) {
    log("ERROR occured : e")
}
