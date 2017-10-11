var chai = require('chai');
var parentDir = require('path');
var currentDir = parentDir.dirname(module.filename);
var Connection = require(parentDir.dirname(currentDir) + '/dist/api/connection').Connection
var path = parentDir.dirname(currentDir) + "/lib/libcxs.so";
var expect = chai.expect;
var assert = chai.assert;

// console.log(release(handle)) // tslint:disable-line


describe('A Connection object with ', function () {
    let connection;
    beforeEach(function() {
        connection = new Connection(path)
    });

    //connection_create tests

    it('valid parameters in create should return success', function () {
        assert.equal(connection.create("dog, cat, man"), 0)
    })

    it('null param in create should return unknown error', function () {
        assert.equal(connection.create(null), 1001)
    })

    it('invalid type as param to create throws TypeError', function () {
        expect(function (){connection.create(3)}).to.throw(TypeError)
    })


// connection_connect tests

    it(' a call to connect with connection already created should return success', function () {
        connection.create("info")
        assert.equal(connection.connect(), 0)
    })

    it(' a call to create with no connection created should return unknown error', function () {
        assert.equal(connection.connect(), 1003)
    })


// connection_get_data tests

    it('a call to get_data where connection exists should return back the connections data', function () {
        connection.create("dog, cat, man")
        var data = connection.getData()
        var jsonData = JSON.parse(data)
        assert.notEqual(data, null)
        assert.equal(jsonData.handle, connection.connectionHandle)
    })

    it('a call to get_data where connection doesnt exist should return a null value', function () {
        assert.equal(connection.getData(), null)
    })

    it('a call to get_data where connection was released should return a null value', function () {
        connection.create("info")
        assert.equal(connection.connect(), 0)
        var data = connection.getData()
        assert.notEqual(data, null)
        assert.equal(connection.release(), 0)
        data = connection.getData()
        assert.equal(data, null)
    })

// connection_getState tests

    it('call to getState where connection exists should return success', function () {
        connection.create("info")
        connection.connect()
        assert.equal(connection.getState(), 0)
        assert.equal(connection.state, 2)
    })

    it('call to getState where no connection exists should have a state value of 0', function () {
        assert.equal(connection.getState(), 0)
        assert.equal(connection.state, 0)
    })


    it('call to get_state where connection exists but not connected should have a state value of 1', function () {
        var connection = new Connection(path)

        connection.create("info2")
        assert.equal(connection.getState(), 0)
        assert.equal(connection.state, 1)
    })


// connection_release tests

    it('call to connection_release where connection exists should return success', function () {
        connection.create("info")
        assert.equal(connection.connect(), 0)
        assert.equal(connection.release(), 0)
        assert.equal(connection.connect(), 1003)
        assert.equal(connection.getData(), null)
    })

    it('call to connection_release with no connection should return unknown error', function () {
        assert.equal(connection.release(), 1003)
    })

    const sleep = (time) => new Promise((res) => setTimeout(res, time))

    const waitFor = async (predicate) => {
        if (!predicate()) {
            await sleep(1000)
            return waitFor(predicate)
        }
        return predicate()
    }

    it('connection and GC deletes object should return null whet get_data is called ', function () {
        const connection = new Connection(path)
        connection.create("msg")
        connection.connect()
        const getData = connection.RUST_API.cxs_connection_get_data
        const handle = connection.connectionHandle
        assert.notEqual(connection.getData(handle), null)

        this.timeout(30000)
        delete connection
        global.gc()

        // this will timeout if condition is never met
        // get_data will return "" because the connection object was released
        return waitFor(() => !getData(handle))
    })

})
