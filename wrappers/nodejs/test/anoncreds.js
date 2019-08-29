var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var initTestPool = require('./helpers/initTestPool')
var tempy = require('tempy')

test('anoncreds', async function (t) {
  var pool = await initTestPool()
  var walletConfig = { 'id': 'wallet-' + cuid() }
  var walletCredentials = { 'key': 'key' }
  await indy.createWallet(walletConfig, walletCredentials)
  var wh = await indy.openWallet(walletConfig, walletCredentials)
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
    'base_dir': tempy.directory(),
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
  // note that encoding is not standardized by Indy except that 32-bit integers are encoded as themselves. IS-786
  var [cred, revId, revDelta] = await indy.issuerCreateCredential(wh, credOffer, credReq, {
    name: { 'raw': 'Alex', 'encoded': '1139481716457488690172217916278103335' },
    height: { 'raw': '175', 'encoded': '175' },
    age: { 'raw': '28', 'encoded': '28' }
  }, revocRegId, blobReaderHandle)
  t.not(typeof cred, 'string')

  // Prover process and store credential
  var outCredId = await indy.proverStoreCredential(wh, 'cred_1_id', credReqMetadata, cred, credDef, revocRegDef)
  t.is(typeof outCredId, 'string')

  // Prover get Credential
  var credential = await indy.proverGetCredential(wh, outCredId)
  t.not(typeof credential, 'string')
  t.is(credential.schema_id, schemaId)
  t.is(credential.cred_def_id, credDefId)

  // Prover searches Credentials
  var [sh, totalCount] = await indy.proverSearchCredentials(wh, { schema_id: schemaId })
  t.truthy(totalCount > 0)

  var credentials = await indy.proverFetchCredentials(sh, totalCount)
  t.truthy(Array.isArray(credentials))
  t.truthy(credentials.length > 0)
  t.is(credentials[0].schema_id, schemaId)

  await indy.proverCloseCredentialsSearch(sh)

  var nonce = await indy.generateNonce()

  // Prover gets credentials for Proof Request
  var proofReq = {
    'nonce': nonce,
    'name': 'proof_req_1',
    'version': '0.1',
    'requested_attributes': {
      'attr1_referent': { 'name': 'name' }
    },
    'requested_predicates': {
      'predicate1_referent': { 'name': 'age', 'p_type': '>=', 'p_value': 18 }
    },
    'non_revoked': { 'from': 80, 'to': 100 }
  }
  var credentialsForProof = await indy.proverGetCredentialsForProofReq(wh, proofReq)

  credentials = await indy.proverGetCredentials(wh)
  t.truthy(Array.isArray(credentials))
  t.truthy(credentials.length > 0)

  credentials = await indy.proverGetCredentials(wh, { schema_id: schemaId })
  t.truthy(Array.isArray(credentials))
  t.truthy(credentials.length > 0)
  t.is(credentials[0].schema_id, schemaId)

  // Prover searches Credentials for Proof Request
  sh = await indy.proverSearchCredentialsForProofReq(wh, proofReq, null)

  credentials = await indy.proverFetchCredentialsForProofReq(sh, 'attr1_referent', 100)
  t.truthy(Array.isArray(credentials))
  t.truthy(credentials.length > 0)
  t.is(credentials[0]['cred_info'].schema_id, schemaId)

  credentials = await indy.proverFetchCredentialsForProofReq(sh, 'predicate1_referent', 100)
  t.truthy(Array.isArray(credentials))
  t.truthy(credentials.length > 0)
  t.is(credentials[0]['cred_info'].schema_id, schemaId)

  await indy.proverCloseCredentialsSearchForProofReq(sh)

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
    'requested_attributes': { 'attr1_referent': { 'cred_id': referent, 'revealed': true, 'timestamp': timestamp } },
    'requested_predicates': { 'predicate1_referent': { 'cred_id': referent, 'timestamp': timestamp } }
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

  await indy.issuerMergeRevocationRegistryDeltas(revDelta, revocedDelta)

  // Rotate credential definition
  var tempCredDef = await indy.issuerRotateCredentialDefStart(wh, credDefId, null)
  t.not(cred, tempCredDef)

  await indy.issuerRotateCredentialDefApply(wh, credDefId)

  await indy.closeWallet(wh)
  await indy.deleteWallet(walletConfig, walletCredentials)
  pool.cleanup()
})
