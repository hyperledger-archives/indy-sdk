var test = require('ava')
var indy = require('../')
var cuid = require('cuid')
var initTestPool = require('./helpers/initTestPool')

test('payments', async function (t) {
  var pool = await initTestPool()

  var walletConfig = { 'id': 'wallet-' + cuid() }
  var walletCredentials = { 'key': 'key' }
  await indy.createWallet(walletConfig, walletCredentials)

  var wh = await indy.openWallet(walletConfig, walletCredentials)

  var [trusteeDid] = await indy.createAndStoreMyDid(wh, { seed: '000000000000000000000000Trustee1' })

  var paymentMethod = 'null'
  var paymentAddress = 'pay:null:test'
  var inputs = ['pay:null:1']
  var outputs = [{ 'recipient': 'pay:null:1', 'amount': 1 }]
  var from = 1

  var err = await t.throwsAsync(indy.createPaymentAddress(wh, paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  t.deepEqual(await indy.listPaymentAddresses(wh), [])

  err = await t.throwsAsync(indy.addRequestFees(wh, trusteeDid, {}, inputs, outputs, null))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throwsAsync(indy.parseResponseWithFees(paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throwsAsync(indy.buildGetPaymentSourcesRequest(wh, trusteeDid, paymentAddress))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throwsAsync(indy.buildGetPaymentSourcesWithFromRequest(wh, trusteeDid, paymentAddress, from))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throwsAsync(indy.parseGetPaymentSourcesWithFromResponse(paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throwsAsync(indy.buildPaymentReq(wh, trusteeDid, inputs, outputs, null))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throwsAsync(indy.parsePaymentResponse(paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  var extra = await indy.preparePaymentExtraWithAcceptanceData(null, 'indy agreement', '1.0.0', null, 'acceptance mechanism label 1', 123379200)
  var expectedExtra = {
    'mechanism': 'acceptance mechanism label 1',
    'taaDigest': '7213b9aabf8677edf6b17d20a9fbfaddb059ea4cb122d163bdf658ea67196120',
    'time': 123379200
  }
  t.deepEqual(extra['taaAcceptance'], expectedExtra)

  err = await t.throwsAsync(indy.buildMintReq(wh, trusteeDid, outputs, null))

  t.is(err.indyName, 'PaymentUnknownMethodError')

  var fees = { 'txnType1': 1, 'txnType2': 2 }

  err = await t.throwsAsync(indy.buildSetTxnFeesReq(wh, trusteeDid, paymentMethod, fees))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throwsAsync(indy.buildGetTxnFeesReq(wh, trusteeDid, paymentMethod))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throwsAsync(indy.parseGetTxnFeesResponse(paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  var receipt = 'pay:null:0_PqVjwJC42sxCTJp'

  err = await t.throwsAsync(indy.buildVerifyPaymentReq(wh, trusteeDid, receipt))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  err = await t.throwsAsync(indy.parseVerifyPaymentResponse(paymentMethod, {}))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  var message = Buffer.from('123456789', 'utf8')
  err = await t.throwsAsync(indy.signWithAddress(wh, paymentAddress, message))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  var signature = Buffer.from('987654321', 'utf8')
  err = await t.throwsAsync(indy.verifyWithAddress(paymentAddress, message, signature))
  t.is(err.indyName, 'PaymentUnknownMethodError')

  var getAuthRuleResp = { 'result': { 'data': [ { 'new_value': '0', 'constraint': { 'need_to_be_owner': false, 'sig_count': 1, 'metadata': { 'fees': '1' }, 'role': '0', 'constraint_id': 'ROLE' }, 'field': 'role', 'auth_type': '1', 'auth_action': 'ADD' } ], 'identifier': 'LibindyDid111111111111', 'auth_action': 'ADD', 'new_value': '0', 'reqId': 15616, 'auth_type': '1', 'type': '121', 'field': 'role' }, 'op': 'REPLY' }
  var requesterInfo = { 'role': '0', 'need_to_be_owner': false, 'sig_count': 1 }
  fees = { '1': 100 }

  var requestInfo = await indy.getRequestInfo(getAuthRuleResp, requesterInfo, fees)
  var expectedRequestInfo = {
    'price': 100,
    'requirements': [{
      'role': '0',
      'need_to_be_owner': false,
      'sig_count': 1
    }]
  }
  t.deepEqual(expectedRequestInfo, requestInfo)

  await indy.closeWallet(wh)
  await indy.deleteWallet(walletConfig, walletCredentials)

  pool.cleanup()
})
