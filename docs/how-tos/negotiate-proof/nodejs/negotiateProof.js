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
    // 1.
    log("1. Creates Issuer wallet and opens it to get handle.")
    const issuerWalletName = {"id": "issuer_wallet"}
    const issuerWalletCredentials = {"key": "issuer_wallet_key"}
    await indy.createWallet(issuerWalletName, issuerWalletCredentials)
    const issuerWalletHandle = await indy.openWallet(issuerWalletName, issuerWalletCredentials)

    // 2.
    log("2. Creates Prover wallet and opens it to get handle.")
    const proverWalletName = {"id": "prover_wallet"}
    const proverWalletCredentials = {"key": "prover_wallet_key"}
    await indy.createWallet(proverWalletName, proverWalletCredentials)
    const proverWalletHandle = await indy.openWallet(proverWalletName, proverWalletCredentials)

    // 3.
    log("3. Issuer creates credential definition for schema")
    const schemaId = "1"
    const schema = {
        "id": schemaId,
        "ver": "1.0",
        "name": "gvt",
        "version": "1.0",
        "attrNames": ["age", "sex", "height", "name"]
    }
    const [credDefId, credDef] = await indy.issuerCreateAndStoreCredentialDef(issuerWalletHandle, issuerDid, schema, "tag1", "CL", '{"support_revocation": false}')

    // 4.
    log("4. Prover creates Link Secret")
    const proverMasterSecret = await indy.proverCreateMasterSecret(proverWalletHandle, "link_secret")

    // 5.
    log("5. Issuer create Cred Offer")
    const credOffer = await indy.issuerCreateCredentialOffer(issuerWalletHandle, credDefId)

    // 6.
    log("6. Prover creates and stores Cred Request")
    const [credReq, credReqMetadata] = await indy.proverCreateCredentialReq(proverWalletHandle, proverDid, credOffer,
                                                credDef, proverMasterSecret)

    // 7.
    log("7. Issuer creates Credential for received Cred Request")
    const credValues = {
        "sex": {"raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050011144233115103"},
        "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
        "height": {"raw": "175", "encoded": "175"},
        "age": {"raw": "28", "encoded": "28"}
    }
    const tailsWriterConfig = {'base_dir': util.getPathToIndyClientHome() + "/tails", 'uri_pattern': ''}
    const blobStorageReaderHandle = await indy.openBlobStorageReader('default', tailsWriterConfig)
    const [cred] = await indy.issuerCreateCredential(issuerWalletHandle, credOffer, credReq, credValues, undefined, blobStorageReaderHandle)

    // 8.
    log("8. Prover processes and stores received Credential")
    await indy.proverStoreCredential(proverWalletHandle, undefined, credReqMetadata, cred, credDef,undefined)

    // Step 3 code goes here.
    log("9. Prover gets Credentials for Proof Request")
    const proofRequest = {
        'nonce': '123432421212',
        'name': 'proof_req_1',
        'version': '0.1',
        'requested_attributes': {
            'attr1_referent': {
                'name': 'name',
                'restrictions': [{
                    'cred_def_id': credDefId
                    /*
                    'issuer_did': issuerDid,
                    'schema_key': schemaKey
                    */
                }]
            }
        },
        'requested_predicates': {
            'predicate1_referent': {
                'name': 'age',
                'p_type': '>=',
                'p_value': 18,
                'restrictions': [{'issuer_did': issuerDid}]
            }
        }
    }
    const credsForProofRequest = await indy.proverGetCredentialsForProofReq(proverWalletHandle, proofRequest)

    // Step 4 code goes here.
    log("10. Prover creates Proof for Proof Request")
    const credForAttr1 = credsForProofRequest["attrs"]["attr1_referent"]
    const referent = credForAttr1[0].cred_info.referent
    const requestedCredentials = {
        "self_attested_attributes": {},
        "requested_attributes": {
            "attr1_referent": {
                cred_id: referent,
                revealed: true
            }
        },
        "requested_predicates": {
            "predicate1_referent": {
                cred_id: referent
            }
        }
    }
    const schemas = {
        [schemaId]: schema
    }
    const credentialDefs = {
        [credDefId]: credDef
    }
    const revocRegs = {}
    const revRegs = {}
    const proof = await indy.proverCreateProof(proverWalletHandle, proofRequest, requestedCredentials, proverMasterSecret, schemas, credentialDefs, revocRegs)

    // Step 5 code goes here.
    log("11. Verifier is verifying proof from Prover")
    const verified = await indy.verifierVerifyProof(proofRequest, proof, schemas, credentialDefs, revocRegs, revRegs)

    logValue("Proof :")
    logValue(". Name="+proof['requested_proof']['revealed_attrs']['attr1_referent']['raw'])
    logValue(". Verified="+verified)

    // 12
    log("12. Closing both wallet_handles")
    await indy.closeWallet(issuerWalletHandle)
    await indy.closeWallet(proverWalletHandle)

    // 13
    log("13. Deleting created wallet_handles")
    await indy.deleteWallet(proverWalletName, proverWalletCredentials)
    await indy.deleteWallet(issuerWalletName, issuerWalletCredentials)
}

try {
    run()
} catch (e) {
    log("ERROR occured : e")
}
