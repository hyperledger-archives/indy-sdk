var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var initTestPool = require('./helpers/initTestPool')

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
  var wName = 'wallet-' + cuid()
  await indy.createWallet(pool.name, wName, 'default', null, null)
  var wh = await indy.openWallet(wName, null, null)
  var [trusteeDid, trusteeVerkey] = await indy.createAndStoreMyDid(wh, {seed: '000000000000000000000000Trustee1'})
  var [myDid, myVerkey] = await indy.createAndStoreMyDid(wh, {seed: '00000000000000000000000000000My1', cid: true})
  var schemaName = 'schema-' + cuid()
  var [schemaId, schema] = await indy.issuerCreateSchema(trusteeDid, schemaName, '1.0', ['name', 'age'])

  console.log('trustee-->', trusteeDid, trusteeVerkey)
  console.log('my------->', myDid, myVerkey)

  // Nym
  var req = await indy.buildNymRequest(trusteeDid, myDid, myVerkey, null, null)
  var res = await indy.signAndSubmitRequest(pool.handle, wh, trusteeDid, req)
  t.is(res.result.verkey, myVerkey)

  req = await indy.buildGetNymRequest(trusteeDid, myDid)
  t.is(req.identifier, trusteeDid)
  t.is(req.operation.dest, myDid)

  // Schema
  req = await indy.buildSchemaRequest(trusteeDid, schema)
  req = await indy.signRequest(wh, trusteeDid, req)
  res = await indy.submitRequest(pool.handle, req)

  req = await indy.buildGetSchemaRequest(trusteeDid, schemaId)
  res = await waitUntilApplied(pool.handle, req, res => res['result']['seqNo'] != null)
  var data = await indy.parseGetSchemaResponse(res)
  t.is(data[0], schemaId)
  t.is(data[1].name, schema.name)
  req = await indy.buildGetTxnRequest(myDid, data[1].seqNo)
  res = await waitUntilApplied(pool.handle, req, res => res['result']['data']['seqNo'] != null)
  t.is(res.result.data.data.name, schema.name)

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

  await indy.closeWallet(wh)
  await indy.deleteWallet(wName, null)
  pool.cleanup()
})
/*
  buildGetDdoRequest

  buildPoolConfigRequest
  buildPoolRestartRequest
  buildPoolUpgradeRequest

  buildRevocRegEntryRequest
  buildGetRevocRegRequest
  parseGetRevocRegResponse

  buildRevocRegDefRequest
  buildGetRevocRegDefRequest
  parseGetRevocRegDefResponse

  buildGetRevocRegDeltaRequest
  parseGetRevocRegDeltaResponse

  buildCredDefRequest
  buildGetCredDefRequest
  parseGetCredDefResponse
*/
