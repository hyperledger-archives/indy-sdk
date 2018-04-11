const assert = require('chai').assert
const vcx = require('../dist/index')
const { stubInitVCX } = require('./helpers')
const { Connection, VCXMock, VCXMockMessage } = vcx

describe('A VCXMock', function () {
  this.timeout(30000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('can be called.', async () => {
    VCXMock.setVcxMock(VCXMockMessage.CreateKey)
    try {
      await Connection.create({ id: '234', DIDself: '456', DIDremote: '0' })
    } catch (err) {
      assert.equal(err.toString(), 'Error: vcx_connection_create -> 1010')
    }
  })
})
