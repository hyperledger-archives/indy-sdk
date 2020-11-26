const test = require('ava')
const indy = require('../')
const cuid = require('cuid')
const initTestPool = require('./helpers/initTestPool')

test('crypto', async function (t) {
  const pool = await initTestPool()
  const walletConfig = { id: 'wallet-' + cuid() }
  const walletCredentials = { key: 'key' }
  await indy.createWallet(walletConfig, walletCredentials)
  const wh = await indy.openWallet(walletConfig, walletCredentials)

  // Create Key
  const error = await t.throwsAsync(indy.createKey(-1, {}))
  t.is(error.indyName, 'WalletInvalidHandle')

  let verkey = await indy.createKey(wh, {})
  t.is(typeof verkey, 'string')

  const seed1 = '00000000000000000000000000000My1'
  verkey = await indy.createKey(wh, { seed: seed1 })
  t.is(typeof verkey, 'string')

  // Sign + Verify
  const message = Buffer.from('{"reqId":1496822211362017764}', 'utf8')
  const signature = await indy.cryptoSign(wh, verkey, message)
  t.true(Buffer.isBuffer(signature))
  t.is(signature.toString('base64'), 'qdcI4QdrbgnBosrWokLu0z/RDMQI0zcbeF7MkzVoZz08+e1/Zy7c3wpfSzX10vGXvykwHgkQTvydztKRfYVtCw==')
  t.true(await indy.cryptoVerify(verkey, message, signature))
  t.false(await indy.cryptoVerify(verkey, Buffer.from('wat?', 'utf8'), signature))

  // Metadata
  await indy.setKeyMetadata(wh, verkey, 'foobar')
  const metadata = await indy.getKeyMetadata(wh, verkey)
  t.is(metadata, 'foobar')

  // Auth
  const [, stewardVerkey] = await indy.createAndStoreMyDid(wh, { seed: '000000000000000000000000Steward1' })
  const [, trusteeVerkey] = await indy.createAndStoreMyDid(wh, { seed: '000000000000000000000000Trustee1' })

  let encrypted = await indy.cryptoAuthCrypt(wh, stewardVerkey, trusteeVerkey, message)
  t.true(Buffer.isBuffer(encrypted))

  let decrypted = await indy.cryptoAuthDecrypt(wh, trusteeVerkey, encrypted)
  t.is(decrypted[0], stewardVerkey)
  t.true(Buffer.isBuffer(decrypted[1]))
  t.is(decrypted[1].toString('utf8'), message.toString('utf8'))

  // Anon
  encrypted = await indy.cryptoAnonCrypt(trusteeVerkey, message)
  t.true(Buffer.isBuffer(encrypted))

  decrypted = await indy.cryptoAnonDecrypt(wh, trusteeVerkey, encrypted)
  t.true(Buffer.isBuffer(decrypted))
  t.is(decrypted.toString('utf8'), message.toString('utf8'))

  // Pack Auth Crypt
  const [, senderVerkey] = await indy.createAndStoreMyDid(wh, {})
  const receiverKeys = [trusteeVerkey, stewardVerkey]

  let packedMessage = await indy.packMessage(wh, message, receiverKeys, senderVerkey)
  t.true(Buffer.isBuffer(packedMessage))

  let unpackedMessage = await indy.unpackMessage(wh, packedMessage)
  t.true(Buffer.isBuffer(unpackedMessage))

  unpackedMessage = JSON.parse(unpackedMessage.toString('utf8'))
  t.is(unpackedMessage.message, message.toString('utf8'))
  t.is(unpackedMessage.sender_verkey, senderVerkey)
  t.is(unpackedMessage.recipient_verkey, trusteeVerkey)

  // Pack Anon Crypt
  packedMessage = await indy.packMessage(wh, message, receiverKeys, null)
  t.true(Buffer.isBuffer(packedMessage))

  unpackedMessage = await indy.unpackMessage(wh, packedMessage)
  t.true(Buffer.isBuffer(unpackedMessage))

  unpackedMessage = JSON.parse(unpackedMessage.toString('utf8'))
  t.is(unpackedMessage.message, message.toString('utf8'))
  t.is(unpackedMessage.sender_verkey, undefined)
  t.is(unpackedMessage.recipient_verkey, trusteeVerkey)

  await indy.closeWallet(wh)
  await indy.deleteWallet(walletConfig, walletCredentials)
  pool.cleanup()
})
