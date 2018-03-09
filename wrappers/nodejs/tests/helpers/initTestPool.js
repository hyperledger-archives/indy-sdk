var indy = require('../../').promise
var makeTestPool = require('./makeTestPool')

module.exports = async function () {
  var pool = await makeTestPool()
  await indy.create_pool_ledger_config(pool.name, JSON.stringify({
    'genesis_txn': pool.file
  }))
  var poolH = await indy.open_pool_ledger(pool.name, 'null')

  return {
    name: pool.name,
    handle: poolH,
    cleanup: async function () {
      await indy.close_pool_ledger(poolH)
      await indy.delete_pool_ledger_config(pool.name)
      pool.cleanup()
    }
  }
}
