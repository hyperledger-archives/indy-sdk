const assert = require('chai').assert
const { IssuerClaim, Connection, StateType, Error, initCxs } = require('../dist/index')

let config = {
  sourceId: 'jsonCreation',
  schemaNum: 1234,
  issuerDid: 'arandomdidfoobar',
  attr: "{key: 'value'}"
}
describe('An issuerClaim', async function () {
  this.timeout(10000)

  before(async () => {
    await initCxs('ENABLE_TEST_MODE')
  })

  it('can be created.', async function () {
    const claim = new IssuerClaim('Bank Claim')
    assert(claim)
  })

  it('can have a source Id.', async function () {
    const claim = await new IssuerClaim('Bank Claim')
    assert.equal(claim.getSourceId(), 'Bank Claim')
  })

  it('has a state of 0 after instanstiated', async function () {
    const claim = await new IssuerClaim('State Claim')
    const state = await claim.getState()
    assert.equal(state, 0)
  })

  it('has a claimHandle and a sourceId after it is created', async function () {
    const sourceId = 'Claim'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    assert(claim.getClaimHandle() > 0)
    assert.equal(claim.getSourceId(), sourceId)
  })

  it('has state that can be found', async function () {
    config.sourceId = 'TestState'
    const claim = await IssuerClaim.create(config)
    await claim.updateState()
    assert.equal(claim.getState(), 1)
  })

  it('can be sent with a valid connection', async function () {
    const sourceId = 'Bank Claim'
    config.sourceId = sourceId
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    await connection.updateState()
    assert.equal(2, connection.getState())
    const claim = await IssuerClaim.create(config)
    await claim.sendOffer(connection)
    await claim.updateState()
    assert.equal(await claim.getState(), 2)
  })

  it('can be created, then serialized, then deserialized and have the same sourceId, state, and claimHandle', async function () {
    const sourceId = 'SerializeDeserialize'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    const jsonClaim = await claim.serialize()
    assert.equal(jsonClaim.state, StateType.Initialized)
    const claim2 = await IssuerClaim.deserialize(jsonClaim)
    assert.equal(claim.getClaimHandle(), claim2.getClaimHandle())
    assert.equal(claim.getState(), claim2.getState())
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
    assert.equal(claim.getState(), StateType.OfferSent)
    assert.equal(claim.getState(), claim2.getState())
    assert.equal(claim.getClaimHandle(), claim2.getClaimHandle())
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
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    assert(claim.getSourceId(), sourceId)
  })

  it('will have different claim handles even with the same sourceIds', async function () {
    const sourceId = 'sameSourceIds'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    const claim2 = await IssuerClaim.create(config)
    assert.notEqual(claim.getClaimHandle(), claim2.getClaimHandle)
  })

  it('deserialize is a static method', async function () {
    const sourceId = 'deserializeStatic'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    const serializedJson = await claim.serialize()

    const claimDeserialized = await IssuerClaim.deserialize(serializedJson)
    assert.equal(claimDeserialized.getState(), StateType.Initialized)
  })

  it('accepts claim attributes and schema sequence number', async function () {
    const sourceId = 'attributesAndSequenceNumber'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    assert.equal(claim.getSourceId(), config.sourceId)
    assert.equal(claim.getSchemaNum(), config.schemaNum)
    assert.equal(claim.getAttr(), config.attr)
  })

  it('accepts a claim offer DID as part of create', async function () {
    const sourceId = 'claimOfferDidTest'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    assert.equal(claim.getSourceId(), config.sourceId)
    assert.equal(claim.getIssuedDid(), config.issuerDid)
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
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    try {
      await claim.sendClaim(releasedConnection)
    } catch (error) {
      assert.equal(error.toString(), 'Error: cxs_issuer_send_claim -> ' + Error.INVALID_CONNECTION_HANDLE)
    }
  })

  it('sending claim with no claim offer should throw exception', async function () {
    let connection = await Connection.create({id: '123'})
    const sourceId = 'Claim'
    config.sourceId = sourceId
    const claim = await IssuerClaim.create(config)
    try {
      await claim.sendClaim(connection)
    } catch (error) {
      assert.equal(error.toString(), 'Error: cxs_issuer_send_claim -> ' + Error.NOT_READY)
    }
  })

  it('sending claim with valid claim offer should have state CxsStateAccepted', async function () {
    let connection = await Connection.create({id: '123'})
    await connection.connect({ sms: true })
    const sourceId = 'Claim'
    config.sourceId = sourceId
    let claim = await IssuerClaim.create(config)
    await claim.sendOffer(connection)
    assert.equal(await claim.getState(), StateType.OfferSent)
    let jsonClaim = await claim.serialize()
    jsonClaim.state = StateType.RequestReceived
    jsonClaim.handle += 1
    claim = await IssuerClaim.deserialize(jsonClaim)
    await claim.sendClaim(connection)
    assert.equal(await claim.getState(), StateType.Accepted)
  })

  it('can be created from a json', async function () {
    await IssuerClaim.create(config)
  })
})
