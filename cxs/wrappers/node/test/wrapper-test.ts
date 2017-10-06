import {expect} from 'chai';
import {describe, it} from 'mocha';
import {init_cxs} from '../src/api/init';

describe('cxs_init', () => {
    it ('should return 0 when given 4 string arguments', () =>{
            var result = init_cxs('pool1', 'config1', 'wallet1', 'default')
            expect(result).to.equal(0)

    })
    it ('should return 1001 when given an invalid argument', () => {
            var result = init_cxs(null, 'config1', 'wallet2', 'default')
            expect(result).to.equal(1001)
    })
});