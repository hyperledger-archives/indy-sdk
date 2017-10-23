const mysleep = require('sleep');
const chai = require('chai');
const parentDir = require('path');
const currentDir = parentDir.dirname(module.filename);
const Connection = require(parentDir.dirname(currentDir) + '/dist/api/connection').Connection
const StateType = require(parentDir.dirname(currentDir) + '/dist/api/api').StateType
const IRecipientInfo = require(parentDir.dirname(currentDir) + '/dist/api/api').IRecipientInfo
const path = parentDir.dirname(currentDir) + "/lib/libcxs.so";
const cxs = require('../dist/index.js')
const expect = chai.expect;
const assert = chai.assert;

const sleep = (time) => new Promise((res) => setTimeout(res, time))

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

    before(async function() {
        cxs.init_cxs('ENABLE_TEST_MODE')
        await sleep(3000)
    });

    //connection_create tests

    it('valid parameters in create should return success', function () {
        const connection = new Connection(path)
        assert.equal(connection.create({
            id: "234",
            DIDself: "456",
            DIDremote: "0"}), 0)
    })

    it('null param in create should return success', function () {
        const connection = new Connection(path)
        assert.equal(connection.create(null), 0)
    })

// connection_connect tests

    it(' a call to connect with connection already created should return success', function () { 
        const connection = new Connection(path)       
        connection.create("connection_connect tests")
        return connection.connect({sms: true})
    })

    it(' a call to create with no connection created should return unknown error', function () {
        const connection = new Connection(path)
        assert.equal(connection._connect({sms: true}), 1003)
    })


// connection_get_data tests

    it('a call to get_data where connection exists should return back the connections data', function () {
        const connection = new Connection(path)
        connection.create("dog, cat, man")
        const data = connection.getData()
        const jsonData = JSON.parse(data)
        assert.notEqual(data, null)
        assert.equal(jsonData.handle, connection.connectionHandle)
    })

    it('a call to get_data where connection doesnt exist should return a null value', function () {
        const connection = new Connection(path)
        assert.equal(connection.getData(), null)
    })

    it('a call to get_data where connection was released should return a null value', async function () {
        const connection = new Connection(path)
        assert.equal(connection.create("connection_get_data tests"), 0)

        await connection.connect({sms: true})
        
        assert.equal(connection.getState(), StateType.OfferSent)
        assert.notEqual(connection.getData(), null)
        assert.equal(connection.release(), 0)
        assert.equal(connection.getData(), null)
    })

    // connection_getState tests

    it('call to getState where connection exists should return success', async function () {
        const connection = new Connection(path)
        connection.create("connection_getState tests")
        await connection.connect({sms: true})
        assert.equal(connection.getState(),StateType.OfferSent)
    })

    it('call to getState where no connection exists should have a state value of 0', function () {
        const connection = new Connection(path)
        assert.equal(connection.getState(),StateType.None)
    })


    it('call to get_state where connection exists but not connected should have a state value of 1', function () {
        const connection = new Connection(path)
        connection.create("info2")
        return waitFor(() => connection.getState() === StateType.Initialized)
    })


// connection_release tests

    it('call to connection_release where connection exists should return success', async function () {
        const connection = new Connection(path)
        connection.create("connection_release tests")
        await connection.connect({sms: true})
        assert.equal(connection.release(), 0)
        assert.equal(connection._connect({sms: true}), 1003)
        assert.equal(connection.getData(), null)
    })

    it('call to connection_release with no connection should return unknown error', function () {
        const connection = new Connection(path)
        assert.equal(connection.release(), 1003)
    })

    it('connection and GC deletes object should return null whet get_data is called ', function () {
        this.timeout(30000)
        const connection = new Connection(path)
        connection.create("msg")
        connection._connect({sms: true})
        const getData = connection.RUST_API.cxs_connection_get_data
        const handle = connection.connectionHandle
        assert.notEqual(connection.getData(handle), null)

        delete connection
        global.gc()

        // this will timeout if condition is never met
        // get_data will return "" because the connection object was released
        return waitFor(() => !getData(handle))
    })

})
