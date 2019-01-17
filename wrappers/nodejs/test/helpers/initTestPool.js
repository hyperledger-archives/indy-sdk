var indy = require('../../')
var makeTestPool = require('./makeTestPool')

module.exports = async function () {
  await indy.setProtocolVersion(2)

  var pool = await makeTestPool()
  await indy.createPoolLedgerConfig(pool.name, {
    'genesis_txn': pool.file
  })
  var poolH = await indy.openPoolLedger(pool.name)

  return {
    name: pool.name,
    file: pool.file,
    handle: poolH,
    cleanup: async function () {
      await indy.closePoolLedger(poolH)
      await indy.deletePoolLedgerConfig(pool.name)
    }
  }
}
