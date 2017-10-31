const chai = require('chai')
const parentDir = require('path')
const currentDir = parentDir.dirname(module.filename)
const Connection = require(parentDir.dirname(currentDir) + '/dist/api/connection').Connection
const StateType = require(parentDir.dirname(currentDir) + '/dist/api/api').StateType
const path = parentDir.dirname(currentDir) + '/lib/libcxs.so'
const cxs = require('../dist/index.js')
const assert = chai.assert



const sleep = (time) => new Promise((resolve) => setTimeout(resolve, time))

// const getDataPromise = (getData, handle) => {
//
//     const data = new Promise( function (resolve, reject) {
//         var callback = getCallback('void', ['uint32', 'uint32', 'string'],
//             function(handle, err, data) {
//                 console.log("here")
//                 if (err) {
//                     reject(err)
//                     return
//                 }
//                 resolve(data)
//             })
//         getData(
//             handle,
//             callback
//             )
//     })
//     process.on('exit', function() {
//         callback
//     })
//
//     return data
// }

// const waitFor2 = async (getData) => {
//     var result =getDataPromise(getData)
//     await sleep(1000)
//     result.then(function(data) {
//         if (!data) {
//             return getDataPromise(getData)
//         } else {
//             return data
//         }
//     })
//
// }

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

  before(async function () {
    cxs.init_cxs('ENABLE_TEST_MODE')
    await sleep(3000)
  })

    // connection_create tests

  it('valid parameters in create should return success', function () {
    const connection = new Connection(path)
    assert.equal(connection.create({
      id: '234',
      DIDself: '456',
      DIDremote: '0'
    }), 0)
  })

  it('object with id as param in create should return success', function () {
    const connection = new Connection(path)
    assert.equal(connection.create({ id: '999' }), 0)
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

  it(' a call to create with no connection created should return unknown error', function () {
    const connection = new Connection(path)
    assert.equal(connection._connect({ sms: true }), 1003)
  })

    // connection_get_data tests

  it('a call to get_data where connection exists should return back the connections data', function () {
    const connection = new Connection(path)
    connection.create({ id: '234' })
    var result = connection.getData()
    result.then(function(data) {
        assert.notEqual(data, "")
        assert.equal(data.handle, connection.connectionHandle)
    })
  })

  it('a call to get_data where connection doesnt exist should return an empty string', function () {
    const connection = new Connection(path)
    var result = connection.getData()
    result.then(function(data) {
        assert.equal(data, "")
    })
  })

  it('a call to get_data where connection was released should return an empty string', async function () {
    const connection = new Connection(path)
    assert.equal(connection.create({ id: '234' }), 0)

    await connection.connect({ sms: true })

    assert.equal(connection.getState(), StateType.OfferSent)
    var result = connection.getData()
    result.then(function(data) {
        assert.notEqual(data, "")
        assert.equal(data.handle, connection.connectionHandle)
    })
    assert.equal(connection.release(), 0)
    result = connection.getData()
    result.then(function(data) {
        assert.notEqual(data, "")
    })
  })

    // connection_getState tests

  it('call to getState where connection exists should return success', async function () {
    const connection = new Connection(path)
    connection.create({ id: '234' })
    await connection.connect({ sms: true })
    assert.equal(connection.getState(), StateType.OfferSent)
  })

  it('call to getState where no connection exists should have a state value of 0', function () {
    const connection = new Connection(path)
    assert.equal(connection.getState(), StateType.None)
  })

  it('call to get_state where connection exists but not connected should have a state value of 1', function () {
    const connection = new Connection(path)
    connection.create({ id: '234' })
    return waitFor(() => connection.getState() === StateType.Initialized)
  })

    // connection_release tests

  it('call to connection_release where connection exists should return success', async function () {
    const connection = new Connection(path)
    connection.create({ id: '234' })
    await connection.connect({ sms: true })
    assert.equal(connection.release(), 0)
    assert.equal(connection._connect({ sms: true }), 1003)
    var result = connection.getData()
    result.then(function(data) {
        assert.equal(data, "")
    })
  })

  it('call to connection_release with no connection should return unknown error', function () {
    const connection = new Connection(path)
    assert.equal(connection.release(), 1003)
  })

  it('getData() should return CxsStateType as an integer', async function () {
    const connection = new Connection(path)
    connection.create({ id: '234' })
    await connection.connect({ sms: true })
    var result = connection.getData()
    result.then(function(data) {
        assert.equal(data['state'], 2)
    })
  })

  it('connection and GC deletes object should return empty string when get_data is called ', async function () {

      this.timeout(30000)
      var connection = new Connection(path)
      connection.create({ id: '234' })
      connection._connect({ sms: true })
      const getData = connection.RUST_API.cxs_connection_serialize
      const handle = connection.connectionHandle
      assert.notEqual(connection.getData(handle), null)

      connection = null
      global.gc()

      var isComplete = false
      const Callback = require('ffi').Callback

        while(!isComplete){
          var callback = null
          const data = await new Promise( function (resolve, reject) {
              callback = Callback('void', ['uint32', 'uint32', 'string'],
                  function(handle, err, data) {
                      if (err) {
                          reject(err)
                          return
                      }
                      resolve(data)
                  })
              getData(
                  handle,
                  callback
              )
          })
          if(!data){
              isComplete = true;
          }
          process.on('exit', function() {
              callback
          })
      }


      // this will timeout if condition is never met
      // get_data will return "" because the connection object was released

      // return waitFor(() => !getDataPromise(getData, handle))
      return isComplete
  })

})
