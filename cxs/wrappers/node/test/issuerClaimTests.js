const assert = require('chai').assert

const IssuerClaim = require('../dist/index').IssuerClaim
const cxs = require('../dist/index')
const Connection = require('../dist/api/connection').Connection

const StateType = require('../dist/api/api').StateType
const SCHEMANUM = 32
const ATTR = '{"attr":"value"}'
const DID = '8XFh8yBzrpJQmNyZzgoTqB'

describe('An issuerClaim', async function () {
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
    const claim = await IssuerClaim.create(sourceId, SCHEMANUM, DID, ATTR)
    assert(claim.getClaimHandle() > 0)
    assert.equal(claim.getSourceId(), sourceId)
  })

  it('has state that can be found', async function () {
    const claim = await IssuerClaim.create('TestState', SCHEMANUM, DID, ATTR)
    await claim.updateState()
    assert.equal(claim.getState(), 1)
  })

  it('can be sent with a valid connection', async function () {
    const sourceId = 'Bank Claim'
    await cxs.init_cxs('ENABLE_TEST_MODE')
    var connection = new Connection()
    await connection.create({ id: '234' })
    const connectionHandle = await connection.getHandle()
    await connection.connect()
    await connection.updateState()
    assert.equal(2, connection.state)
    const claim = await IssuerClaim.create(sourceId, SCHEMANUM, DID, ATTR)
    await claim.send(connectionHandle)
    await claim.updateState()
    assert.equal(await claim.getState(), 2)
  })

  it('can be created, then serialized, then deserialized and have the same sourceId, state, and claimHandle', async function () {
    const sourceId = 'SerializeDeserialize'
    const claim = await IssuerClaim.create(sourceId, SCHEMANUM, DID, ATTR)
    const jsonClaim = await claim.serialize()
    assert.equal(jsonClaim.state, StateType.Initialized)
    const claim2 = await IssuerClaim.deserialize(jsonClaim)
    assert.equal(claim.getClaimHandle(), claim2.getClaimHandle())
    assert.equal(claim.getState(), claim2.getState())
  })

  it('can be sent, then serialized, then deserialized', async function () {
    // create a connection, send the claim, serialize and then deserialize
    // and compare
    cxs.init_cxs('ENABLE_TEST_MODE')
    var connection = new Connection()
    await connection.create({ id: '234' })
    const connectionHandle = await connection.getHandle()
    await connection.connect()

    const sourceId = 'SendSerializeDeserialize'
    const claim = await IssuerClaim.create(sourceId, SCHEMANUM, DID, ATTR)

    await claim.send(connectionHandle)
    const claimData = await claim.serialize()

    const claim2 = await IssuerClaim.deserialize(claimData)
    await claim.updateState()
    await claim2.updateState()
    assert.equal(claim.getState(), StateType.OfferSent)
    assert.equal(claim.getState(), claim2.getState())
    assert.equal(claim.getClaimHandle(), claim2.getClaimHandle())
  })

  it('is created from a static method', async function () {
    const sourceId = 'staticMethodCreation'
    const claim = await IssuerClaim.create(sourceId, SCHEMANUM, DID, ATTR)
    assert(claim.getSourceId(), sourceId)
  })

  it('will have different claim handles even with the same sourceIds', async function () {
    const sourceId = 'sameSourceIds'
    const claim = await IssuerClaim.create(sourceId, SCHEMANUM, DID, ATTR)
    const claim2 = await IssuerClaim.create(sourceId, SCHEMANUM, DID, ATTR)
    assert.notEqual(claim.getClaimHandle(), claim2.getClaimHandle)
  })

  it('deserialize is a static method', async function () {
    const sourceId = 'deserializeStatic'
    const claim = await IssuerClaim.create(sourceId, SCHEMANUM, DID, ATTR)
    const serializedJson = await claim.serialize()

    const claimDeserialized = await IssuerClaim.deserialize(serializedJson)
    assert.equal(claimDeserialized.getState(), StateType.Initialized)
  })

  it('accepts claim attributes and schema sequence number', async function () {
    const sourceId = 'attributesAndSequenceNumber'
    const schemaNum = 32
    const attributes = '{"attr":"value"}'
    const claim = await IssuerClaim.create(sourceId, schemaNum, DID, attributes)
    assert.equal(claim.getSourceId(), sourceId)
    assert.equal(claim.getSchemaNum(), schemaNum)
    assert.equal(claim.getAttr(), attributes)
  })

  it('accepts a claim offer DID as part of create', async function () {
    const sourceId = 'claimOfferDidTest'
    const claim = await IssuerClaim.create(sourceId, SCHEMANUM, DID, ATTR)
    assert.equal(claim.getSourceId(), sourceId)
    assert.equal(claim.getIssuedDid(), DID)
  })
})
