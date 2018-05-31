const chai = require('chai')
const ffi = require('ffi')
const vcx = require('../dist')
const { stubInitVCX } = require('./helpers')
const assert = chai.assert

const { Connection, StateType, Error, rustAPI, VCXMock, VCXMockMessage } = vcx

describe('A Connection object with ', function () {
  this.timeout(10000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  // connection_create tests
  it('a valid connection that is deleted should error when serialized', async () => {
    const connection = await Connection.create({
      id: '234',
      DIDself: '456',
      DIDremote: '0'
    })
    assert.notEqual(connection._handle, undefined)
    await connection.delete()
    try {
      await connection.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, 1003)
      assert.equal(error.vcxFunction, 'vcx_connection_serialize')
      assert.equal(error.message, 'Invalid Connection Handle')
    }
  })

  it('object with id as param in create should return success', async () => {
    const connection = await Connection.create({ id: '999' })
    assert.notEqual(connection._handle, undefined)
  })

  // connection_connect tests

  it(' a call to connect with connection already created should return success', async () => {
    const connection = await Connection.create({
      id: '234',
      DIDself: '548NLfYrPxtB299RVafcjR',
      DIDremote: '0'
    })
    const inviteDetails = await connection.connect({ sms: true })
    assert(inviteDetails)
  })

  it(' a call to create with no connection created should return unknown error', async () => {
    const connection = new Connection()
    assert.equal(await connection.connect({ sms: true }), Error.INVALID_CONNECTION_HANDLE)
  })

  // connection_get_data tests

  it('a call to serialize where connection exists should return back the connections data', async () => {
    const connection = await Connection.create({ id: '999' })
    assert.notEqual(connection._handle, undefined)
    const data = await connection.serialize()
    assert.notEqual(data, null)
    assert.equal(data.source_id, connection.sourceId)
  })

  it('a call to serialize where connection doesnt exist should throw error', async () => {
    const connection = new Connection()
    try {
      await connection.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, 1003)
      assert.equal(error.vcxFunction, 'vcx_connection_serialize')
      assert.equal(error.message, 'Invalid Connection Handle')
    }
  })

  it('a call to serialize where connection was released should throw error', async () => {
    const connection = await Connection.create({ id: '234' })
    assert.notEqual(connection._handle, undefined)

    const inviteDetails = await connection.connect({ sms: true })
    assert(inviteDetails)
    await connection.updateState()
    assert.equal(await connection.getState(), StateType.OfferSent)
    let data = await connection.serialize()
    assert.notEqual(data, null)
    assert.equal(data.source_id, connection.sourceId)
    assert.equal(await connection.release(), Error.SUCCESS)
    try {
      await connection.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, 1003)
      assert.equal(error.vcxFunction, 'vcx_connection_serialize')
      assert.equal(error.message, 'Invalid Connection Handle')
    }
  })

  // deserialize
  it('a call to deserialize with correct data should return object with same sourceId', async () => {
    const connection1 = await Connection.create({ id: '234' })
    assert.notEqual(connection1._handle, undefined)
    const data = await connection1.serialize()
    const connection2 = await Connection.deserialize(data)
    assert.equal(connection2.sourceId, connection1.sourceId)
    const data2 = await connection2.serialize()
    assert.equal(JSON.stringify(data), JSON.stringify(data2))
  })

  it('a call to deserialize with incorrect data should throw error', async () => {
    try {
      await Connection.deserialize({source_id: 'Invalid'})
    } catch (error) {
      assert.equal(error.vcxCode, 1016)
      assert.equal(error.vcxFunction, 'vcx_connection_deserialize')
      assert.equal(error.message, 'Invalid JSON string')
    }
  })

  it('a call to serialize then deserialize then serialize should have same data', async () => {
    const connection = await Connection.create({ id: '234' })
    assert.notEqual(connection._handle, undefined)

    const inviteDetails = await connection.connect({ sms: true })
    assert(inviteDetails)
    await connection.updateState()
    assert.equal(await connection.getState(), StateType.OfferSent)
    let data = await connection.serialize()
    const connection2 = await Connection.deserialize(data)
    assert.equal(connection2.sourceId, connection.sourceId)
    let data2 = await connection2.serialize()
    assert.equal(JSON.stringify(data2), JSON.stringify(data))
  })

  // connection_getState tests
  it('call to updateState where connection exists should return success', async () => {
    const connection = await Connection.create({ id: '234' })
    assert.notEqual(connection._handle, undefined)
    const inviteDetails = await connection.connect({ sms: true })
    assert(inviteDetails)
    await connection.updateState()
    assert.equal(await connection.getState(), StateType.OfferSent)
  })

  it('call to updateState where no connection exists should have a state value of 0', async () => {
    const connection = new Connection()
    await connection.updateState()
    assert.equal(await connection.getState(), StateType.None)
  })

  it('call to updateState where connection exists but not connected should have a state value of 1', async () => {
    const connection = await Connection.create({ id: 'Unique ID 999' })
    assert.notEqual(connection._handle, undefined)
    await connection.updateState()
    assert.equal(await connection.getState(), StateType.Initialized)
  })

  it(`call to updateState with mocked updateState reply should have a state value of ${StateType.Accepted}`, async () => {
    const connection = await Connection.create({ id: '234' })
    assert.notEqual(connection._handle, undefined)
    const inviteDetails = await connection.connect({ sms: true })
    assert(inviteDetails)
    VCXMock.setVcxMock(VCXMockMessage.GetMessages)
    await connection.updateState()
    assert.equal(await connection.getState(), StateType.Accepted)
  })

  it('call to inviteDetails with abbr returns non-empty string', async () => {
    const connection = await Connection.create({ id: 'Unique ID 999' })
    assert.notEqual(connection._handle, undefined)
    await connection.connect({ sms: true })
    const details = await connection.inviteDetails(true)
    assert.include(details, '"dp":', 'expect to see this in output')
  })

  it('call to inviteDetails without abbr returns non-empty string', async () => {
    const connection = await Connection.create({ id: 'Unique ID 999' })
    assert.notEqual(connection._handle, undefined)
    await connection.connect({ sms: true })
    const details = await connection.inviteDetails(false)
    assert.include(details, '"senderAgencyDetail":', 'expect to see this in output')
  })

  // connection_release tests

  it('call to connection_release where connection exists should return success', async () => {
    const connection = await Connection.create({ id: '234' })
    assert.notEqual(connection._handle, undefined)
    const inviteDetails = await connection.connect({ sms: true })
    assert(inviteDetails)
    assert.equal(await connection.release(), Error.SUCCESS)
    assert.equal(await connection.connect({ sms: true }), Error.INVALID_CONNECTION_HANDLE)
    try {
      await connection.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, 1003)
      assert.equal(error.vcxFunction, 'vcx_connection_serialize')
      assert.equal(error.message, 'Invalid Connection Handle')
    }
  })

  it('call to connection_release with no connection should return unknown error', async () => {
    const connection = new Connection()
    assert.equal(await connection.release(), Error.SUCCESS)
  })

  it('serialize() should return VcxStateType as an integer', async () => {
    const connection = await Connection.create({ id: 'returnVcxTypeInteger' })
    const inviteDetails = await connection.connect({ sms: true })
    assert(inviteDetails)
    const result = await connection.serialize()
    assert.equal(result['state'], StateType.OfferSent)
  })

  const connectionCreateCheckAndDelete = async () => {
    let connection = await Connection.create({ id: '234' })
    assert.notEqual(connection._handle, undefined)
    const inviteDetails = await connection.connect({ sms: true })
    assert(inviteDetails)
    const serialize = rustAPI().vcx_connection_serialize
    const handle = connection._handle
    const data = await connection.serialize()
    assert.notEqual(data, null)
    connection = null
    return {
      handle,
      serialize
    }
  }

  // Fix the GC issue
  it('connection and GC deletes object should return null when get_data is called ', async function () {
    this.timeout(30000)

    const { handle, serialize } = await connectionCreateCheckAndDelete()

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
            0,
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
