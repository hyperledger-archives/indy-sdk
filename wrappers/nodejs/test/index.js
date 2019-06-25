var test = require('ava')
var indy = require('../')

var did = 'VsKV7grR1BUE29mG2Fm2kX'
var verkey = 'GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa'
var abbrVerkey = '~HYwqs2vrTc8Tn4uBV7NBTe'

test.before('test getCurrentError before any errors', function (t) {
  t.is(indy.capi.getCurrentError(), null)
})

test('wrapper essentials', async function (t) {
  t.is(await indy.abbreviateVerkey(did, verkey), abbrVerkey)

  var err = await t.throwsAsync(indy.abbreviateVerkey())
  t.is(err.message, 'CommonInvalidParam3')
  t.is(err.indyCode, 102)
  t.is(err.indyName, 'CommonInvalidParam3')
  t.is(err.indyMessage, 'Error: Invalid parameter 3\n  Caused by: Invalid pointer has been passed\n')
  t.is(err.indyBacktrace, '')
  t.is(typeof err.indyCurrentErrorJson, 'string')
  t.is(err.indyCurrentErrorJson[0], '{')

  err = t.throws(function () {
    indy.abbreviateVerkey(1, verkey)
  }, Error)
  t.is(err.message, 'abbreviateVerkey expects String or null for did')

  err = t.throws(function () {
    indy.abbreviateVerkey(did, [1, 2, 3])
  }, Error)
  t.is(err.message, 'abbreviateVerkey expects String or null for fullVerkey')

  err = await t.throwsAsync(indy.abbreviateVerkey(null, verkey))
  t.is(err.indyName, 'CommonInvalidParam3')
  err = await t.throwsAsync(indy.abbreviateVerkey(void 0, verkey))
  t.is(err.indyName, 'CommonInvalidParam3')

  err = await t.throwsAsync(indy.abbreviateVerkey(did, null))
  t.is(err.indyName, 'CommonInvalidParam4')

  err = await t.throwsAsync(indy.abbreviateVerkey('?', verkey))
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

      try {
        indy.capi.abbreviateVerkey(t.fail)
        t.fail('should fail b/c not enough arguments were given')
      } catch (err) {
        t.is(err + '', 'Error: abbreviateVerkey expects 3 arguments')
      }
      try {
        indy.capi.abbreviateVerkey('a', 'b', t.fail, 'c')
        t.fail('should fail b/c too many arguments were given')
      } catch (err) {
        t.is(err + '', 'Error: abbreviateVerkey expects 3 arguments')
      }
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
