var test = require('ava')
var indy = require('../').promise

test('abbreviate_verkey', async function (t) {
  var did = 'VsKV7grR1BUE29mG2Fm2kX'
  var verkey = 'GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa'

  t.is(await indy.abbreviate_verkey(did, verkey), '~HYwqs2vrTc8Tn4uBV7NBTe')

  var err = await t.throws(indy.abbreviate_verkey())
  t.is(err.message, 'Expected 3 arguments: abbreviate_verkey(did, full_verkey, cb(err, verkey))')

  err = await t.throws(indy.abbreviate_verkey(did, verkey, did))
  t.is(err.message, 'Expected 3 arguments: abbreviate_verkey(did, full_verkey, cb(err, verkey))')

  err = await t.throws(indy.abbreviate_verkey(1, verkey))
  t.is(err.message, 'Expected String for arg 0: abbreviate_verkey(did, full_verkey, cb(err, verkey))')

  err = await t.throws(indy.abbreviate_verkey(did, 2))
  t.is(err.message, 'Expected String for arg 1: abbreviate_verkey(did, full_verkey, cb(err, verkey))')

  err = await t.throws(indy.abbreviate_verkey('?', '?'))
  t.is(err + '', 'IndyError: CommonInvalidStructure')
  t.is(err.indy_code, 113)
  t.is(err.indy_name, 'CommonInvalidStructure')
})
