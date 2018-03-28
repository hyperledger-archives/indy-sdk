var test = require('ava')
var indy = require('../')

test('abbreviate_verkey', async function (t) {
  var did = 'VsKV7grR1BUE29mG2Fm2kX'
  var verkey = 'GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa'

  t.is(await indy.abbreviate_verkey(did, verkey), '~HYwqs2vrTc8Tn4uBV7NBTe')

  var err = await t.throws(indy.abbreviate_verkey())
  t.is(err.message, 'CommonInvalidParam3')

  err = await t.throws(function () {
    indy.abbreviate_verkey(1, verkey)
  }, Error)
  t.is(err.message, 'Expected String or null for did: abbreviate_verkey(did, full_verkey, cb(err, verkey))')

  err = await t.throws(function () {
    indy.abbreviate_verkey(did, [1, 2, 3])
  }, Error)
  t.is(err.message, 'Expected String or null for full_verkey: abbreviate_verkey(did, full_verkey, cb(err, verkey))')

  err = await t.throws(indy.abbreviate_verkey(null, verkey))
  t.is(err.indy_name, 'CommonInvalidParam3')
  err = await t.throws(indy.abbreviate_verkey(void 0, verkey))
  t.is(err.indy_name, 'CommonInvalidParam3')

  err = await t.throws(indy.abbreviate_verkey(did, null))
  t.is(err.indy_name, 'CommonInvalidParam4')

  err = await t.throws(indy.abbreviate_verkey('?', verkey))
  t.is(err + '', 'IndyError: CommonInvalidStructure')
  t.is(err.indy_code, 113)
  t.is(err.indy_name, 'CommonInvalidStructure')
})
