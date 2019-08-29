/**
 * This script is a sample to show revocation process.
 *
 * This idea is to illustrate revocation process, not the basis of indy SDK (for this, you can take a look
 * at other scripts like gettingStarted.js.
 *
 * Scenario :
 * - Issuer create schema, credential definition, revocation registry
 * - Issuer create and send offer to prover
 * - Prover request credential
 * - Issuer issues credential to prover
 * - Verifier create proof request
 * - Prover search credential and create proof, with revocation state
 * - Verifier gets revocation data, and verifies proof is non-revoked (--> true)
 * - Issuer revoke credential and posts delta to ledger
 * - Verifier gets current revocation registry value from ledger, and verifies that proof is revoked (false)
 * - Verifier gets revocation registry value from ledger until proof reception timestamp, and verifies that proof at this timestamp is non-revoked
 *
 * All logs are colored differently for each actor (cyan for issuer, magenta for prover and yellow for verifier).
 * Data checked as expected are logged with green color, otherwise the logs are red.
 *
 * To ensure that there is no data shared with local variables between each actors, all data are stored in
 * separated spaces (issuer = {}, prover = {}, verifier = {}). Likewise, some instructions with local variables
 * are grouped in "blocks" ( {...} ) to avoid involuntary data sharing between actors.
 */

const indy = require('indy-sdk')
const util = require('./util')
const COLOR = require('./colors')

const log = console.log

const ISSUER_COLOR = COLOR.CYAN
const PROVER_COLOR = COLOR.MAGENTA
const VERIFIER_COLOR = COLOR.YELLOW

function logIssuer(s) { log(ISSUER_COLOR+s+COLOR.NONE) }
function logProver(s) { log("\t"+PROVER_COLOR+s+COLOR.NONE) }
function logVerifier(s) { log("\t\t"+VERIFIER_COLOR+s+COLOR.NONE) }
function logOK(s) { log(COLOR.GREEN+s+COLOR.NONE) }
function logKO(s) { log(COLOR.RED+s+COLOR.NONE) }

// Functions for wallet

async function createAndOpenWallet(actor) {
    const walletConfig = {"id": actor + ".wallet"}
    const walletCredentials = {"key": actor + ".wallet_key"}
    await indy.createWallet(walletConfig, walletCredentials)
    return await indy.openWallet(walletConfig, walletCredentials)
}

async function closeAndDeleteWallet(wallet, actor) {
    await indy.closeWallet(wallet)
    const walletConfig = {"id": actor + ".wallet"}
    const walletCredentials = {"key": actor + ".wallet_key"}
    await indy.deleteWallet(walletConfig, walletCredentials)
}

// Functions for pool handler

async function createAndOpenPoolHandle(actor) {
    const poolName = actor + "-pool-sandbox"
    const poolGenesisTxnPath = await util.getPoolGenesisTxnPath(poolName)
    const poolConfig = {"genesis_txn": poolGenesisTxnPath}
    await indy.createPoolLedgerConfig(poolName, poolConfig)
        .catch(e => {
            console.log("ERROR : ", e)
            process.exit()
        })
    return await indy.openPoolLedger(poolName, poolConfig)
}

async function closeAndDeletePoolHandle(poolHandle, actor) {
    await indy.closePoolLedger(poolHandle)
    const poolName = actor + "-pool-sandbox"
    await indy.deletePoolLedgerConfig(poolName)
}

// Misc

async function createAndStoreMyDid(wallet, seed) {
    const [did] = await indy.createAndStoreMyDid(wallet, {'seed': seed})
    return did
}

async function verifierVerifyProof(revRegDefId, revRegDef, timestamp, revRegValue, proofReq, proof, schemas, credDefs) {
    const revRefDefs = {
        [revRegDefId]: revRegDef
    }
    const revRegs = {
        [revRegDefId]: {
            [timestamp]: revRegValue
        }
    }
    return indy.verifierVerifyProof(proofReq, proof, schemas, credDefs, revRefDefs, revRegs)
}

