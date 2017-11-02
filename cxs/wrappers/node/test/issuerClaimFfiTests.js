var assert = require('chai').assert
var Callback = require('ffi').Callback
var CXSRuntime = require('../dist/index').CXSRuntime
var CXSRuntimeConfig = require('../dist/index').CXSRuntimeConfig

describe('The wrapper', async function () {
  var callback = null
  it('can call the ffi directly', async function () {
    var ffi = new CXSRuntime(new CXSRuntimeConfig(null))._ffi
    callback = Callback('void', ['uint32', 'uint32', 'uint32'],
                      function (handle, err, data) {
                        /* tslint:disable */
                        console.log('commandHandle: ' + handle)
                        /* tslint:enable */
                      })
    const res = await ffi.cxs_issuer_create_claim(0, 'sourceId', 32, 'regularstring', callback)
    assert.equal(res, 0)
  })
})
