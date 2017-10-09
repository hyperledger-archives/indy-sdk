var chai = require('chai');
var fs = require('fs-extra');
var ref = require('ref')
var Struct = require('ref-struct')
var Connection = require('../dist/api/connection').Connection
var parentDir = require('path');
var currentDir = parentDir.dirname(module.filename);
var path = parentDir.dirname(currentDir) + "/lib/libcxs.so";
var expect = chai.expect;
var assert = chai.assert;

// console.log(release(handle)) // tslint:disable-line

const sleep = (time) => new Promise((res) => setTimeout(res, time))

const waitFor = async (predicate) => {
    if (!predicate()) {
        await sleep(1000)
        return waitFor(predicate)
    }
    return predicate()
}


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
        assert.equal(connection.connect(), 1001)
    })


// connection_get_data tests

    it('a call to get_data where connection exists should return back the connections data', function () {
        connection.create("dog, cat, man")
        var data = connection.get_data()
        var jsonData = JSON.parse(data)
        assert.notEqual(data, "")
        assert.equal(jsonData.handle, connection.connectionHandle)
    })

    it('a call to get_data where connection doesnt exist should return an empty string', function () {
        assert.equal(connection.get_data(), "")
    })

    it('a call to get_data where connection with connection released should return an empty string', function () {
        connection.create("info")
        assert.equal(connection.connect(), 0)
        var data = connection.get_data()
        assert.notEqual(data, "")
        assert.equal(connection.release(), 0)
        data = connection.get_data()
        assert.equal(data, "")
    })

// connection_get_state tests

    it('call to get_state where connection exists should return success', function () {
        connection.create("info")
        connection.connect()
        assert.equal(connection.get_state(), 0)
        assert.equal(connection.state, 2)
    })

    it('call to get_state where no connection exists should have a state value of 0', function () {
        assert.equal(connection.get_state(), 0)
        assert.equal(connection.state, 0)
    })


    it('call to get_sate where connection exists but not connected should have a state value of 1', function () {
        connection.create("info")
        assert.equal(connection.get_state(), 0)
        assert.equal(connection.state, 1)
    })


// connection_release tests

    it('call to connection_release where connection exists should return success', function () {
        connection.create("info")
        assert.equal(connection.connect(), 0)
        assert.equal(connection.release(), 0)
        assert.equal(connection.connect(), 1001)
        assert.equal(connection.get_data(), "")
    })


    it('call to connection_release with no connection should return unknown error', function () {
        assert.equal(connection.release(), 1001)
    })


    it('connection and GC deletes object should return empty whet get_data is called ', function () {
        const connection = new Connection(path)
        connection.create("msg")
        connection.connect()
        const get_data = connection.RUST_API.cxs_connection_get_data
        const handle = connection.connectionHandle
        assert.notEqual(get_data(handle), "")

        this.timeout(30000)
        delete connection
        global.gc()

        // this will timeout if condition is never met
        // get_data will return "" because the connection object was released
        return waitFor(() => !get_data(handle))
    })

})