function checkResponse(response) {
    if (!response) {
        throw new Error("ERROR in 'ensurePreviousRequestApplied' : response is undefined !")
    }
    if (response.op === "REJECT") {
        throw new Error("ERROR in 'ensurePreviousRequestApplied' : response.op is "+response.op+" and must be REPLY. Reason : "+response.reason)
    }
    if (response.op !== "REPLY") {
        throw new Error("ERROR in 'ensurePreviousRequestApplied' : response.op is "+response.op+" and must be REPLY")
    }
    if (!response.result) {
        throw new Error("ERROR in 'ensurePreviousRequestApplied' : response.result is undefined ! response=" + JSON.stringify(response))
    }
}

async function ensureSubmitRequest(poolHandle, request) {
    const response = await indy.submitRequest(poolHandle, request)
    checkResponse(response)
    return response
}

async function ensureSignAndSubmitRequest(poolHandle, wallet, did, request) {
    const response = await indy.signAndSubmitRequest(poolHandle, wallet, did, request)
    checkResponse(response)
    return response
}

async function postSchemaToLedger(poolHandle, wallet, did, schema) {
    const schemaRequest = await indy.buildSchemaRequest(did, schema)
    try {
        await ensureSignAndSubmitRequest(poolHandle, wallet, did, schemaRequest)
    } catch (e) {
        // Accept if schema already exists
        if (e.message.indexOf("can have one and only one SCHEMA with name")) {
            return
        }
        throw e
    }
}

async function getSchemaFromLedger(poolHandle, did, schemaId) {
    const getSchemaRequest = await indy.buildGetSchemaRequest(did, schemaId)
    const getSchemaResponse = await ensureSubmitRequest(poolHandle, getSchemaRequest)
    const [, schema] = await indy.parseGetSchemaResponse(getSchemaResponse)
    return schema
}

async function getCredDefFromLedger(poolHandle, did, credDefId) {
    const getCredDefRequest = await indy.buildGetCredDefRequest(did, credDefId)
    const getCredDefResponse = await ensureSubmitRequest(poolHandle, getCredDefRequest)
    const [, credDef] = await indy.parseGetCredDefResponse(getCredDefResponse)
    return credDef
}

async function postCredDefToLedger(poolHandle, wallet, did, credDef) {
    const credDefRequest = await indy.buildCredDefRequest(did, credDef)
    await ensureSignAndSubmitRequest(poolHandle, wallet, did, credDefRequest)
}

// Main "run" function

async function postRevocRegDefRequestToLedger(poolHandle, wallet, did, revRegDef) {
    const revocRegRequest = await indy.buildRevocRegDefRequest(did, revRegDef)
    await ensureSignAndSubmitRequest(poolHandle, wallet, did, revocRegRequest)
}

async function postRevocRegEntryRequestToLedger(poolHandle, wallet, did, revRegDefId, revRegEntry) {
    const revocRegEntryRequest = await indy.buildRevocRegEntryRequest(did, revRegDefId, "CL_ACCUM", revRegEntry)
    await ensureSignAndSubmitRequest(poolHandle, wallet, did, revocRegEntryRequest)
}

async function getRevocRegDefFromLedger(poolHandle, did, revRegDefId) {
    const getRevocRegDefRequest = await indy.buildGetRevocRegDefRequest(did, revRegDefId)
    const getRevocRegDefResponse = await ensureSubmitRequest(poolHandle, getRevocRegDefRequest)
    const [, revRegDef] = await indy.parseGetRevocRegDefResponse(getRevocRegDefResponse)
    return revRegDef
}

async function getRevocRegDeltaFromLedger(poolHandle, did, revRegDefId, from, to) {
    const getRevocRegDeltaRequest = await indy.buildGetRevocRegDeltaRequest(did, revRegDefId, from, to)
    const getRevocRegDeltaResponse = await ensureSubmitRequest(poolHandle, getRevocRegDeltaRequest)
    const [, revRegDelta, timestamp] = await indy.parseGetRevocRegDeltaResponse(getRevocRegDeltaResponse)
    return [revRegDelta, timestamp]
}

