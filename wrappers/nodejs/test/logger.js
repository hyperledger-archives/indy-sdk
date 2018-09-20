var test = require('ava')
var indy = require('../')
var IndyError = require('../src/IndyError')

test('setDefaultLogger', async function (t) {
  t.notThrows(function () {
    indy.setDefaultLogger('trace')
  })

  var err = t.throws(function () {
    indy.setDefaultLogger('foo')
  }, IndyError)
  t.is(err.indyName, 'CommonInvalidState')
})
