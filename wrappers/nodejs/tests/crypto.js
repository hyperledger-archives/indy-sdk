var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var initTestPool = require('./helpers/initTestPool')

test('crypto', async function (t) {
  var pool = await initTestPool()

  var wName = 'wallet-' + cuid()

  await indy.createWallet(pool.name, wName, 'default', null, null)
  var wHandle = await indy.openWallet(wName, null, null)

  var seed1 = '00000000000000000000000000000My1'
  var verkey = await indy.createKey(wHandle, {'seed': seed1})

  var message = Buffer.from('{"reqId":1496822211362017764}', 'utf8')

  var signed = await indy.cryptoSign(wHandle, verkey, message)
  t.truthy(Buffer.isBuffer(signed))
  t.is(signed.toString('base64'), 'qdcI4QdrbgnBosrWokLu0z/RDMQI0zcbeF7MkzVoZz08+e1/Zy7c3wpfSzX10vGXvykwHgkQTvydztKRfYVtCw==')

  await indy.closeWallet(wHandle)
  await indy.deleteWallet(wName, null)
  pool.cleanup()
})
