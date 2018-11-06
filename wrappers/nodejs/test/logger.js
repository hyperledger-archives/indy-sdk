// NOTE: there are 2 test files for logger so ava can run them in separate nodejs processes
//       why? setLogger and setDefaultLogger cannot be both called in the same process
var test = require('ava')
var indy = require('../')
var IndyError = require('../src/IndyError')

test('setDefaultLogger', function (t) {
  t.notThrows(function () {
    indy.setDefaultLogger('trace')
  })

  var err = t.throws(function () {
    indy.setDefaultLogger('foo')
  }, IndyError)
  t.is(err.indyName, 'CommonInvalidState')
})
