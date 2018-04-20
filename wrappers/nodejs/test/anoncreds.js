var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var path = require('path')
var initTestPool = require('./helpers/initTestPool')
var indyHomeDir = require('home-dir')('.indy_client')

test('anoncreds', async function (t) {
  // 1. Create My Wallet and Get Wallet Handle
  var pool = await initTestPool()
  var wName = 'wallet-' + cuid()
  await indy.createWallet(pool.name, wName, 'default', null, null)
  var walletH = await indy.openWallet(wName, null, null)
  var issuerDid = 'NcYxiDXkpYi6ov5FcYDi1e'
  var proverDid = 'VsKV7grR1BUE29mG2Fm2kX'

  // 2. Issuer create credential Definition for Schema
  var [schemaId, schema] = await indy.issuerCreateSchema(issuerDid, 'gvt', '1.0', ['age', 'height', 'name'])
  schema = JSON.parse(schema)

  var [credDefId, credDef] = await indy.issuerCreateAndStoreCredentialDef(walletH, issuerDid, schema, 'tag1', 'CL', {
    support_revocation: true
  })
  credDef = JSON.parse(credDef)

  // 3. Issuer create Revocation Registry
  var tailsWriterConfig = {
    'base_dir': path.join(indyHomeDir, 'tails'),
    'uri_pattern': ''
  }
  var tailsWriterHandle = await indy.openBlobStorageWriter('default', tailsWriterConfig)
  var [revocRegId, revocRegDef, revocRegEntry] = await indy.issuerCreateAndStoreRevocReg(walletH, issuerDid, null, 'tag1', credDefId, {
    max_cred_num: 5
  }, tailsWriterHandle)
  revocRegDef = JSON.parse(revocRegDef)
  revocRegEntry = JSON.parse(revocRegEntry)

  t.is(typeof revocRegEntry, 'object')

  // 4. Prover create Master Secret
  var masterSecretName = 'master_secret'
  await indy.proverCreateMasterSecret(walletH, masterSecretName)

  // 5. Issuer create credential Offer
  var credOffer = await indy.issuerCreateCredentialOffer(walletH, credDefId)

  // 6. Prover create credential Request
  var [credReqJson, credReqMetadataJson] = await indy.proverCreateCredentialReq(walletH, proverDid, credOffer, credDef, 'master_secret')

  // 7. Issuer open Tails reader
  var blobReaderHandle = await indy.openBlobStorageReader('default', tailsWriterConfig)

  // 8. Issuer create credential for credential Request
  var [credJson, revId, revRegDelta] = await indy.issuerCreateCredential(walletH, credOffer, credReqJson, {
    name: {'raw': 'Alex', 'encoded': '1139481716457488690172217916278103335'},
    height: {'raw': '175', 'encoded': '175'},
    age: {'raw': '28', 'encoded': '28'}
  }, revocRegId, blobReaderHandle)
  revRegDelta = JSON.parse(revRegDelta)

  // 9. Prover process and store credential
  var outCredId = await indy.proverStoreCredential(walletH, 'cred_1_id', credReqMetadataJson, credJson, credDef, revocRegDef)
  t.is(typeof outCredId, 'string')

  // 10. Prover gets credentials for Proof Request
  var proofReq = {
    'nonce': '123432421212',
    'name': 'proof_req_1',
    'version': '0.1',
    'requested_attributes': {
      'attr1_referent': {'name': 'name'}
    },
    'requested_predicates': {
      'predicate1_referent': {'name': 'age', 'p_type': '>=', 'p_value': 18}
    },
    'non_revoked': {'from': 80, 'to': 100}
  }
  var credentialsForProof = await indy.proverGetCredentialsForProofReq(walletH, proofReq)

  // 11. Prover gets credentials for Proof Request
  var timestamp = 100
  var revState = await indy.createRevocationState(blobReaderHandle, revocRegDef, revRegDelta, timestamp, revId)

  // 12. Prover create Proof for Proof Request
  var referent = credentialsForProof['attrs']['attr1_referent'][0]['cred_info']['referent']
  var requestedCredentials = {
    'self_attested_attributes': {},
    'requested_attributes': {'attr1_referent': {'cred_id': referent, 'revealed': true, 'timestamp': timestamp}},
    'requested_predicates': {'predicate1_referent': {'cred_id': referent, 'timestamp': timestamp}}
  }

  var schemas = {}
  schemas[schemaId] = schema

  var credentialDefs = {}
  credentialDefs[credDefId] = credDef

  var revocStates = {}
  revocStates[revocRegId] = {}
  revocStates[revocRegId][timestamp] = revState

  var proof = await indy.proverCreateProof(walletH, proofReq, requestedCredentials, masterSecretName, schemas, credentialDefs, revocStates)

  // 13. Verifier verify proof
  t.is(proof['requested_proof']['revealed_attrs']['attr1_referent']['raw'], 'Alex')

  var revocRefDefs = {}
  revocRefDefs[revocRegId] = revocRegDef

  var revocRegs = {}
  revocRegs[revocRegId] = {}
  revocRegs[revocRegId][timestamp] = revRegDelta

  var isValid = await indy.verifierVerifyProof(proofReq, proof, schemas, credentialDefs, revocRefDefs, revocRegs)
  t.is(isValid, true)

  await indy.closeWallet(walletH)
  await indy.deleteWallet(wName, null)
  pool.cleanup()
})
