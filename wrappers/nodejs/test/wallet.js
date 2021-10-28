const test = require('ava')
const indy = require('../')
const cuid = require('cuid')
const path = require('path')
const fs = require('fs')
const initTestPool = require('./helpers/initTestPool')
const indyHomeDir = require('home-dir')('.indy_client')

test('wallet', async function (t) {
  const pool = await initTestPool()

  const walletConfig = { id: 'wallet-' + cuid() }
  let walletCredentials = { key: 'key' }
  await indy.createWallet(walletConfig, walletCredentials)

  let err = await t.throwsAsync(indy.createWallet(walletConfig, walletCredentials))
  t.is(err.indyName, 'WalletAlreadyExistsError')

  let handle = await indy.openWallet(walletConfig, walletCredentials)
  t.truthy(handle >= 0)

  //  err = await t.throwsAsync(indy.openWallet(walletConfig, walletCredentials))
  //  t.is(err.indyName, 'WalletAlreadyOpenedError')

  err = await t.throwsAsync(indy.closeWallet(-1))
  t.is(err.indyName, 'WalletInvalidHandle')

  const [did] = await indy.createAndStoreMyDid(handle, {})
  const didBeforeExport = await indy.getMyDidWithMeta(handle, did)

  const exportPath = path.join(indyHomeDir, 'export_wallet-' + cuid())
  const exportConfig = {
    key: 'export_key',
    path: exportPath
  }
  await indy.exportWallet(handle, exportConfig)

  t.is(fs.existsSync(exportPath), true)

  await indy.closeWallet(handle)

  await indy.deleteWallet(walletConfig, walletCredentials)

  await indy.importWallet(walletConfig, walletCredentials, exportConfig)

  handle = await indy.openWallet(walletConfig, walletCredentials)
  t.truthy(handle >= 0)

  const didAfterImport = await indy.getMyDidWithMeta(handle, did)

  t.deepEqual(didBeforeExport, didAfterImport)

  await indy.closeWallet(handle)
  await indy.deleteWallet(walletConfig, walletCredentials)

  const key = await indy.generateWalletKey({})
  walletCredentials = { key: key, key_derivation_method: 'RAW' }
  await indy.createWallet(walletConfig, walletCredentials)
  await indy.deleteWallet(walletConfig, walletCredentials)

  pool.cleanup()
})
