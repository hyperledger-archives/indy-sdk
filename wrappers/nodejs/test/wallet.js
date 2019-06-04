var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var path = require('path')
var fs = require('fs')
var initTestPool = require('./helpers/initTestPool')
var indyHomeDir = require('home-dir')('.indy_client')

test('wallet', async function (t) {
  var pool = await initTestPool()

  var walletConfig = { 'id': 'wallet-' + cuid() }
  var walletCredentials = { 'key': 'key' }
  await indy.createWallet(walletConfig, walletCredentials)

  var err = await t.throwsAsync(indy.createWallet(walletConfig, walletCredentials))
  t.is(err.indyName, 'WalletAlreadyExistsError')

  var handle = await indy.openWallet(walletConfig, walletCredentials)
  t.truthy(handle >= 0)

  //  err = await t.throwsAsync(indy.openWallet(walletConfig, walletCredentials))
  //  t.is(err.indyName, 'WalletAlreadyOpenedError')

  err = await t.throwsAsync(indy.closeWallet(-1))
  t.is(err.indyName, 'WalletInvalidHandle')

  var [did] = await indy.createAndStoreMyDid(handle, {})
  var didBeforeExport = await indy.getMyDidWithMeta(handle, did)

  var exportPath = path.join(indyHomeDir, 'export_wallet-' + cuid())
  var exportConfig = {
    'key': 'export_key',
    'path': exportPath
  }
  await indy.exportWallet(handle, exportConfig)

  t.is(fs.existsSync(exportPath), true)

  await indy.closeWallet(handle)

  await indy.deleteWallet(walletConfig, walletCredentials)

  await indy.importWallet(walletConfig, walletCredentials, exportConfig)

  handle = await indy.openWallet(walletConfig, walletCredentials)
  t.truthy(handle >= 0)

  var didAfterImport = await indy.getMyDidWithMeta(handle, did)

  t.deepEqual(didBeforeExport, didAfterImport)

  await indy.closeWallet(handle)
  await indy.deleteWallet(walletConfig, walletCredentials)

  var key = await indy.generateWalletKey({})
  walletCredentials = { 'key': key, 'key_derivation_method': 'RAW' }
  await indy.createWallet(walletConfig, walletCredentials)
  await indy.deleteWallet(walletConfig, walletCredentials)

  pool.cleanup()
})
