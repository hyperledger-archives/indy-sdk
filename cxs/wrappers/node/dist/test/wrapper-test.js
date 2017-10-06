"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var chai_1 = require("chai");
var mocha_1 = require("mocha");
var init_1 = require("../src/api/init");
mocha_1.describe('cxs_init', function () {
    mocha_1.it('should return 0 when given 4 string arguments', function () {
        var result = init_1.init_cxs('pool1', 'config1', 'wallet1', 'default');
        chai_1.expect(result).to.equal(0);
    });
    mocha_1.it('should return 1001 when given an invalid argument', function () {
        var result = init_1.init_cxs(null, 'config1', 'wallet2', 'default');
        chai_1.expect(result).to.equal(1001);
    });
});
//# sourceMappingURL=wrapper-test.js.map