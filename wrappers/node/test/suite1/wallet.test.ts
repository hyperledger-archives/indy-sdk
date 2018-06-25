import '../module-resolver-helper'

import { assert } from 'chai'
import { initVcxTestMode } from 'helpers/utils'
import { Wallet } from 'src'

const WALLET_RECORD = {
  id: 'RecordId',
  tags: {},
  type_: 'TestType',
  value: 'RecordValue'
}

const QUERY_JSON = { tagName1: 'str1' }

const UPDATE_WALLET_RECORD = {
  id: 'RecordId',
  type_: 'TestType',
  value: 'RecordValueNew'
}
const UPDATE_WALLET_TAGS = {
  id: 'RecordId',
  tags: {},
  type_: 'TestType',
  value: ''
}

const TAGS = '{"tagName1":"str1","tagName2":"5","tagName3":"12"}'

const SEARCHED_RECORD = {
  id: 'RecordId',
  tags: TAGS,
  type: null,
  value: 'RecordValue'
}

describe('Wallet:', () => {
  before(() => initVcxTestMode())

  describe('getTokenInfo:', () => {
    it('success', async () => {
      const info = await Wallet.getTokenInfo()
      assert.ok(info)
    })
  })

  describe('sendTokens:', () => {
    it('success', async () => {
      const receipt = await Wallet.sendTokens({
        payment: 0,
        recipient: 'address',
        tokens: 30
      })
      assert.ok(receipt)
    })
  })

  describe('createPaymentAddress:', () => {
    it('success', async () => {
      const address = await Wallet.createPaymentAddress()
      assert.ok(address)
    })
  })

  describe('records:', () => {
    it('success', async () => {
      await Wallet.addRecord(WALLET_RECORD)
      await Wallet.getRecord({ type: WALLET_RECORD.type_, id: WALLET_RECORD.id })
      await Wallet.updateRecordValue(UPDATE_WALLET_RECORD)
      await Wallet.updateRecordTags(UPDATE_WALLET_TAGS)
      await Wallet.addRecordTags(UPDATE_WALLET_TAGS)
      await Wallet.deleteRecordTags(WALLET_RECORD, { tagList: ['one', 'two'] })
      await Wallet.deleteRecord({ type: WALLET_RECORD.type_, id: WALLET_RECORD.id })
      const searchHandle = await Wallet.openSearch({
        options: 'null',
        queryJson: JSON.stringify(QUERY_JSON),
        type: WALLET_RECORD.type_
      })
      assert(searchHandle === 1)
      const retrievedRecords = JSON.parse(await Wallet.searchNextRecords(searchHandle, { count: 1 }))
      assert.deepEqual(retrievedRecords, SEARCHED_RECORD)
      await Wallet.closeSearch(searchHandle)
    })
  })
})
