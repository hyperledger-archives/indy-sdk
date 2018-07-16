var test = require('ava')
var indy = require('../')
var makeTestPool = require('./helpers/makeTestPool')

test('pool', async function (t) {
  var err = await t.throws(indy.createPoolLedgerConfig('', ''))
  t.is(err.indyName, 'CommonInvalidParam2')

  err = await t.throws(indy.createPoolLedgerConfig('not_a_real_pool', {
    'genesis_txn': '/not/a/real/file.txn'
  }))
  t.is(err.indyName, 'CommonIOError')

  var pool = await makeTestPool()

  t.is((await indy.listPools()).map(p => p.pool).indexOf(pool.name), -1)

  t.is(await indy.createPoolLedgerConfig(pool.name, {
    'genesis_txn': pool.file
  }), null)

  await indy.setProtocolVersion(1)

  err = await t.throws(indy.openPoolLedger(pool.name, 'null'))
  t.is(err.indyName, 'PoolIncompatibleProtocolVersion')

  await indy.setProtocolVersion(2)

  var poolH = await indy.openPoolLedger(pool.name, 'null')
  t.truthy(poolH >= 0)

  err = await t.throws(indy.refreshPoolLedger(-1))
  t.is(err.indyName, 'PoolLedgerInvalidPoolHandle')
  await indy.refreshPoolLedger(poolH)

  t.truthy((await indy.listPools()).map(p => p.pool).indexOf(pool.name) >= 0)

  err = await t.throws(indy.deletePoolLedgerConfig(pool.name))
  t.is(err.indyName, 'CommonInvalidState')

  await indy.closePoolLedger(poolH)

  await indy.deletePoolLedgerConfig(pool.name)
})
