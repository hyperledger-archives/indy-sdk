var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var path = require('path')
var fs = require('fs')
var initTestPool = require('./helpers/initTestPool')
var indyHomeDir = require('home-dir')('.indy_client')

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

  var [did] = await indy.createAndStoreMyDid(handle, {})
  var didBeforeExport = await indy.getMyDidWithMeta(handle, did)

  var exportPath = path.join(indyHomeDir, 'export_wallet')
  var exportConfig = {
    'key': 'export_key',
    'path': exportPath
  }
  await indy.exportWallet(handle, exportConfig)

  t.is(fs.existsSync(exportPath), true)

  await indy.closeWallet(handle)

  t.deepEqual(await listWallets(), [{name: wName, type: 'default'}])

  await indy.deleteWallet(wName, walletCredentials)

  t.deepEqual(await listWallets(), [])

  await indy.importWallet(pool.name, wName, 'default', null, walletCredentials, exportConfig)

  t.deepEqual(await listWallets(), [{name: wName, type: 'default'}])

  handle = await indy.openWallet(wName, null, walletCredentials)
  t.truthy(handle >= 0)

  var didAfterImport = await indy.getMyDidWithMeta(handle, did)

  t.deepEqual(didBeforeExport, didAfterImport)

  await indy.closeWallet(handle)

  pool.cleanup()
})
