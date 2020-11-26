// NOTE: there are 2 test files for logger so ava can run them in separate nodejs processes
//       why? setLogger and setDefaultLogger cannot be both called in the same process
const test = require('ava')
const indy = require('../')
const IndyError = require('../src/IndyError')

test('setDefaultLogger', function (t) {
  t.notThrows(function () {
    indy.setDefaultLogger('trace')
  })

  const err = t.throws(function () {
    indy.setDefaultLogger('foo')
  }, { instanceOf: IndyError })
  t.is(err.indyName, 'CommonInvalidState')
})
