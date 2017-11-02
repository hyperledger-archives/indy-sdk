const chai = require('chai')
const parentDir = require('path')
const currentDir = parentDir.dirname(module.filename)
const api = require(parentDir.dirname(currentDir) + '/dist/api/api.js')
const StateType = api.StateType
const Error = api.Error
const Connection = require(parentDir.dirname(currentDir) + '/dist/api/connection').Connection
const path = parentDir.dirname(currentDir) + '/lib/libcxs.so'
const cxs = require('../dist/index.js')
const assert = chai.assert
const ffi = require('ffi')

const sleep = (time) => new Promise((resolve) => setTimeout(resolve, time))

const waitFor = async (predicate) => {
  if (!predicate()) {
    await sleep(1000)
    return waitFor(predicate)
  }
  return predicate()
}

// console.log(release(handle)) // tslint:disable-line
describe('A Connection object with ', function () {
  this.timeout(10000)

  before(async () => {
    await cxs.init_cxs('ENABLE_TEST_MODE')
    await sleep(3000)
  })

    // connection_create tests

  it('valid parameters in create should return success', async () => {
    const connection = new Connection(path)
    await connection.create({
      id: '234',
      DIDself: '456',
      DIDremote: '0'
    })
    assert.notEqual(connection.connectionHandle, undefined)
  })

  it('object with id as param in create should return success', async () => {
    const connection = new Connection(path)
    await connection.create({ id: '999' })
    assert.notEqual(connection.connectionHandle, undefined)
  })

    // connection_connect tests

  it(' a call to connect with connection already created should return success', async () => {
    const connection = new Connection(path)
    await connection.create({
      id: '234',
      DIDself: '548NLfYrPxtB299RVafcjR',
      DIDremote: '0'
    })
    await connection.connect({ sms: true })
  })

  it(' a call to create with no connection created should return unknown error', async () => {
    const connection = new Connection(path)
    assert.equal(await connection._connect({ sms: true }), Error.INVALID_CONNECTION_HANDLE)
  })

    // connection_get_data tests

  it('a call to serialize where connection exists should return back the connections data', async () => {
    const connection = new Connection(path)
    await connection.create({ id: '999' })
    assert.notEqual(connection.connectionHandle, undefined)
    const data = await connection.serialize()
    assert.notEqual(data, null)
    assert.equal(data.handle, connection.connectionHandle)
  })

  it('a call to serialize where connection doesnt exist should return null', async () => {
    const connection = new Connection(path)
    const data = await connection.serialize()
    assert.equal(data, null)
  })

  it('a call to serialize where connection was released should return null', async () => {
    const connection = new Connection(path)
    await connection.create({ id: '234' })
    assert.notEqual(connection.connectionHandle, undefined)

    await connection.connect({ sms: true })
    await connection.updateState()
    assert.equal(connection.state, StateType.OfferSent)
    let data = await connection.serialize()
    assert.notEqual(data, null)
    assert.equal(data.handle, connection.connectionHandle)
    assert.equal(connection.release(), Error.SUCCESS)
    data = await connection.serialize()
    assert.equal(data, null)
  })

  // deserialize
  it('a call to deserialize with correct data should return the connection handle', async () => {
    const connection1 = new Connection(path)
    await connection1.create({ id: '234' })
    assert.notEqual(connection1.connectionHandle, undefined)
    let data = await connection1.serialize()

    const connection2 = new Connection(path)
    console.log(JSON.toString(data))
    await connection2.deserialize(JSON.stringify(data))
    assert.equal(connection2.connectionHandle, connection1.connectionHandle)
  })

  it('a call to deserialize with incorrect data should throw error', async () => {
    const connection = new Connection(path)
    try {
      await connection.deserialize(JSON.stringify('fail'))
    } catch (error) {
      assert.equal(error.toString(), 'Error: cxs_connection_deserialize -> 1001')
    }
    assert.equal(connection.connectionHandle, undefined)
  })

    // connection_getState tests
  it('call to updateState where connection exists should return success', async () => {
    const connection = new Connection(path)
    await connection.create({ id: '234' })
    assert.notEqual(connection.connectionHandle, undefined)
    await connection.connect({ sms: true })
    await connection.updateState()
    assert.equal(connection.state, StateType.OfferSent)
  })

  it('call to updateState where no connection exists should have a state value of 0', async () => {
    const connection = new Connection(path)
    await connection.updateState()
    assert.equal(connection.state, StateType.None)
  })

  it('call to updateState where connection exists but not connected should have a state value of 1', async () => {
    const connection = new Connection(path)
    await connection.create({ id: 'Unique ID 999' })
    assert.notEqual(connection.connectionHandle, undefined)
    await connection.updateState()
    assert.equal(connection.state, StateType.Initialized)
  })

    // connection_release tests

  it('call to connection_release where connection exists should return success', async () => {
    const connection = new Connection(path)
    await connection.create({ id: '234' })
    assert.notEqual(connection.connectionHandle, undefined)
    await connection.connect({ sms: true })
    assert.equal(connection.release(), Error.SUCCESS)
    assert.equal(await connection._connect({ sms: true }), Error.INVALID_CONNECTION_HANDLE)
    const result = await connection.serialize()
    assert.equal(result, null)
  })

  it('call to connection_release with no connection should return unknown error', () => {
    const connection = new Connection(path)
    assert.equal(connection.release(), Error.INVALID_CONNECTION_HANDLE)
  })

  it('serialize() should return CxsStateType as an integer', async () => {
    const connection = new Connection(path)
    await connection.create({ id: '234' })
    await connection.connect({ sms: true })
    const result = await connection.serialize()
    assert.equal(result['state'], StateType.OfferSent)
  })

  it('connection and GC deletes object should return null when get_data is called ', async () => {
    this.timeout(30000)
    let connection = new Connection(path)
    await connection.create({ id: '234' })
    assert.notEqual(connection.connectionHandle, undefined)
    await connection._connect({ sms: true })
    const serialize = connection.RUST_API.cxs_connection_serialize
    const handle = connection.connectionHandle
    const data = await connection.serialize()
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
        const rc = serialize(
          handle,
          callback
        )

        if (rc === Error.INVALID_CONNECTION_HANDLE) {
          resolve(null)
        }
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
