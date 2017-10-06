"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var chai_1 = require("chai");
var mocha_1 = require("mocha");
var init_1 = require("../src/api/init");
mocha_1.describe('cxs_init', function () {
    mocha_1.it('should return 0 when given a null argument', function () {
        var result = init_1.init_cxs(null);
        chai_1.expect(result).to.equal(0);
    });
    mocha_1.it('should return 1001 when given an invalid argument', function () {
        var result = init_1.init_cxs('garbage');
        chai_1.expect(result).to.equal(1001);
    });
});
//# sourceMappingURL=wrapper-test.js.map