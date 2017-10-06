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

//connection_create tests

describe('call to connection_create with good params ', function() {
    var connection = new Connection(path)
    it('should return success', function () {
        assert.equal(connection.create("dog, cat, man"), 0)
    })
});

describe('call to connection_create with null connection_handle ', function() {
    var connection = new Connection(path)
    it('should return unknown error', function () {
        assert.equal(connection.create(null), 1001)
    })
});

describe('call to connection_create with invalid type ', function() {
    var connection = new Connection(path)
    it('should throw TypeError', function () {
        expect(function (){connection.create(3)}).to.throw(TypeError)
    })
});


// connection_connect tests

describe('call to cxs_connection_connect with connection already created ', function() {
    var connection = new Connection(path)
    it('should return success', function () {
        connection.create("info")
        assert.equal(connection.connect(), 0)
    })
});

describe('call to cxs_connection_connect with no connection created ', function() {
    var connection = new Connection(path)
    it('should return unknown error', function () {
        assert.equal(connection.connect(), 1001)
    })
});


// connection_get_data tests

describe('call to connection_get_data with connection already created ', function() {
    var connection = new Connection(path)
    it('should return back the connections data', function () {
        connection.create("dog, cat, man")
        var data = connection.get_data()
        var jsonData = JSON.parse(data)
        assert.notEqual(data, "")
        assert.equal(jsonData.handle, connection.connectionHandle)
    })
});

describe('call to connection_get_data with no previous connection ', function() {
    var connection = new Connection(path)
    it('should return an empty string', function () {
        assert.equal(connection.get_data(), "")
    })
});

describe('call to get_data with with connection released ', function() {
    var connection = new Connection(path)
    it('should return an empty string', function () {
        connection.create("info")
        assert.equal(connection.connect(), 0)
        var data = connection.get_data()
        assert.notEqual(data, "")
        assert.equal(connection.release(), 0)
        data = connection.get_data()
        assert.equal(data, "")
    })
});

// connection_get_state tests

describe('call to get_state with connection already created ', function() {
    var connection = new Connection(path)
    it('should return success', function () {
        connection.create("info")
        connection.connect()
        assert.equal(connection.get_state(), 0)
        assert.equal(connection.state, 2)
    })
});

describe('call to get_sate with no connection created ', function() {
    var connection = new Connection(path)
    it('should have a state value of 0', function () {
        assert.equal(connection.get_state(), 0)
        assert.equal(connection.state, 0)
    })
});


describe('call to get_sate with connection created but not connected ', function() {
    var connection = new Connection(path)
    it('should have a state value of 1', function () {
        connection.create("info")
        assert.equal(connection.get_state(), 0)
        assert.equal(connection.state, 1)
    })
});


// connection_release tests

describe('call to connection_release with connection already created ', function() {
    var connection = new Connection(path)
    it('should return success', function () {
        connection.create("info")
        assert.equal(connection.connect(), 0)
        assert.equal(connection.release(), 0)
        assert.equal(connection.connect(), 1001)
        assert.equal(connection.get_data(), "")
    })
});

describe('call to connection_release with no connection created ', function() {
    var connection = new Connection(path)
    it('should return unknown error', function () {
        assert.equal(connection.release(), 1001)
    })
});


// connection_list_state
describe('call to connection_release ', function() {
    var connection = new Connection(path)
    it('should return success', function () {
        assert.equal(connection.list_state(), 0)
    })
});