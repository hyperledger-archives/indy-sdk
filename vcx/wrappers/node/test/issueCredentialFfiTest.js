const assert = require('chai').assert
const Callback = require('ffi').Callback
const { VCXRuntime } = require('../dist')

describe('The wrapper', async function () {
  var callback = null
  it('can call the ffi directly', async function () {
    var ffi = new VCXRuntime().ffi
    callback = Callback('void', ['uint32', 'uint32', 'uint32'],
                      function (handle, err, data) {
                        /* tslint:disable */
                        console.log('commandHandle: ' + handle)
                        /* tslint:enable */
                      })
    const res = await ffi.vcx_issuer_create_credential(0, 'sourceId', 32, 'regularstring', 'regularstring', 'regularstring', callback)
    assert.equal(res, 0)
  })
})
