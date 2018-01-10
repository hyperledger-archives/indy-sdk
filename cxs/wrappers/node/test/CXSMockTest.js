const assert = require('chai').assert
const cxs = require('../dist/index')
const { stubInitCXS } = require('./helpers')
const { Connection, CXSMock } = cxs

describe('A CXSMock', function () {
  this.timeout(30000)

  before(async () => {
    stubInitCXS()
    await cxs.initCxs('ENABLE_TEST_MODE')
  })

  it('can be called.', async () => {
    CXSMock.setCxsMock(1)
    try {
      await Connection.create({ id: '234', DIDself: '456', DIDremote: '0' })
    } catch (err) {
      assert.equal(err.toString(), 'Error: cxs_connection_create -> 1010')
    }
  })
})
