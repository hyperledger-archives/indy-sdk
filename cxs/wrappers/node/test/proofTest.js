const assert = require('chai').assert
const cxs = require('../dist/index')
const { stubInitCXS } = require('./helpers')
const { Connection, Proof, StateType, Error } = cxs

const ATTR = [{'issuer_did': '8XFh8yBzrpJQmNyZzgoTqB', schema_seq_no: 1, name: 'test'}]
const DID = '8XFh8yBzrpJQmNyZzgoTqB'

describe('A Proof', function () {
  this.timeout(30000)

  before(async () => {
    stubInitCXS()
    await cxs.initCxs('ENABLE_TEST_MODE')
  })

  it('can be created.', async () => {
    const proof = new Proof('Proof ID')
    assert(proof)
  })

  it('can have a source Id.', async () => {
    const proof = new Proof('Proof ID')
    assert.equal(proof.sourceId, 'Proof ID')
  })

  it('has a state of 0 after instanstiated', async () => {
    const proof = new Proof('Proof ID')
    const state = await proof.state
    assert.equal(state, 0)
  })

  it('has a proofHandle and a sourceId after it is created', async () => {
    const sourceId = '1'
    const proof = await Proof.create({ sourceId, proof_requester_did: DID, attrs: ATTR, name: 'TestProof' })
    assert(proof.handle)
    assert.equal(proof.sourceId, sourceId)
  })

  it('has state of Initialized after creating', async () => {
    const sourceId = 'Proof ID'
    const proof = await Proof.create({ sourceId, proof_requester_did: DID, attrs: ATTR, name: 'TestProof' })
    assert.equal(proof.state, StateType.Initialized)
  })

  it('can be created, then serialized, then deserialized and have the same sourceId, state, and claimHandle', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, proof_requester_did: DID, attrs: ATTR, name: 'TestProof' })
    const jsonProof = await proof.serialize()
    assert.equal(jsonProof.state, StateType.Initialized)
    const proof2 = await Proof.deserialize(jsonProof)
    assert.equal(proof.handle, proof2.handle)
    assert.equal(proof.state, proof2.state)
  })

  it('will throw error on serialize when proof has been released', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, proof_requester_did: DID, attrs: ATTR, name: 'TestProof' })
    const jsonProof = await proof.serialize()
    assert.equal(await proof.state, StateType.Initialized)
    let data = await proof.serialize()
    assert(data)
    assert.equal(data.handle, jsonProof.handle)
    assert.equal(await proof.release(), Error.SUCCESS)
    try {
      await proof.serialize()
    } catch (error) {
      assert.equal(error.toString(), 'Error: cxs_proof_serialize -> 1017')
    }
  })

  it('has correct state after deserializing', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, proof_requester_did: DID, attrs: ATTR, name: 'TestProof' })
    const jsonProof = await proof.serialize()
    const proof2 = await Proof.deserialize(jsonProof)
    assert.equal(proof2.state, StateType.Initialized)
  })

  it('has state of OfferSent after sending proof request', async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, proof_requester_did: DID, attrs: ATTR, name: 'TestProof' })
    await proof.requestProof(connection)
    assert.equal(proof.state, StateType.OfferSent)
  })

  it('requesting a proof throws invalid connection error with released connection', async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    await connection.release()
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, proof_requester_did: DID, attrs: ATTR, name: 'TestProof' })
    try {
      await proof.requestProof(connection)
    } catch (err) {
      assert.equal(err.toString(), 'Error: cxs_proof_send_request -> 1003')
    }
  })

  it('requesting a proof throws invalid proof error with released proof', async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    await connection.release()
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, proof_requester_did: DID, attrs: ATTR, name: 'TestProof' })
    await proof.release()
    try {
      await proof.requestProof(connection)
    } catch (err) {
      assert.equal(err.toString(), 'Error: cxs_proof_send_request -> 1017')
    }
  })
})
