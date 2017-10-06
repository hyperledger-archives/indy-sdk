import {expect} from 'chai';
import {describe, it} from 'mocha';
import {init_cxs} from '../src/api/init';

describe('cxs_init', () => {
    it ('should return 0 when given a null argument', () =>{
            var result = init_cxs(null)
            expect(result).to.equal(0)

    })
    it ('should return 1001 when given an invalid argument', () => {
            var result = init_cxs('garbage')
            expect(result).to.equal(1001)
    })
});