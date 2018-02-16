var test = require('ava')
var indy = require('../')

test('hello world', function (t) {
  t.is(indy.hello(), 'Hello indy!')
})

test.cb('abbreviate_verkey', function (t) {
  t.plan(2)
  t.log(indy.abbreviate_verkey(function (err, verkey) {
    t.is(err, 0)
    t.is(verkey, '~HYwqs2vrTc8Tn4uBV7NBTe')
    t.end()
  }))
})
