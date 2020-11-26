const test = require('ava')
const indy = require('../')
const cuid = require('cuid')
const initTestPool = require('./helpers/initTestPool')

test('did', async function (t) {
  const pool = await initTestPool()
  const walletConfig = { id: 'wallet-' + cuid() }
  const walletCredentials = { key: 'key' }
  await indy.createWallet(walletConfig, walletCredentials)
  const wh = await indy.openWallet(walletConfig, walletCredentials)

  // List, create, and get
  t.deepEqual(await indy.listMyDidsWithMeta(wh), [])
  let [did, verkey] = await indy.createAndStoreMyDid(wh, { seed: '000000000000000000000000Steward1' })
  t.deepEqual(await indy.listMyDidsWithMeta(wh), [
    { did: did, metadata: null, verkey: verkey, tempVerkey: null }
  ])
  t.is(await indy.keyForLocalDid(wh, did), verkey)
  t.is(await indy.keyForDid(pool.handle, wh, did), verkey)

  // Abbreviate
  const abbr = await indy.abbreviateVerkey(did, verkey)
  t.not(abbr, verkey)

  // Replace
  const verkey2 = await indy.replaceKeysStart(wh, did, {})
  t.is(typeof verkey2, 'string')
  t.not(verkey2, verkey)
  t.is(await indy.keyForLocalDid(wh, did), verkey, 'the verkey should not be replaced yet')
  await indy.replaceKeysApply(wh, did)
  t.is(await indy.keyForLocalDid(wh, did), verkey2, 'the verkey should be changed now')
  verkey = verkey2

  // Store
  await indy.storeTheirDid(wh, { did: 'VsKV7grR1BUE29mG2Fm2kX', verkey: 'GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa' })

  // Endpoint
  const endpoint = '127.0.0.1:9700'
  await indy.setEndpointForDid(wh, did, endpoint, verkey)
  let data = await indy.getEndpointForDid(wh, -1, did)
  t.is(data[0], endpoint)
  t.is(data[1], verkey)

  // Metadata
  const metadata = 'Hi!'
  await indy.setDidMetadata(wh, did, metadata)
  data = await indy.getDidMetadata(wh, did)
  t.is(data, metadata)
  data = await indy.getMyDidWithMeta(wh, did)
  t.deepEqual(data, {
    did: did,
    metadata: metadata,
    verkey: verkey,
    tempVerkey: null
  })
  t.deepEqual(await indy.listMyDidsWithMeta(wh), [data])

  // Qualify DID
  const method = 'peer'
  const fullQualifiedDid = await indy.qualifyDid(wh, did, method)
  t.is('did:' + method + ':' + did, fullQualifiedDid)

  await indy.closeWallet(wh)
  await indy.deleteWallet(walletConfig, walletCredentials)
  pool.cleanup()
})
