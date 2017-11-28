const assert = require('chai').assert
const expect = require('chai').expect
const cxs = require('../dist')
const { stubInitCXS, shouldThrow } = require('./helpers')

const { IssuerClaim, Connection, StateType, Error } = cxs

let config = {
  sourceId: 'jsonCreation',
  schemaNum: 1234,
  issuerDid: 'arandomdidfoobar',
  attr: JSON.stringify({key: ['value']}),
  claimName: 'Claim Name'
}
describe('An issuerClaim', async function () {
  this.timeout(30000)

  before(async () => {
    stubInitCXS()
    await cxs.initCxs('ENABLE_TEST_MODE')
  })

  it('can be created.', async function () {
    const claim = new IssuerClaim('Bank Claim')
    assert(claim)
  })

  it('can have a source Id.', async function () {
    const claim = await new IssuerClaim('Bank Claim')
    assert.equal(claim.sourceId, 'Bank Claim')
  })

  it('has a state of 0 after instanstiated', async function () {
    const claim = await new IssuerClaim('State Claim')
    const state = await claim.state
    assert.equal(state, 0)
  })

  it('has a claimHandle and a sourceId after it is created', async function () {
    const sourceId = 'Claim'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    assert(claim.handle > 0)
    assert.equal(claim.sourceId, sourceId)
  })

  it('has state that can be found', async function () {
    config.sourceId = 'TestState'
    const claim = await IssuerClaim.create(config)
    await claim.updateState()
    assert.equal(claim.state, 1)
  })

  it('can be sent with a valid connection', async function () {
    const sourceId = 'Bank Claim'
    config.sourceId = sourceId
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    await connection.updateState()
    assert.equal(2, connection.state)
    const claim = await IssuerClaim.create(config)
    await claim.sendOffer(connection)
    await claim.updateState()
    assert.equal(await claim.state, 2)
  })

  it('can be created, then serialized, then deserialized and have the same sourceId, state, and claimHandle', async function () {
    const sourceId = 'SerializeDeserialize'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    const jsonClaim = await claim.serialize()
    assert.equal(jsonClaim.state, StateType.Initialized)
    const claim2 = await IssuerClaim.deserialize(jsonClaim)
    assert.equal(claim.handle, claim2.handle)
    assert.equal(claim.state, claim2.state)
  })

  it('can be sent, then serialized, then deserialized', async function () {
    // create a connection, send the claim, serialize and then deserialize
    // and compare
    let connection = await Connection.create({ id: '234' })
    await connection.connect()

    const sourceId = 'SendSerializeDeserialize'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)

    await claim.sendOffer(connection)
    const claimData = await claim.serialize()

    const claim2 = await IssuerClaim.deserialize(claimData)
    await claim.updateState()
    await claim2.updateState()
    assert.equal(claim.state, StateType.OfferSent)
    assert.equal(claim.state, claim2.state)
    assert.equal(claim.handle, claim2.handle)
  })

  it('serialize without correct handle throws error', async function () {
    const claim = new IssuerClaim(null)
    try {
      await claim.serialize()
    } catch (error) {
      assert.equal(error.toString(), 'Error: cxs_issuer_claim_serialize -> ' + Error.INVALID_ISSUER_CLAIM_HANDLE)
    }
  })

  it('is created from a static method', async function () {
    const sourceId = 'staticMethodCreation'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    assert(claim.sourceId, sourceId)
  })

  it('will have different claim handles even with the same sourceIds', async function () {
    const sourceId = 'sameSourceIds'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    const claim2 = await IssuerClaim.create(config)
    assert.notEqual(claim.handle, claim2.handle)
  })

  it('deserialize is a static method', async function () {
    const sourceId = 'deserializeStatic'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    const serializedJson = await claim.serialize()

    const claimDeserialized = await IssuerClaim.deserialize(serializedJson)
    assert.equal(claimDeserialized.state, StateType.Initialized)
  })

  it('accepts claim attributes and schema sequence number', async function () {
    const sourceId = 'attributesAndSequenceNumber'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    assert.equal(claim.sourceId, config.sourceId)
    assert.equal(claim.schemaNum, config.schemaNum)
    assert.equal(claim.attr, config.attr)
  })

  it('accepts a claim offer DID as part of create', async function () {
    const sourceId = 'claimOfferDidTest'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    assert.equal(claim.sourceId, config.sourceId)
    assert.equal(claim.issuerDid, config.issuerDid)
  })

  it('throws exception for sending claim with invalid claim handle', async function () {
    let connection = await Connection.create({id: '123'})
    const claim = new IssuerClaim(null)
    try {
      await claim.sendClaim(connection)
    } catch (error) {
      assert.equal(error.toString(), 'Error: cxs_issuer_send_claim -> ' + Error.INVALID_ISSUER_CLAIM_HANDLE)
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
      assert.equal(error.toString(), 'Error: cxs_issuer_send_claim -> ' + Error.INVALID_CONNECTION_HANDLE)
    }
  })

  it('sending claim with no claim offer should throw exception', async function () {
    let connection = await Connection.create({id: '123'})
    const sourceId = 'Claim'
    const claim = await IssuerClaim.create({ ...config, sourceId })
    const error = await shouldThrow(() => claim.sendClaim(connection))
    assert.equal(error.toString(), 'Error: cxs_issuer_send_claim -> ' + Error.NOT_READY)
  })

  it('sending claim with valid claim offer should have state CxsStateAccepted', async function () {
    let connection = await Connection.create({id: '123'})
    await connection.connect({ sms: true })
    const sourceId = 'Claim'
    let claim = await IssuerClaim.create({ ...config, sourceId })
    await claim.sendOffer(connection)
    assert.equal(await claim.state, StateType.OfferSent)
    // we serialize and deserialize because this is the only
    // way to fool the libcxs into thinking we've received a
    // valid claim requset.
    let jsonClaim = await claim.serialize()
    jsonClaim.state = StateType.RequestReceived
    jsonClaim.handle += 1
    claim = await IssuerClaim.deserialize(jsonClaim)
    console.log('Claim Data' + claim.claimData)
    await claim.sendClaim(connection)
    assert.equal(claim.state, StateType.Accepted)
  })

  it('can be created from a json', async function () {
    const claim = await IssuerClaim.create(config)
    expect(claim.sourceId).to.equal(config.sourceId)
  })
})
