import { expect } from 'chai'
import { describe, it } from 'mocha'
import { initCxs } from '../dist/index'

describe('cxs_init', () => {
  it ('should return 0 when given a null argument', () => {
    const result = initCxs(null)
    expect(result).to.equal(0)
  })
  it ('should return 1001 when given an invalid argument', () => {
    const result = initCxs('garbage')
    expect(result).to.equal(1001)
  })
})
