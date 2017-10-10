require('chai');
require('fs-extra');
var parentDir = require('path');
var currentDir = parentDir.dirname(module.filename);
var index = require(parentDir.dirname(currentDir) + '/dist/index')
var rustlib = require(parentDir.dirname(currentDir) + '/dist/rustlib')
var assert = require('assert');
var CXSRuntime = index.CXSRuntime;
var CXSRuntimeConfig = rustlib.CXSRuntimeConfig;
var ref = require('ref')


describe('call to cxs_init with provided path', function() {
    var path = parentDir.dirname(currentDir);
    path += "/lib/libcxs.so";
    var run = new CXSRuntime(new CXSRuntimeConfig(path));
    it('should return 0', function () {
        assert.equal(run.ffi.cxs_init(null), 0);
    })

    it('should return 1001', function() {
            assert.equal(run.ffi.cxs_init('garbage'), 1001);
    })
});

// these tests were created to only test that the ffi could be called with each function

describe('Using the cxs ffi directly ', function() {
    var path = parentDir.dirname(currentDir);
    path += "/lib/libcxs.so";
    var run = new CXSRuntime(new CXSRuntimeConfig(path));

    it('a call to cxs_connection_create should return 0', function () {
        var intPtr = ref.alloc('int')
        assert.equal(run.ffi.cxs_connection_create("dog, cat, man", intPtr), 0)
    })

    it('a to cxs_connection_connect without the ability to connect should return 1', function () {
        assert.equal(run.ffi.cxs_connection_connect(2), 1001)
    })

    it('a call to cxs_connection_get_data should return 0', function () {
        assert.equal(run.ffi.cxs_connection_get_data(2), null)
    })

    it('a call to cxs_connection_get_state should return 0', function () {
        var intPtr = ref.alloc('int')
        assert.equal(run.ffi.cxs_connection_get_state(2, intPtr), 0)
    })

    it('a call to cxs_connection_release without ability to release should return 1', function() {
        assert.equal(run.ffi.cxs_connection_release(2), 1001)
    })

});
