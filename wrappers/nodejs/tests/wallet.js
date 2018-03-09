var test = require('ava')
var indy = require('../').promise
var cuid = require('cuid')
var initTestPool = require('./helpers/initTestPool')

test('wallet', async function (t) {
  var pool = await initTestPool()

  var listWallets = async function () {
    var list = JSON.parse(await indy.list_wallets())
    return list
      .filter(a => a.associated_pool_name === pool.name)
      .map(a => ({name: a.name, type: a.type}))
  }

  t.deepEqual(await listWallets(), [])

  var wName = 'wallet-' + cuid()

  await indy.create_wallet(pool.name, wName, 'default', null, null)

  t.deepEqual(await listWallets(), [{name: wName, type: 'default'}])

  var err = await t.throws(indy.create_wallet(pool.name, wName, 'default', null, null))
  t.is(err.indy_name, 'WalletAlreadyExistsError')

  var handle = await indy.open_wallet(wName, null, null)
  t.truthy(handle >= 0)

  err = await t.throws(indy.open_wallet(wName, null, null))
  t.is(err.indy_name, 'WalletAlreadyOpenedError')

  err = await t.throws(indy.close_wallet(-1))
  t.is(err.indy_name, 'WalletInvalidHandle')

  await indy.close_wallet(handle)

  t.deepEqual(await listWallets(), [{name: wName, type: 'default'}])

  await indy.delete_wallet(wName, null)

  t.deepEqual(await listWallets(), [])

  pool.cleanup()
})
