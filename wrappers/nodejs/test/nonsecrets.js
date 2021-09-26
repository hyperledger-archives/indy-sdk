const test = require('ava')
const indy = require('../')
const cuid = require('cuid')

test('nonsecrets', async function (t) {
  const walletConfig = { id: 'wallet-' + cuid() }
  const walletCredentials = { key: 'key' }
  await indy.createWallet(walletConfig, walletCredentials)
  const wh = await indy.openWallet(walletConfig, walletCredentials)

  await indy.addWalletRecord(wh, 'contact', '1', 'john', JSON.stringify({
    '~score': 'aaa'
  }))
  await indy.addWalletRecord(wh, 'contact', '2', 'jessica', JSON.stringify({
    '~score': 'ccc'
  }))
  await indy.addWalletRecord(wh, 'contact', '3', 'jack', JSON.stringify({
    '~score': 'eee',
    '~note': 'foo'
  }))

  await indy.updateWalletRecordValue(wh, 'contact', '3', 'george')
  await indy.updateWalletRecordTags(wh, 'contact', '3', JSON.stringify({
    '~score': 'fff'
  }))

  const record = await indy.getWalletRecord(wh, 'contact', '1', '{}')
  t.is(record.id, '1')
  t.is(record.value, 'john')
  t.is(record.tags, null)
  t.is(record.type, null)

  const record2 = await indy.getWalletRecord(wh, 'contact', '1', '{"retrieveType": true, "retrieveTags": true}')
  t.is(record2.id, '1')
  t.is(record2.value, 'john')
  t.deepEqual(record2.tags, {
    '~score': 'aaa'
  })
  t.is(record2.type, 'contact')

  const query = { '~score': { $gte: 'bbb' } }
  const searchHandle = await indy.openWalletSearch(wh, 'contact', JSON.stringify(query), JSON.stringify({
    retrieveRecords: true,
    retrieveTotalCount: true,
    retrieveType: true,
    retrieveValue: true,
    retrieveTags: true
  }))
  const searchResult = await indy.fetchWalletSearchNextRecords(wh, searchHandle, 10)
  t.is(searchResult.totalCount, 2)
  const jessica = searchResult.records.find(r => r.value === 'jessica')
  t.is(jessica.id, '2')
  t.is(jessica.type, 'contact')
  t.is(jessica.tags['~score'], 'ccc')
  const jack = searchResult.records.find(r => r.value === 'george')
  t.is(jack.id, '3')
  t.is(jack.type, 'contact')
  t.is(jack.tags['~score'], 'fff')
  t.is(jack.tags['~note'], undefined)

  await indy.closeWalletSearch(searchHandle)
  await indy.closeWallet(wh)
  await indy.deleteWallet(walletConfig, walletCredentials)
})
