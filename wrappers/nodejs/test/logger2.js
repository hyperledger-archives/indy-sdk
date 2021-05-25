// NOTE: there are 2 test files for logger so ava can run them in separate nodejs processes
//       why? setLogger and setDefaultLogger cannot be both called in the same process
const test = require('ava')
const indy = require('../')
const IndyError = require('../src/IndyError')

test('setLogger', async function (t) {
  let nEntries = 0
  function logFn (level, target, message, modulePath, file, line) {
    nEntries++
    t.is(typeof level, 'number')
    t.is(typeof target, 'string')
    t.is(typeof message, 'string')
    t.is(typeof modulePath, 'string')
    t.is(typeof file, 'string')
    t.is(typeof line, 'number')
  }
  t.notThrows(function () {
    indy.setLogger(logFn)
  })

  t.is(nEntries, 0)
  await indy.abbreviateVerkey('VsKV7grR1BUE29mG2Fm2kX', 'GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa')
  t.true(nEntries > 0)

  const err = t.throws(function () {
    indy.setLogger(function () {})
  }, { instanceOf: IndyError })
  t.is(err.indyName, 'CommonInvalidState')
})
