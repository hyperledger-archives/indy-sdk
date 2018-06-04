var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var initTestPool = require('./helpers/initTestPool')

test('wallet', async function (t) {
  var pool = await initTestPool()

  var listWallets = async function () {
    var list = await indy.listWallets()
    return list
      .filter(a => a.pool_name === pool.name)
      .map(a => ({name: a.name, type: a.type}))
  }

  t.deepEqual(await listWallets(), [])

  var wName = 'wallet-' + cuid()

  var walletCredentials = {'key': 'key'}
  await indy.createWallet(pool.name, wName, 'default', null, walletCredentials)

  t.deepEqual(await listWallets(), [{name: wName, type: 'default'}])

  var err = await t.throws(indy.createWallet(pool.name, wName, 'default', null, walletCredentials))
  t.is(err.indyName, 'WalletAlreadyExistsError')

  var handle = await indy.openWallet(wName, null, walletCredentials)
  t.truthy(handle >= 0)

  err = await t.throws(indy.openWallet(wName, null, walletCredentials))
  t.is(err.indyName, 'WalletAlreadyOpenedError')

  err = await t.throws(indy.closeWallet(-1))
  t.is(err.indyName, 'WalletInvalidHandle')

  await indy.closeWallet(handle)

  t.deepEqual(await listWallets(), [{name: wName, type: 'default'}])

  await indy.deleteWallet(wName, walletCredentials)

  t.deepEqual(await listWallets(), [])

  pool.cleanup()
})
