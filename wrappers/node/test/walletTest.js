const chai = require('chai')
const vcx = require('../dist')
const { stubInitVCX } = require('./helpers')
const assert = chai.assert

const { Wallet } = vcx
const WALLET_RECORD = {
  type_: 'TestType',
  id: 'RecordId',
  value: 'RecordValue',
  tags: {}
}

const QUERY_JSON = {'tagName1': 'str1'}

const UPDATE_WALLET_RECORD = {
  type_: 'TestType',
  id: 'RecordId',
  value: 'RecordValueNew'
}
const UPDATE_WALLET_TAGS = {
  type_: 'TestType',
  id: 'RecordId',
  value: '',
  tags: {}
}

const TAGS = '{"tagName1":"str1","tagName2":"5","tagName3":"12"}'

const SEARCHED_RECORD = {
  'id': 'RecordId',
  'type': null,
  'value': 'RecordValue',
  'tags': TAGS
}

describe('A Connection object with ', function () {
  this.timeout(10000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  // getTokenInfo tests
  it('can get token info', async () => {
    const info = await Wallet.getTokenInfo(0)
    assert(info)
  })

  // sendToken tests
  it('can send tokens', async () => {
    const receipt = await Wallet.sendTokens(0, 30, 'address')
    assert(receipt)
  })

  // createPaymentAddress tests
  it('can create a payment address', async () => {
    const address = await Wallet.createPaymentAddress()
    assert(address)
  })

  // wallet store
  it('can perform record operations', async () => {
    await Wallet.addRecord(WALLET_RECORD)
    await Wallet.getRecord(WALLET_RECORD.type_, WALLET_RECORD.id)
    await Wallet.updateRecordValue(UPDATE_WALLET_RECORD)
    await Wallet.updateRecordTags(UPDATE_WALLET_TAGS)
    await Wallet.addRecordTags(UPDATE_WALLET_TAGS)
    await Wallet.deleteRecordTags(WALLET_RECORD, ['one', 'two'])
    await Wallet.deleteRecord(WALLET_RECORD.type_, WALLET_RECORD.id)
    var searchHandle = await Wallet.openSearch(WALLET_RECORD.type_, JSON.stringify(QUERY_JSON), 'null')
    assert(searchHandle === 1)
    var retrievedRecords = JSON.parse(await Wallet.searchNextRecords(searchHandle, 1))
    assert(JSON.stringify(retrievedRecords) === JSON.stringify(SEARCHED_RECORD))
    await Wallet.closeSearch(searchHandle)
  })
})
