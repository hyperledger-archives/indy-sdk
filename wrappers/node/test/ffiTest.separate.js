require('chai')
require('fs-extra')
const ffi = require('ffi')
const vcx = require('../dist')
const assert = require('assert')
const { VCXRuntime } = vcx

describe('call to vcx_init with provided path', function () {
  this.timeout(10000)

  let path = '/usr/lib/libvcx.so'
  const run = new VCXRuntime({ basepath: path })

  it('should return 1004', async () => {
    let result = null
    result = await new Promise((resolve, reject) => {
      result = run.ffi.vcx_init(
              0,
              'garbage',
              ffi.Callback('void', ['uint32', 'uint32'],
                  (xhandle, err) => {
                    resolve(err)
                  }))
      if (result !== 0) {
        resolve(result)
      }
    })
    assert.equal(result, 1004)
  })

  // null path given should always error
  it('should return 1004', async () => {
    let result = await new Promise((resolve, reject) =>
            resolve(run.ffi.vcx_init(
                0,
                null,
                ffi.Callback('void', ['uint32', 'uint32'],
                    (xhandle, err) => ({})))
            )
    )
    assert.equal(result, 1004)
  })
})

// these tests were created to only test that the ffi could be called with each function

describe('Using the vcx ffi directly ', async () => {
  let path = '/usr/lib/libvcx.so'
  const run = new VCXRuntime({ basepath: path })

  it('a call to vcx_connection_create should return 0', async () => {
    let result = null
    result = await new Promise((resolve, reject) => {
      result = run.ffi.vcx_connection_create(
          0,
          '1',
          ffi.Callback('void', ['uint32', 'uint32', 'uint32'],
              (xhandle, err, connectionHandle) => {
                if (err) {
                  reject(err)
                }
              }))
      resolve(result)
    })
    assert.equal(result, 0)
  })

  it('a to vcx_connection_connect without the ability to connect should return 1', async () => {
    let connectResult = null
    connectResult = await new Promise((resolve, reject) => {
      connectResult = run.ffi.vcx_connection_connect(
              0,
              1,
              JSON.stringify({connection_type: 'sms', phone: 123}),
              ffi.Callback('void', ['uint32', 'uint32'],
                  (xhandle, err) => {
                    resolve(err)
                  }))
      if (connectResult !== 0) {
        resolve(connectResult)
      }
    })
    assert.equal(connectResult, 1003)
  })

  it('a call to vcx_connection_serialize should return 0', async function () {
    const result = await new Promise(function (resolve, reject) {
      const rc = run.ffi.vcx_connection_serialize(
        0,
        1,
        ffi.Callback('void', ['uint32', 'uint32', 'string'], (handle, err, data) => {
          if (err) {
            reject(err)
            return
          } else if (!data) {
            data = null
          }
          resolve(data)
        }))
      if (rc) {
        resolve(null)
      }
    })
    assert.equal(result, null)
  })

  it('a call to vcx_connection_get_state should return 0', async () => {
    const state = await new Promise((resolve, reject) => {
      const rc = run.ffi.vcx_connection_update_state(
          0,
          1,
          ffi.Callback('void', ['uint32', 'uint32', 'uint32'], (handle, err, state) => {
            if (err) {
              reject(err)
              return
            }
            resolve(state)
          }))
      if (rc) {
        resolve(0)
      }
    })
    assert.equal(state, 0)
  })

  it('a call to vcx_connection_release without ability to release should return 1', function () {
    assert.equal(run.ffi.vcx_connection_release(2), 0)
  })
})
