var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var initTestPool = require('./helpers/initTestPool')

test('payments', async function (t) {
  var pool = await initTestPool()

  var walletConfig = {'id': 'wallet-' + cuid()}
  var walletCredentials = {'key': 'key'}
  await indy.createWallet(walletConfig, walletCredentials)

  var wh = await indy.openWallet(walletConfig, walletCredentials)

  var [trusteeDid] = await indy.createAndStoreMyDid(wh, {seed: '000000000000000000000000Trustee1'})

  var paymentMethod = 'null'
  var paymentAddress = 'pay:null:test'
  var inputs = ['pay:null:1']
  var outputs = [{'recipient': 'pay:null:1', 'amount': 1}]

  var err = await t.throws(indy.createPaymentAddress(wh, paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  t.deepEqual(await indy.listPaymentAddresses(wh), [])

  err = await t.throws(indy.addRequestFees(wh, trusteeDid, {}, inputs, outputs, null))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throws(indy.parseResponseWithFees(paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throws(indy.buildGetPaymentSourcesRequest(wh, trusteeDid, paymentAddress))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throws(indy.parseGetPaymentSourcesResponse(paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throws(indy.buildPaymentReq(wh, trusteeDid, inputs, outputs, null))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throws(indy.parsePaymentResponse(paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throws(indy.buildMintReq(wh, trusteeDid, outputs, null))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  var fees = {'txnType1': 1, 'txnType2': 2}

  err = await t.throws(indy.buildSetTxnFeesReq(wh, trusteeDid, paymentMethod, fees))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throws(indy.buildGetTxnFeesReq(wh, trusteeDid, paymentMethod))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throws(indy.parseGetTxnFeesResponse(paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  var receipt = 'pay:null:0_PqVjwJC42sxCTJp'

  err = await t.throws(indy.buildVerifyPaymentReq(wh, trusteeDid, receipt))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throws(indy.parseVerifyPaymentResponse(paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  await indy.closeWallet(wh)
  await indy.deleteWallet(walletConfig, walletCredentials)

  pool.cleanup()
})
