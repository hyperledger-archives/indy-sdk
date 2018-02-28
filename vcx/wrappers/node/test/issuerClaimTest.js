const assert = require('chai').assert
const expect = require('chai').expect
const vcx = require('../dist')
const { stubInitVCX, shouldThrow } = require('./helpers')

const { IssuerClaim, Connection, StateType, Error } = vcx

const config = {
  sourceId: 'jsonCreation',
  schemaNum: 1234,
  issuerDid: 'arandomdidfoobar',
  attr: {
    key: 'value',
    key2: 'value2',
    key3: 'value3'
  },
  claimName: 'Claim Name'
}
const formattedAttrs = {
  key: ['value'],
  key2: ['value2'],
  key3: ['value3']
}
const claimDummyArgs = [
  'Dummy Claim',
  {
    schemaNum: 1,
    issuerDid: 'arandomdidfoobar',
    claimName: 'Claim Name'
  }
]
describe('An issuerClaim', async function () {
  this.timeout(30000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('can be created.', async function () {
    const claim = new IssuerClaim(...claimDummyArgs)
    assert(claim)
  })

  it('can have a source Id.', async function () {
    const claim = await new IssuerClaim(...claimDummyArgs)
    assert.equal(claim.sourceId, claimDummyArgs[0])
  })

  it('has a state of 0 after instanstiated', async function () {
    const claim = await new IssuerClaim(...claimDummyArgs)
    const state = await claim.getState()
    assert.equal(state, 0)
  })

  it('has a claimHandle and a sourceId after it is created', async function () {
    const sourceId = 'Claim'
    const claim = await IssuerClaim.create({ ...config, sourceId })
    assert(claim.handle > 0)
    assert.equal(claim.sourceId, sourceId)
  })

  it('has state that can be found', async function () {
    const sourceId = 'TestState'
    const claim = await IssuerClaim.create({ ...config, sourceId })
    await claim.updateState()
    assert.equal(await claim.getState(), 1)
  })

  it('can be sent with a valid connection', async function () {
    const sourceId = 'Bank Claim'
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    assert.equal(StateType.OfferSent, await connection.getState())
    const claim = await IssuerClaim.create({ ...config, sourceId })
    await claim.sendOffer(connection)
    await claim.updateState()
    assert.equal(await claim.getState(), StateType.OfferSent)
  })

  it('can be created, then serialized, then deserialized and have the same sourceId, state, and claimHandle', async function () {
    const sourceId = 'SerializeDeserialize'
    const claim = await IssuerClaim.create({ ...config, sourceId })
    const jsonClaim = await claim.serialize()
    assert.equal(jsonClaim.state, StateType.Initialized)
    const claim2 = await IssuerClaim.deserialize(jsonClaim)
    assert.equal(claim.handle, claim2.handle)
    assert.equal(await claim.getState(), await claim2.getState())
  })

  it('can be sent, then serialized, then deserialized', async function () {
    // create a connection, send the claim, serialize and then deserialize
    // and compare
    const connection = await Connection.create({ id: '234' })
    await connection.connect()

    const sourceId = 'SendSerializeDeserialize'
    const claim = await IssuerClaim.create({ ...config, sourceId })

    await claim.sendOffer(connection)
    const claimData = await claim.serialize()

    const claim2 = await IssuerClaim.deserialize(claimData)
    await claim.updateState()
    await claim2.updateState()
    assert.equal(await claim.getState(), StateType.OfferSent)
    assert.equal(await claim.getState(), await claim2.getState())
    assert.equal(claim.handle, claim2.handle)
  })

  it('serialize without correct handle throws error', async function () {
    const claim = new IssuerClaim(null, {})
    try {
      await claim.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, Error.INVALID_ISSUER_CLAIM_HANDLE)
      assert.equal(error.message, 'Invalid Claim Issuer Handle')
    }
  })

  it('is created from a static method', async function () {
    const sourceId = 'staticMethodCreation'
    const claim = await IssuerClaim.create({ ...config, sourceId })
    assert(claim.sourceId, sourceId)
  })

  it('will have different claim handles even with the same sourceIds', async function () {
    const sourceId = 'sameSourceIds'
    const claim = await IssuerClaim.create({ ...config, sourceId })
    const claim2 = await IssuerClaim.create({ ...config, sourceId })
    assert.notEqual(claim.handle, claim2.handle)
  })

  it('deserialize is a static method', async function () {
    const sourceId = 'deserializeStatic'
    const claim = await IssuerClaim.create({ ...config, sourceId })
    const serializedJson = await claim.serialize()

    const claimDeserialized = await IssuerClaim.deserialize(serializedJson)
    assert.equal(await claimDeserialized.getState(), StateType.Initialized)
  })

  it('accepts claim attributes and schema sequence number', async function () {
    const sourceId = 'attributesAndSequenceNumber'
    const claim = await IssuerClaim.create({ ...config, sourceId })
    assert.equal(claim.sourceId, sourceId)
    assert.equal(claim.schemaNum, config.schemaNum)
    assert.deepEqual(claim.attr, formattedAttrs)
  })

  it('throws exception for sending claim with invalid claim handle', async function () {
    let connection = await Connection.create({id: '123'})
    const claim = new IssuerClaim(null, {})
    try {
      await claim.sendClaim(connection)
    } catch (error) {
      assert.equal(error.vcxCode, Error.INVALID_ISSUER_CLAIM_HANDLE)
      assert.equal(error.message, 'Invalid Claim Issuer Handle')
    }
  })

  it('throws exception for sending claim with invalid connection handle', async function () {
    let releasedConnection = await Connection.create({id: '123'})
    await releasedConnection.release()
    const sourceId = 'Claim'
    const claim = await IssuerClaim.create({ ...config, sourceId })
    try {
      await claim.sendClaim(releasedConnection)
    } catch (error) {
      assert.equal(error.vcxCode, Error.INVALID_CONNECTION_HANDLE)
      assert.equal(error.message, 'Invalid Connection Handle')
    }
  })

  it('sending claim with no claim offer should throw exception', async function () {
    let connection = await Connection.create({id: '123'})
    const sourceId = 'Claim'
    const claim = await IssuerClaim.create({ ...config, sourceId })
    const error = await shouldThrow(() => claim.sendClaim(connection))
    assert.equal(error.vcxCode, Error.NOT_READY)
    assert.equal(error.vcxFunction, 'vcx_issuer_send_claim')
    assert.equal(error.message, 'Object not ready for specified action')
  })

  it('sending claim with valid claim offer should have state VcxStateAccepted', async function () {
    let connection = await Connection.create({id: '123'})
    await connection.connect({ sms: true })
    const sourceId = 'Claim'
    let claim = await IssuerClaim.create({ ...config, sourceId })
    await claim.sendOffer(connection)
    assert.equal(await claim.getState(), StateType.OfferSent)
    // we serialize and deserialize because this is the only
    // way to fool the libvcx into thinking we've received a
    // valid claim requset.
    let jsonClaim = await claim.serialize()
    jsonClaim.state = StateType.RequestReceived
    jsonClaim.handle += 1
    claim = await IssuerClaim.deserialize(jsonClaim)
    await claim.sendClaim(connection)
    assert.equal(await claim.getState(), StateType.Accepted)
  })

  it('can be created from a json', async function () {
    const claim = await IssuerClaim.create(config)
    expect(claim.sourceId).to.equal(config.sourceId)
  })
})
