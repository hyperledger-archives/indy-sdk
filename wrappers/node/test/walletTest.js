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
    const info = await Wallet.getTokenInfo()
    assert(info)
  })

  // sendToken tests
  it('errors when sending tokens and has insufficient amount', async () => {
    try {
      const receipt = await Wallet.sendTokens({
        payment: 0,
        tokens: 30,
        recipient: 'address'
      })
      assert(receipt)
    } catch (error) {
      assert.equal(error.vcxCode, 1064)
      assert.equal(error.message, 'Insufficient amount of tokens to process request')
    }
  })

  // createPaymentAddress tests
  it('can create a payment address', async () => {
    const address = await Wallet.createPaymentAddress()
    assert(address)
  })

  // wallet store
  it('can perform record operations', async () => {
    await Wallet.addRecord(WALLET_RECORD)
    await Wallet.getRecord({ type: WALLET_RECORD.type_, id: WALLET_RECORD.id })
    await Wallet.updateRecordValue(UPDATE_WALLET_RECORD)
    await Wallet.updateRecordTags(UPDATE_WALLET_TAGS)
    await Wallet.addRecordTags(UPDATE_WALLET_TAGS)
    await Wallet.deleteRecordTags(WALLET_RECORD, { tagList: ['one', 'two'] })
    await Wallet.deleteRecord({ type: WALLET_RECORD.type_, id: WALLET_RECORD.id })
    const searchHandle = await Wallet.openSearch({
      type: WALLET_RECORD.type_,
      queryJson: JSON.stringify(QUERY_JSON),
      options: 'null'
    })
    assert(searchHandle === 1)
    const retrievedRecords = JSON.parse(await Wallet.searchNextRecords(searchHandle, { count: 1 }))
    assert(JSON.stringify(retrievedRecords) === JSON.stringify(SEARCHED_RECORD))
    await Wallet.closeSearch(searchHandle)
  })
})
