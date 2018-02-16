var test = require('ava')
var indy = require('../')

test('hello world', function (t) {
  t.is(indy.hello(), 'Hello indy!')
})
