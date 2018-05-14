var test = require('ava')
var indy = require('../')

var did = 'VsKV7grR1BUE29mG2Fm2kX'
var verkey = 'GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa'
var abbrVerkey = '~HYwqs2vrTc8Tn4uBV7NBTe'

test('wrapper essentials', async function (t) {
  t.is(await indy.abbreviateVerkey(did, verkey), abbrVerkey)

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

test.cb('wrapper capi', function (t) {
  indy.capi.abbreviateVerkey(did, verkey, function (err, data) {
    t.falsy(err)
    t.is(data, abbrVerkey)

    indy.capi.abbreviateVerkey('?', verkey, function (err) {
      t.is(err, 113)
      t.end()
    })
  })
})

test.cb('wrapper callbacks', function (t) {
  indy.abbreviateVerkey(did, verkey, function (err, data) {
    t.falsy(err)
    t.is(data, abbrVerkey)

    indy.abbreviateVerkey('?', verkey, function (err) {
      t.is(err + '', 'IndyError: CommonInvalidStructure')
      t.is(err.indyCode, 113)
      t.is(err.indyName, 'CommonInvalidStructure')
      t.end()
    })
  })
})
