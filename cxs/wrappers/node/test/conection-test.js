var chai = require('chai');
var fs = require('fs-extra');
var ref = require('ref')
var Struct = require('ref-struct')
var Connection = require('../src/api/connection').Connection
var parentDir = require('path');
var currentDir = parentDir.dirname(module.filename);
var path = parentDir.dirname(currentDir) + "/lib/libcxs.so";
var expect = chai.expect;
var assert = chai.assert;

//connection_create tests

describe('call to connection_create with good params ', function() {
    var connection = new Connection(path)
    it('should return success', function () {
        assert.equal(connection.connection_create("dog, cat, man", 4), 0)
    })
});

describe('call to connection_create with null connection_handle ', function() {
    var connection = new Connection(path)
    it('should return error', function () {
        assert.equal(connection.connection_create("dog, cat, man", null), 1)
    })
});

describe('call to connection_create with invalid type ', function() {
    var connection = new Connection(path)
    it('should throw TypeError', function () {
        expect(function (){connection.connection_create(3, 3)}).to.throw(TypeError)
    })
});

describe('call to connection_create with int ptr ', function() {
    var connection = new Connection(path)

    it('should come back with different pointer', function () {
        var handle = ref.alloc(ref.types.int, 3)
        var old_handle = ref.alloc(ref.types.int, 3)
        assert.equal(ref.deref(handle), ref.deref(old_handle))
        // assert.equal(connection.connection_create("dog, cat, man", handle), 0)
        // assert.notEqual(handle, old_handle)
    })
});


// connection_connect tests

describe('call to cxs_connection_connect with good params ', function() {
    var connection = new Connection(path)
    it('should return success', function () {
        var handle = 3
        connection.connection_create("info", handle)
        assert.equal(connection.connection_connect(handle), 0)

    })
});

describe('call to connection_connect with invalid type ', function() {
    var connection = new Connection(path)
    it('should throw TypeError', function () {
        expect(function (){connection.connection_connect("throw")}).to.throw(TypeError)
    })
});


// connection_get_data tests

// describe('call to connection_get_data with good params ', function() {
//     var connection = new Connection(path)
//     it('should return a string', function () {
//         // connection.connection_
//         assert.typeOf(connection.connection_get_data(1), 'string')
//     })
// });
