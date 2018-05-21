var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var path = require('path')
var initTestPool = require('./helpers/initTestPool')
var indyHomeDir = require('home-dir')('.indy_client')

test('anoncreds', async function (t) {
  var pool = await initTestPool()
  var wName = 'wallet-' + cuid()
  await indy.createWallet(pool.name, wName, 'default', null, null)
  var wh = await indy.openWallet(wName, null, null)
  var issuerDid = 'NcYxiDXkpYi6ov5FcYDi1e'
  var proverDid = 'VsKV7grR1BUE29mG2Fm2kX'

  // Issuer create a credential schema
  var [schemaId, schema] = await indy.issuerCreateSchema(issuerDid, 'gvt', '1.0', ['age', 'height', 'name'])
  t.not(typeof schema, 'string')

  var [credDefId, credDef] = await indy.issuerCreateAndStoreCredentialDef(wh, issuerDid, schema, 'tag1', 'CL', {
    support_revocation: true
  })
  t.not(typeof credDef, 'string')

  // Issuer create Revocation Registry
  var tailsWriterConfig = {
    'base_dir': path.join(indyHomeDir, 'tails'),
    'uri_pattern': ''
  }
  var tailsWriterHandle = await indy.openBlobStorageWriter('default', tailsWriterConfig)
  var [revocRegId, revocRegDef, revocRegEntry] = await indy.issuerCreateAndStoreRevocReg(wh, issuerDid, null, 'tag1', credDefId, {
    max_cred_num: 5
  }, tailsWriterHandle)
  t.not(typeof revocRegDef, 'string')
  t.not(typeof revocRegEntry, 'string')

  // Prover create Master Secret
  var masterSecretName = 'master_secret'
  await indy.proverCreateMasterSecret(wh, masterSecretName)

  // Issuer create credential Offer
  var credOffer = await indy.issuerCreateCredentialOffer(wh, credDefId)

  // Prover create credential Request
  var [credReq, credReqMetadata] = await indy.proverCreateCredentialReq(wh, proverDid, credOffer, credDef, 'master_secret')
  t.not(typeof credReq, 'string')
  t.not(typeof credReqMetadata, 'string')

  // Issuer open Tails reader
  var blobReaderHandle = await indy.openBlobStorageReader('default', tailsWriterConfig)

  // Issuer create credential for credential Request
  var [cred, revId, revDelta] = await indy.issuerCreateCredential(wh, credOffer, credReq, {
    name: {'raw': 'Alex', 'encoded': '1139481716457488690172217916278103335'},
    height: {'raw': '175', 'encoded': '175'},
    age: {'raw': '28', 'encoded': '28'}
  }, revocRegId, blobReaderHandle)
  t.not(typeof cred, 'string')
  t.truthy(/^true /.test(revDelta.value.prevAccum))
  t.truthy(/^false /.test(revDelta.value.accum))

  // Prover process and store credential
  var outCredId = await indy.proverStoreCredential(wh, 'cred_1_id', credReqMetadata, cred, credDef, revocRegDef)
  t.is(typeof outCredId, 'string')

  // Prover gets credentials for Proof Request
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
  var credentialsForProof = await indy.proverGetCredentialsForProofReq(wh, proofReq)

  var credentials = await indy.proverGetCredentials(wh)
  t.truthy(Array.isArray(credentials))
  t.truthy(credentials.length > 0)

  credentials = await indy.proverGetCredentials(wh, {schema_id: schemaId})
  t.truthy(Array.isArray(credentials))
  t.truthy(credentials.length > 0)
  t.is(credentials[0].schema_id, schemaId)

  // Prover gets credentials for Proof Request
  var timestamp = 100
  var revState = await indy.createRevocationState(blobReaderHandle, revocRegDef, revDelta, timestamp, revId)
  t.is(revState.timestamp, 100)

  timestamp = 101
  revState = await indy.updateRevocationState(blobReaderHandle, revState, revocRegDef, revDelta, timestamp, revId)
  t.is(revState.timestamp, 101)

  // Prover create Proof for Proof Request
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

  var proof = await indy.proverCreateProof(wh, proofReq, requestedCredentials, masterSecretName, schemas, credentialDefs, revocStates)

  // Verify the proof
  t.is(proof['requested_proof']['revealed_attrs']['attr1_referent']['raw'], 'Alex')

  var revocRefDefs = {}
  revocRefDefs[revocRegId] = revocRegDef

  var revocRegs = {}
  revocRegs[revocRegId] = {}
  revocRegs[revocRegId][timestamp] = revDelta

  var isValid = await indy.verifierVerifyProof(proofReq, proof, schemas, credentialDefs, revocRefDefs, revocRegs)
  t.is(isValid, true)

  // Revoke the credential
  var revocedDelta = await indy.issuerRevokeCredential(wh, blobReaderHandle, revocRegId, revId)
  t.truthy(/^false /.test(revocedDelta.value.prevAccum))
  t.truthy(/^true /.test(revocedDelta.value.accum))

  var mergedDelta = await indy.issuerMergeRevocationRegistryDeltas(revDelta, revocedDelta)
  t.truthy(/^true /.test(mergedDelta.value.prevAccum))
  t.truthy(/^true /.test(mergedDelta.value.accum))

  await indy.closeWallet(wh)
  await indy.deleteWallet(wName, null)
  pool.cleanup()
})
