const test = require('ava')
const indy = require('../')

test('setRuntimeConfig', async function (t) {
  t.notThrows(function () {
    indy.setRuntimeConfig({ crypto_thread_pool_size: 4 })
  })
  t.throws(function () {
    indy.setRuntimeConfig({ crypto_thread_pool_size: 'bad-value' })
  })
  t.notThrows(function () {
    indy.setRuntimeConfig({ crypto_thread_pool_size: 1 })
  })
})
