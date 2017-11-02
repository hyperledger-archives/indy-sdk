const chai = require('chai')
const parentDir = require('path')
const currentDir = parentDir.dirname(module.filename)
const Connection = require(parentDir.dirname(currentDir) + '/dist/api/connection').Connection
const StateType = require(parentDir.dirname(currentDir) + '/dist/api/api').StateType
const path = parentDir.dirname(currentDir) + '/lib/libcxs.so'
const cxs = require('../dist/index.js')
const assert = chai.assert
const ffi = require('ffi')

const sleep = (time) => new Promise((resolve) => setTimeout(resolve, time))

const waitFor = async (predicate) => {
  const ret = await predicate()
  if (!ret) {
    await sleep(1000)
    return waitFor(predicate)
  }
  return ret
}

// console.log(release(handle)) // tslint:disable-line
describe('A Connection object with ', function () {
  this.timeout(10000)

  before(async function () {
    cxs.init_cxs('ENABLE_TEST_MODE')
    await sleep(3000)
  })

    // connection_create tests

  it('valid parameters in create should return success', async function () {
    const connection = new Connection(path)
    const res = await connection.create({
      id: '234',
      DIDself: '456',
      DIDremote: '0'
    })
    assert.equal(res, 0)
  })

  it('object with id as param in create should return success', async function () {
    const connection = new Connection(path)
    const res = await connection.create({ id: '999' })
    assert.equal(res, 0)
  })

    // connection_connect tests

  it(' a call to connect with connection already created should return success', async function () {
    const connection = new Connection(path)
    connection.create({
      id: '234',
      DIDself: '548NLfYrPxtB299RVafcjR',
      DIDremote: '0'
    })

    return connection.connect({ sms: true })
  })

  it(' a call to create with no connection created should return unknown error', async function () {
    const connection = new Connection(path)
    assert.equal(await connection._connect({ sms: true }), 1003)
  })

    // connection_get_data tests

  it('a call to get_data where connection exists should return back the connections data', async function () {
    const connection = new Connection(path)
    // TODO we are not awaiting on this create, which
    // is an async function that has a promise behind it.
    connection.create({ id: '234' })
    const data = await connection.getData()
    assert.notEqual(data, null)
    assert.equal(data.handle, connection.connectionHandle)
  })

  it('a call to get_data where connection doesnt exist should return null', async function () {
    const connection = new Connection(path)
    const data = await connection.getData()
    assert.equal(data, null)
  })

  it('a call to get_data where connection was released should return null', async function () {
    const connection = new Connection(path)
    assert.equal(await connection.create({ id: '234' }), 0)

    await connection.connect({ sms: true })

    assert.equal(await connection.getState(), StateType.OfferSent)
    let data = await connection.getData()
    assert.notEqual(data, null)
    assert.equal(data.handle, connection.connectionHandle)
    assert.equal(await connection.release(), 0)
    data = await connection.getData()
    assert.equal(data, null)
  })

    // connection_getState tests

  it('call to getState where connection exists should return success', async function () {
    const connection = new Connection(path)
    await connection.create({ id: '234' })
    await connection.connect({ sms: true })
    assert.equal(await connection.getState(), StateType.OfferSent)
  })

  it('call to getState where no connection exists should have a state value of 0', async function () {
    const connection = new Connection(path)
    assert.equal(await connection.getState(), StateType.None)
  })

  it('call to get_state where connection exists but not connected should have a state value of 1', async function () {
    const connection = new Connection(path)
    await connection.create({ id: '234' })
    return waitFor(async () => (await connection.getState()) === StateType.Initialized)
  })

    // connection_release tests

  it('call to connection_release where connection exists should return success', async function () {
    const connection = new Connection(path)
    connection.create({ id: '234' })
    await connection.connect({ sms: true })
    assert.equal(await connection.release(), 0)
    // TODO This should be the connect call, not
    // private call.
    assert.equal(connection._connect({ sms: true }), 1003)
    const result = await connection.getData()
    assert.equal(result, null)
  })

  it('call to connection_release with no connection should return unknown error', async function () {
    const connection = new Connection(path)
    assert.equal(await connection.release(), 1003)
  })

  it('getData() should return CxsStateType as an integer', async function () {
    const connection = new Connection(path)
    await connection.create({ id: 'returnCxsTypeInteger' })
    await sleep(3000)
    const data2 = await connection.getData()
    assert.equal(data2['state'], 1)
    await connection.connect({ sms: true })
    const result = await connection.getData()
    assert.equal(result['state'], 2)
  })

  it('connection and GC deletes object should return null when get_data is called ', async function () {
    this.timeout(30000)
    let connection = new Connection(path)
    await connection.create({ id: 'GarbageCollector' })
    connection._connect({ sms: true })
    const getData = connection.RUST_API.cxs_connection_serialize
    const handle = connection.connectionHandle
    const data = await connection.getData()
    assert.notEqual(data, null)

    connection = null
    global.gc()

    let isComplete = false
    //  hold on to callbacks so it doesn't become garbage collected
    const callbacks = []

    while (!isComplete) {
      const data = await new Promise(function (resolve, reject) {
        const callback = ffi.Callback('void', ['uint32', 'uint32', 'string'],
          function (handle, err, data) {
            if (err) {
              reject(err)
              return
            }
            resolve(data)
          })
        callbacks.push(callback)
        getData(
          handle,
          callback
        )
      })
      if (!data) {
        isComplete = true
      }
    }

      // this will timeout if condition is never met
      // get_data will return "" because the connection object was released
    return isComplete
  })
})
