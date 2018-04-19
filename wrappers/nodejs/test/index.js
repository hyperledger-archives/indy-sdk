var test = require('ava')
var indy = require('../')

test('abbreviateVerkey', async function (t) {
  var did = 'VsKV7grR1BUE29mG2Fm2kX'
  var verkey = 'GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa'

  t.is(await indy.abbreviateVerkey(did, verkey), '~HYwqs2vrTc8Tn4uBV7NBTe')

  var err = await t.throws(indy.abbreviateVerkey())
  t.is(err.message, 'CommonInvalidParam3')

  err = await t.throws(function () {
    indy.abbreviateVerkey(1, verkey)
  }, Error)
  t.is(err.message, 'Expected String or null for did: abbreviateVerkey(did, full_verkey, cb(err, verkey))')

  err = await t.throws(function () {
    indy.abbreviateVerkey(did, [1, 2, 3])
  }, Error)
  t.is(err.message, 'Expected String or null for full_verkey: abbreviateVerkey(did, full_verkey, cb(err, verkey))')

  err = await t.throws(indy.abbreviateVerkey(null, verkey))
  t.is(err.indyName, 'CommonInvalidParam3')
  err = await t.throws(indy.abbreviateVerkey(void 0, verkey))
  t.is(err.indyName, 'CommonInvalidParam3')

  err = await t.throws(indy.abbreviateVerkey(did, null))
  t.is(err.indyName, 'CommonInvalidParam4')

  err = await t.throws(indy.abbreviateVerkey('?', verkey))
  t.is(err + '', 'IndyError: CommonInvalidStructure')
  t.is(err.indyCode, 113)
  t.is(err.indyName, 'CommonInvalidStructure')
})
