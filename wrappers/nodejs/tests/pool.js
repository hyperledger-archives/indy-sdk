var test = require('ava')
var indy = require('../').promise

test('create_pool_ledger_config', async function (t) {
  var err = await t.throws(indy.create_pool_ledger_config())
  t.is(err.message, 'Expected 3 arguments: create_pool_ledger_config(config_name, config, cb(err))')

  err = await t.throws(indy.create_pool_ledger_config('', ''))
  t.is(err, 'CommonInvalidParam2')
})
