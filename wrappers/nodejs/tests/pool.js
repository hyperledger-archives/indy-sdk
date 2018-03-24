var test = require('ava')
var indy = require('../')
var makeTestPool = require('./helpers/makeTestPool')

test('pool', async function (t) {
  var err = await t.throws(indy.create_pool_ledger_config('', ''))
  t.is(err.indy_name, 'CommonInvalidParam2')

  err = await t.throws(indy.create_pool_ledger_config('not_a_real_pool', {
    'genesis_txn': '/not/a/real/file.txn'
  }))
  t.is(err.indy_name, 'CommonIOError')

  var pool = await makeTestPool()

  t.is((await indy.list_pools()).map(p => p.pool).indexOf(pool.name), -1)

  t.is(await indy.create_pool_ledger_config(pool.name, {
    'genesis_txn': pool.file
  }), void 0)

  var poolH = await indy.open_pool_ledger(pool.name, 'null')
  t.truthy(poolH >= 0)

  err = await t.throws(indy.refresh_pool_ledger(-1))
  t.is(err.indy_name, 'PoolLedgerInvalidPoolHandle')
  await indy.refresh_pool_ledger(poolH)

  t.truthy((await indy.list_pools()).map(p => p.pool).indexOf(pool.name) >= 0)

  err = await t.throws(indy.delete_pool_ledger_config(pool.name))
  t.is(err.indy_name, 'CommonInvalidState')

  await indy.close_pool_ledger(poolH)

  await indy.delete_pool_ledger_config(pool.name)

  pool.cleanup()
})
