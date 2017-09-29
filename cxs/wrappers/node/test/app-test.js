expect = require('chai').expect
get_one = require('../src/app')

describe('app', () => {
        it ('get_one should return 1', () => {
                result = get_one()
                expect(result).to.equal(1)
        })
});

