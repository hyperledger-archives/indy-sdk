var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var initTestPool = require('./helpers/initTestPool')
var tempy = require('tempy')

function sleep (ms) {
  return new Promise(function (resolve) {
    setTimeout(resolve, ms)
  })
}

async function waitUntilApplied (ph, req, cond) {
  for (let i = 0; i < 3; i++) {
    let res = await indy.submitRequest(ph, req)

    if (cond(res)) {
      return res
    }

    await sleep(5 * 1000)
  }
}

test('ledger', async function (t) {
  var pool = await initTestPool()
  var walletConfig = {'id': 'wallet-' + cuid()}
  var walletCredentials = {'key': 'key'}
  await indy.createWallet(walletConfig, walletCredentials)
  var wh = await indy.openWallet(walletConfig, walletCredentials)
  var [trusteeDid] = await indy.createAndStoreMyDid(wh, {seed: '000000000000000000000000Trustee1'})
  var [myDid, myVerkey] = await indy.createAndStoreMyDid(wh, {seed: '00000000000000000000000000000My1', cid: true})
  var schemaName = 'schema-' + cuid()
  var [schemaId, schema] = await indy.issuerCreateSchema(myDid, schemaName, '1.0', ['name', 'age'])

  // Nym
  var req = await indy.buildNymRequest(trusteeDid, myDid, myVerkey, null, 'TRUSTEE')
  var res = await indy.signAndSubmitRequest(pool.handle, wh, trusteeDid, req)
  t.is(res.result.txn.data.verkey, myVerkey)

  req = await indy.buildGetNymRequest(trusteeDid, myDid)
  t.is(req.identifier, trusteeDid)
  t.is(req.operation.dest, myDid)

  // Schema
  req = await indy.buildSchemaRequest(myDid, schema)
  req = await indy.signRequest(wh, myDid, req)
  res = await indy.submitRequest(pool.handle, req)

  req = await indy.buildGetSchemaRequest(myDid, schemaId)
  res = await waitUntilApplied(pool.handle, req, res => res['result']['seqNo'] != null)
  var data = await indy.parseGetSchemaResponse(res)
  t.is(data[0], schemaId)
  t.is(data[1].name, schema.name)
  req = await indy.buildGetTxnRequest(myDid, null, data[1].seqNo)
  res = await waitUntilApplied(pool.handle, req, res => res['result']['data']['txnMetadata']['seqNo'] != null)
  t.is(res.result.data.txn.data.data.name, schema.name)
  schema = data[1]

  // Node
  req = await indy.buildNodeRequest(myDid, myDid, {
    node_ip: '10.0.0.100',
    node_port: 9710,
    client_ip: '10.0.0.100',
    client_port: 9709,
    alias: 'Node5',
    services: ['VALIDATOR'],
    blskey: 'CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW'
  })
  res = await indy.submitRequest(pool.handle, req)
  t.is(res.op, 'REQNACK')

  // Attrib
  req = await indy.buildAttribRequest(myDid, myDid, null, {endpoint: {ha: '127.0.0.1:5555'}}, null)
  res = await indy.signAndSubmitRequest(pool.handle, wh, myDid, req)

  req = await indy.buildGetAttribRequest(myDid, myDid, 'endpoint', null, null)
  res = await waitUntilApplied(pool.handle, req, data => data['result']['data'] != null)
  t.deepEqual(JSON.parse(res.result.data), {endpoint: {ha: '127.0.0.1:5555'}})

  // Pool
  req = await indy.buildPoolConfigRequest(myDid, false, false)
  t.false(req.operation.writes)

  req = await indy.buildPoolRestartRequest(myDid, 'start', '0')
  t.is(req.operation.action, 'start')

  req = await indy.buildPoolUpgradeRequest(myDid, 'some upgrade action', '2.0.0', 'cancel', 'abc12345', -1, null, null, false, false)
  t.is(req.operation.name, 'some upgrade action')

  // DDO
  req = await indy.buildGetDdoRequest(myDid, trusteeDid)
  t.is(req.operation.dest, trusteeDid)

  // Cred Def
  var [credDefId, credDef] = await indy.issuerCreateAndStoreCredentialDef(wh, myDid, schema, 'TAG', 'CL', {support_revocation: true})
  req = await indy.buildCredDefRequest(myDid, credDef)
  res = await indy.signAndSubmitRequest(pool.handle, wh, myDid, req)

  req = await indy.buildGetCredDefRequest(myDid, credDefId)
  res = await waitUntilApplied(pool.handle, req, res => res['result']['seqNo'] != null)
  res = await indy.parseGetCredDefResponse(res)
  t.is(res[0], credDefId)
  t.is(res[1].id, credDef.id)

  // Revoc Reg Def
  var writerH = await indy.openBlobStorageWriter('default', {
    'base_dir': tempy.directory(),
    'uri_pattern': ''
  })
  var [revRegDefId, revRegDef, revRegEntry] = await indy.issuerCreateAndStoreRevocReg(wh, myDid, null, 'tag1', credDefId, {max_cred_num: 5}, writerH)

  req = await indy.buildRevocRegDefRequest(myDid, revRegDef)
  res = await indy.signAndSubmitRequest(pool.handle, wh, myDid, req)
  t.is(res.result.txn.data.id, revRegDefId)

  req = await indy.buildGetRevocRegDefRequest(myDid, revRegDefId)
  res = await waitUntilApplied(pool.handle, req, res => res['result']['seqNo'] != null)
  res = await indy.parseGetRevocRegDefResponse(res)
  t.is(res[0], revRegDefId)
  t.is(res[1].id, revRegDef.id)

  // Revoc Reg Entry
  req = await indy.buildRevocRegEntryRequest(myDid, revRegDefId, 'CL_ACCUM', revRegEntry)
  res = await indy.signAndSubmitRequest(pool.handle, wh, myDid, req)

  var nowSeconds = Math.floor(Date.now() / 1000)
  req = await indy.buildGetRevocRegRequest(myDid, revRegDefId, nowSeconds + 100)
  res = await waitUntilApplied(pool.handle, req, res => res['result']['seqNo'] != null)
  res = await indy.parseGetRevocRegResponse(res)
  t.is(res[0], revRegDefId)
  t.is(typeof res[1], 'object')
  t.is(typeof res[2], 'number')

  // RevocRegDelta
  req = await indy.buildGetRevocRegDeltaRequest(myDid, revRegDefId, nowSeconds, nowSeconds + 100)
  res = await waitUntilApplied(pool.handle, req, res => res['result']['seqNo'] != null)
  res = await indy.parseGetRevocRegDeltaResponse(res)
  t.is(res[0], revRegDefId)
  t.is(typeof res[1], 'object')
  t.is(typeof res[2], 'number')

  await indy.closeWallet(wh)
  await indy.deleteWallet(walletConfig, walletCredentials)
  pool.cleanup()
})