async function getRevocRegFromLedger(poolHandle, did, revRegDefId, timestamp_) {
    const getRevocRegRequest = await indy.buildGetRevocRegRequest(did, revRegDefId, timestamp_)
    const getRevocRegResponse = await ensureSubmitRequest(poolHandle, getRevocRegRequest)
    const [, revReg, timestamp] = await indy.parseGetRevocRegResponse(getRevocRegResponse)
    return [revReg, timestamp]
}

async function run() {

    console.log("Anoncreds Revocation scenario sample -> started")

    // Storage for each actor
    const issuer = {}
    const prover = {}
    const verifier = {}

    log("Set protocol version 2 to work with Indy Node 1.4")
    await indy.setProtocolVersion(2)

    log("Actors Open connections to ledger")
    issuer.poolHandle = await createAndOpenPoolHandle("issuer")
    prover.poolHandle = await createAndOpenPoolHandle("prover")
    verifier.poolHandle = await createAndOpenPoolHandle("verifier")

    log("Actors Create Wallets")
    issuer.wallet = await createAndOpenWallet("issuer")
    prover.wallet = await createAndOpenWallet("prover")
    verifier.wallet = await createAndOpenWallet("verifier")

    log("Actors Create DIDs")
    issuer.did = await createAndStoreMyDid(issuer.wallet, "000000000000000000000000Steward1")
    prover.did = await createAndStoreMyDid(prover.wallet, "000000000000000000000000Steward2")
    verifier.did = await createAndStoreMyDid(verifier.wallet)

    logIssuer("Issuer creates credential schema")
    {
        const [schemaId, schema] = await indy.issuerCreateSchema(issuer.did, "gvt", "1.0", `["name", "age", "sex", "height"]`)
        issuer.schemaId = schemaId
        issuer.schema = schema
    }

    logIssuer("Issuer posts schema to ledger")
    await postSchemaToLedger(issuer.poolHandle, issuer.wallet, issuer.did, issuer.schema)

    logIssuer("Issuer gets schema from ledger")
    issuer.schema = await getSchemaFromLedger(issuer.poolHandle, issuer.did, issuer.schemaId)

    logIssuer("Issuer creates credential definition for schema")
    {
        const [credDefId, credDef] = await indy.issuerCreateAndStoreCredentialDef(issuer.wallet, issuer.did,
                                                        issuer.schema, "tag1", "CL", {"support_revocation": true})
        issuer.credDefId = credDefId
        issuer.credDef = credDef
    }

    logIssuer("Issuer posts credential definition")
    await postCredDefToLedger(issuer.poolHandle, issuer.wallet, issuer.did, issuer.credDef)

    logIssuer("Issuer opens tails writer")
    {
        const tailsWriterConfig = {"base_dir": util.getPathToIndyClientHome() + "/tails", "uri_pattern": ""}
        issuer.tailsWriter = await indy.openBlobStorageWriter("default", tailsWriterConfig)
    }

    logIssuer("Issuer creates revocation registry")
    {
        const [revRegDefId, revRegDef, revRegEntry] = await indy.issuerCreateAndStoreRevocReg(issuer.wallet, issuer.did,
                                                    undefined, "tag1", issuer.credDefId,
                                                    {"max_cred_num": 5, "issuance_type": "ISSUANCE_ON_DEMAND"},
                                                    issuer.tailsWriter)
        issuer.revRegDefId = revRegDefId
        issuer.revRegDef = revRegDef
        issuer.revRegEntry = revRegEntry
    }

    logIssuer("Issuer posts revocation registry definition to ledger")
    await postRevocRegDefRequestToLedger(issuer.poolHandle, issuer.wallet, issuer.did, issuer.revRegDef)

    logIssuer("Issuer posts revocation registry entry to ledger")
    await postRevocRegEntryRequestToLedger(issuer.poolHandle, issuer.wallet, issuer.did, issuer.revRegDefId, issuer.revRegEntry)

    log("Issuer shares public data (schema ID, credential definition ID, ...) (via HTTP or other) ...")
    prover.schemaId = issuer.schemaId
    verifier.schemaId = issuer.schemaId
    verifier.credDefId = issuer.credDefId

    logProver("Prover gets schema from ledger")
    prover.schema = await getSchemaFromLedger(prover.poolHandle, prover.did, prover.schemaId)

    logVerifier("Verifier gets schema from ledger")
    verifier.schema = await getSchemaFromLedger(verifier.poolHandle, verifier.did, verifier.schemaId)

    logIssuer("Issuer creates credential offer")
    issuer.credOffer = await indy.issuerCreateCredentialOffer(issuer.wallet, issuer.credDefId)

    log("Transfer offer from 'Issuer' to 'Prover' (via HTTP or other) ...")
    prover.credOffer = issuer.credOffer

    logProver("Prover gets credential definition from ledger")
    prover.credDefId = prover.credOffer["cred_def_id"]
    prover.credDef = await getCredDefFromLedger(prover.poolHandle, prover.did, prover.credDefId)

    logProver("Prover creates master secret")
    prover.masterSecretId = await indy.proverCreateMasterSecret(prover.wallet, undefined)

    logProver("Prover creates credential request")
    {
        const [credReq, credReqMetadata] = await indy.proverCreateCredentialReq(prover.wallet, prover.did,
                                                                prover.credOffer, prover.credDef, prover.masterSecretId)
        prover.credReq = credReq
        prover.credReqMetadata = credReqMetadata
    }

    log("Transfer credential request from 'Prover' to 'Issuer' (via HTTP or other) ...")
    issuer.credReq = prover.credReq

    logIssuer("Issuer open Tails reader")
    {
        const tailsReaderConfig = {"base_dir": util.getPathToIndyClientHome() + "/tails", "uri_pattern": ""}
        issuer.blobStorageReaderHandle = await indy.openBlobStorageReader("default", tailsReaderConfig)
    }

    logIssuer("Issuer creates credential")
    {
        const credValues = {
            "sex": {"raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050"},
            "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
            "height": {"raw": "175", "encoded": "175"},
            "age": {"raw": "28", "encoded": "28"}
        }
        const [cred, credRevId, revRegDelta] = await indy.issuerCreateCredential(issuer.wallet, issuer.credOffer,
                                issuer.credReq, credValues, issuer.revRegDefId, issuer.blobStorageReaderHandle)
        issuer.cred = cred
        issuer.credRevId = credRevId
        issuer.revRegDelta = revRegDelta
    }

    logIssuer("Issuer posts revocation registry delta to ledger (#1)")
    await postRevocRegEntryRequestToLedger(issuer.poolHandle, issuer.wallet, issuer.did, issuer.revRegDefId, issuer.revRegDelta)

    log("Transfer credential from 'Issuer' to 'Prover' (via HTTP or other) ...")
    prover.cred = issuer.cred
    issuer.cred = undefined

    logProver("Prover gets revocation registry definition from ledger")
    prover.revRegDefId = prover.cred["rev_reg_id"]
    prover.revRegDef = await getRevocRegDefFromLedger(prover.poolHandle, prover.did, prover.revRegDefId)

    logProver("Prover stores credential")
    await indy.proverStoreCredential(prover.wallet, undefined, prover.credReqMetadata, prover.cred, prover.credDef, prover.revRegDef)

    logVerifier("Verifier creates proof request")
    const nonce = await indy.generateNonce()
    verifier.proofReq = {
        "nonce": nonce,
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "name",
                "restrictions": {"cred_def_id": verifier.credDefId}
            }
        },
        "requested_predicates": {
            "predicate1_referent": {
                "name": "age",
                "p_type": ">=",
                "p_value": 18,
                "restrictions": {"cred_def_id": verifier.credDefId}
            }
        },
        "non_revoked": {/*"from": 0,*/ "to": util.getCurrentTimeInSeconds()}
    }

    log("Transfer proof request from 'Verifier' to 'Prover' (via HTTP or other) ...")
    prover.proofReq = verifier.proofReq

    logProver("Prover gets credentials for proof request")
    {
        const searchHandle = await indy.proverSearchCredentialsForProofReq(prover.wallet, prover.proofReq, undefined)

        const credentialsForAttr1 = await indy.proverFetchCredentialsForProofReq(searchHandle, "attr1_referent", 10)
        prover.credInfoForAttribute = credentialsForAttr1[0]["cred_info"]

        const credentialsForPredicate1 = await indy.proverFetchCredentialsForProofReq(searchHandle, "predicate1_referent", 10)
        prover.credInfoForPredicate = credentialsForPredicate1[0]["cred_info"]

        await indy.proverCloseCredentialsSearchForProofReq(searchHandle)
    }

    logProver("Prover opens tails reader")
    {
        const tailsReaderConfig = {"base_dir": util.getPathToIndyClientHome() + "/tails", "uri_pattern": ""}
        prover.blobStorageReaderHandle = await indy.openBlobStorageReader("default", tailsReaderConfig)
    }

    logProver("Prover gets revocation registry delta from ledger")
    {
        const [revRegDelta, timestamp] = await getRevocRegDeltaFromLedger(prover.poolHandle, prover.did, prover.revRegDefId, 0 /* from */, util.getCurrentTimeInSeconds() /* to */)
        prover.revRegDelta = revRegDelta
        prover.timestampOfDelta = timestamp // = timestamp of "Issuer Posts Revocation Registry Delta to ledger (#1)"
    }

    logProver("Prover creates revocation state")
    prover.credRevId = prover.credInfoForAttribute["cred_rev_id"]
    prover.revState = await indy.createRevocationState(prover.blobStorageReaderHandle, prover.revRegDef, prover.revRegDelta, prover.timestampOfDelta, prover.credRevId)

    logProver("Prover creates proof for proof request")
    prover.requestedCredentials = {
        "self_attested_attributes": {},
        "requested_attributes": {
            "attr1_referent": {
                "cred_id": prover.credInfoForAttribute["referent"], "revealed": true, "timestamp": prover.timestampOfDelta
            }
        },
        "requested_predicates": {
            "predicate1_referent": {"cred_id": prover.credInfoForPredicate["referent"], "timestamp": prover.timestampOfDelta}
        }
    }
    prover.schemas = {
        [prover.schemaId]: prover.schema
    }
    prover.credDefs = {
        [prover.credDefId]: prover.credDef
    }
    prover.revocStates = {
        [prover.revRegDefId]: {
            [prover.timestampOfDelta]: prover.revState
        }
    }
    prover.proof = await indy.proverCreateProof(prover.wallet, prover.proofReq, prover.requestedCredentials, prover.masterSecretId,
                                        prover.schemas, prover.credDefs, prover.revocStates)

    log("Transfer proof from 'Prover' to 'Verifier' (via HTTP or other) ...")
    verifier.proof = prover.proof
    verifier.timestampOfProof = verifier.proof.identifiers[0].timestamp
    verifier.timestampReceptionOfProof = util.getCurrentTimeInSeconds()

    logVerifier("Verifier gets credential definition from ledger")
    verifier.credDefId = verifier.proof.identifiers[0]["cred_def_id"]
    verifier.credDef = await getCredDefFromLedger(verifier.poolHandle, verifier.did, verifier.credDefId)

    logVerifier("Verifier gets revocation registry definition")
    verifier.revRegDefId = verifier.proof.identifiers[0]["rev_reg_id"]
    verifier.revRegDef = await getRevocRegDefFromLedger(verifier.poolHandle, verifier.did, verifier.revRegDefId)

    logVerifier("Verifier gets revocation registry value from ledger")
    {
        const [revRegValue, /* timestamp (unused) */] = await getRevocRegFromLedger(verifier.poolHandle, verifier.did, verifier.revRegDefId, verifier.timestampOfProof)
        // timestamp = timestamp of "Issuer Posts Revocation Registry Delta to ledger (#1)"
        verifier.revRegValue = revRegValue
    }

    logVerifier("Verifier verify proof (#1)")
    verifier.schemas = {
        [verifier.schemaId]: verifier.schema
    }
    verifier.credDefs = {
        [verifier.credDefId]: verifier.credDef
    }
    const verifiedBeforeRevocation = await verifierVerifyProof(verifier.revRegDefId, verifier.revRegDef, verifier.timestampOfProof, verifier.revRegValue, verifier.proofReq, verifier.proof, verifier.schemas, verifier.credDefs)
    if (verifiedBeforeRevocation) {
        logOK("OK : proof is verified as expected :-)")
    } else {
        logKO("KO : proof is expected to be verified but it is NOT... :-(")
    }

    log("Pause....")
    await util.sleep(3000)

    logIssuer("Issuer revokes credential")
    issuer.revRegDeltaAfterRevocation = await indy.issuerRevokeCredential(issuer.wallet, issuer.blobStorageReaderHandle, issuer.revRegDefId, issuer.credRevId)

    log("Pause....")
    await util.sleep(3000)

    logIssuer("Issuer posts revocation registry delta to ledger (#2 after revocation)")
    await postRevocRegEntryRequestToLedger(issuer.poolHandle, issuer.wallet, issuer.did, issuer.revRegDefId, issuer.revRegDeltaAfterRevocation)

    logVerifier("Verifier gets revocation registry delta from ledger")
    {
        const [revRegValue, /* timestamp (unused) */] = await getRevocRegFromLedger(verifier.poolHandle, verifier.did, verifier.revRegDefId, util.getCurrentTimeInSeconds() /* to */)
        // timestamp = timestamp of "Issuer Posts Revocation Registry Delta to ledger (#2 after revocation)"
        verifier.revRegValue2 = revRegValue
    }

    logVerifier("Verifier verifies proof (#2) (proof must be revoked)")
    const verifiedAfterRevocation = await verifierVerifyProof(verifier.revRegDefId, verifier.revRegDef, verifier.timestampOfProof, verifier.revRegValue2, verifier.proofReq, verifier.proof, verifier.schemas, verifier.credDefs)
    if (!verifiedAfterRevocation) {
        logOK("OK : proof is NOT verified as expected :-)")
    } else {
        logKO("KO : proof is verified but is expected to be NOT... :-(")
    }

    logVerifier("Verifier checks non revoked proof with timestamp of reception of proof (before credential revocation)")

    logVerifier("Verifier gets revocation registry delta from ledger")
    {
        const [revRegValue, /* timestamp (unused) */] = await getRevocRegFromLedger(verifier.poolHandle, verifier.did, verifier.revRegDefId, verifier.timestampReceptionOfProof /* to */)
        verifier.revRegValue3 = revRegValue
    }

    logVerifier("Verifier verifies proof (#3) (proof must be non-revoked)")
    const verifiedBeforeRevocation2 = await verifierVerifyProof(verifier.revRegDefId, verifier.revRegDef, verifier.timestampOfProof, verifier.revRegValue3, verifier.proofReq, verifier.proof, verifier.schemas, verifier.credDefs)
    if (verifiedBeforeRevocation2) {
        logOK("OK : proof is verified as expected :-)")
    } else {
        logKO("KO : proof is NOT verified but is expected to be NOT... :-(")
    }

    log("Actors close and delete wallets")
    await closeAndDeleteWallet(issuer.wallet, "issuer")
    await closeAndDeleteWallet(prover.wallet, "prover")
    await closeAndDeleteWallet(verifier.wallet, "verifier")

    log("Actors close and delete poolHandles")
    await closeAndDeletePoolHandle(issuer.poolHandle, "issuer")
    await closeAndDeletePoolHandle(prover.poolHandle, "prover")
    await closeAndDeletePoolHandle(verifier.poolHandle, "verifier")

    log("Anoncreds Revocation sample -> completed")
}

if (require.main.filename == __filename) {
    run()
}

module.exports = {
    run
}
