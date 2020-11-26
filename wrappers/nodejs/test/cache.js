const test = require('ava')
const indy = require('../')
const cuid = require('cuid')
const initTestPool = require('./helpers/initTestPool')

function sleep (ms) {
  return new Promise(function (resolve) {
    setTimeout(resolve, ms)
  })
}

test('cache', async function (t) {
  const pool = await initTestPool()
  const walletConfig = { id: 'wallet-' + cuid() }
  const walletCredentials = { key: 'key' }
  await indy.createWallet(walletConfig, walletCredentials)
  const wh = await indy.openWallet(walletConfig, walletCredentials)
  const [trusteeDid] = await indy.createAndStoreMyDid(wh, { seed: '000000000000000000000000Trustee1' })
  const [myDid, myVerkey] = await indy.createAndStoreMyDid(wh, { })
  const schemaName = 'schema-' + cuid()
  let [schemaId, schema] = await indy.issuerCreateSchema(myDid, schemaName, '1.0', ['name', 'age'])

  // Nym
  const nreq = await indy.buildNymRequest(trusteeDid, myDid, myVerkey, null, 'TRUSTEE')
  const nres = await indy.signAndSubmitRequest(pool.handle, wh, trusteeDid, nreq)
  t.is(nres.result.txn.data.verkey, myVerkey)

  const defaultGetCacheOptions = {
    noCache: false,
    noUpdate: false,
    noStore: false,
    minFresh: -1
  }

  const defaultPurgeCacheOptions = {
    maxAge: -1
  }

  // Schema
  let req = await indy.buildSchemaRequest(myDid, schema)
  req = await indy.signRequest(wh, myDid, req)
  await indy.submitRequest(pool.handle, req)

  await sleep(5 * 1000)

  const schemaRes = await indy.getSchema(pool.handle, wh, myDid, schemaId, defaultGetCacheOptions)
  t.is(schemaRes.name, schema.name)
  schema = schemaRes

  await indy.purgeSchemaCache(wh, defaultPurgeCacheOptions)

  // Cred Def
  const [credDefId, credDef] = await indy.issuerCreateAndStoreCredentialDef(wh, myDid, schema, 'TAG', 'CL', { support_revocation: false })
  req = await indy.buildCredDefRequest(myDid, credDef)
  await indy.signAndSubmitRequest(pool.handle, wh, myDid, req)

  await sleep(5 * 1000)

  const credDefRes = await indy.getCredDef(pool.handle, wh, myDid, credDefId, defaultGetCacheOptions)
  t.is(credDefRes.id, credDef.id)

  await indy.purgeCredDefCache(wh, defaultPurgeCacheOptions)

  // cleanup
  await indy.closeWallet(wh)
  await indy.deleteWallet(walletConfig, walletCredentials)
  pool.cleanup()
})
