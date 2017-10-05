expect = require('chai').expect
get_one = require('../src/app')
cxs_init = require('../src/api/init')
describe('app', () => {
        it ('get_one should return 1', () => {
                result = get_one()
                expect(result).to.equal(1)
        })
});

describe('cxs_init', () => {
        it ('should return 0 when given 4 string arguments', () =>{
                result = cxs_init('pool1', 'config1', 'wallet1', 'default')
                expect(result).to.equal(0)

        })
        it ('should return 1001 when given an invalid argument', () => {
                result = cxs_init(null, 'config1', 'wallet1', 'default')
                expect(result).to.equal(1001)
        })
});

