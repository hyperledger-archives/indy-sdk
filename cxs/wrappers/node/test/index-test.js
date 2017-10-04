var chai = require('chai');
var fs = require('fs-extra');
var index = require('../src/index.js');
var assert = require('assert');
var parentDir = require('path');
var CXSRuntime = index.CXSRuntime;
var CXSRuntimeConfig = index.CXSRuntimeConfig;
var currentDir = parentDir.dirname(module.filename);


describe('call to cxs_init with provided path', function() {
    var path = parentDir.dirname(currentDir);
    path += "/lib/libcxs.so";
    var run = new CXSRuntime(new CXSRuntimeConfig(path));
    it('should return 0', function () {
        assert.equal(run.ffi.cxs_init('pool1', 'config1', 'wallet1','default'), 0); 
    })

    it('should return 1002', function() {
            assert.equal(run.ffi.cxs_init(' ', 'config1', 'wallet1','default'), 0); 
    })
});
