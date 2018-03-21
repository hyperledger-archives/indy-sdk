const assert = require('chai').assert
const vcx = require('../dist/index')
const { stubInitVCX } = require('./helpers')
const { ClaimDef, Error } = vcx

const CLAIM_DEF = {
  name: 'test',
  revocation: false,
  schemaNo: 15,
  sourceId: 'sourceId'
}

describe('A ClaimDef', function () {
  this.timeout(30000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('can be created.', async () => {
    const claimDef = await ClaimDef.create(CLAIM_DEF)
    assert(claimDef)
  })

  it('has a name of test after instanstiated', async () => {
    const claimDef = await ClaimDef.create(CLAIM_DEF)
    const name = await claimDef._name
    assert.equal(name, 'test')
  })

  it('can be created, then serialized, then deserialized and have the same sourceId and name', async () => {
    const claimDef = await ClaimDef.create(CLAIM_DEF)
    const jsonDef = await claimDef.serialize()
    assert.equal(jsonDef.source_id, 'sourceId')
    const claimDef2 = await ClaimDef.deserialize(jsonDef)
    assert.equal(claimDef.name, claimDef2.name)
    assert.equal(claimDef.source_id, claimDef2.source_id)
  })

  it('will throw error on serialize when claimDef has been released', async () => {
    const claimDef = await ClaimDef.create(CLAIM_DEF)
    const jsonDef = await claimDef.serialize()
    let data = await claimDef.serialize()
    assert(data)
    assert.equal(data.handle, jsonDef.handle)
    assert.equal(await claimDef.release(), Error.SUCCESS)
    try {
      await claimDef.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, 1037)
      assert.equal(error.vcxFunction, 'vcx_claimdef_serialize')
      assert.equal(error.message, 'Invalid Claim Definition handle')
    }
  })
})
