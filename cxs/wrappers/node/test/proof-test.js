const assert = require('chai').assert
// const cxs = require('../dist/index')
const { Proof, StateType, Error } = require('../dist/index')
// const Connection = require('../dist/api/connection').Connection

const ATTR = '{"attr":"value"}'
const DID = '8XFh8yBzrpJQmNyZzgoTqB'

describe('A Proof', async () => {
  it('can be created.', async () => {
    const proof = new Proof('Proof ID')
    assert(proof)
  })

  it('can have a source Id.', async () => {
    const proof = new Proof('Proof ID')
    assert.equal(proof.getSourceId(), 'Proof ID')
  })

  it('has a state of 0 after instanstiated', async () => {
    const proof = new Proof('Proof ID')
    const state = await proof.getState()
    assert.equal(state, 0)
  })

  it('has a proofHandle and a sourceId after it is created', async () => {
    const sourceId = 'Proof ID'
    const proof = await Proof.create(sourceId, DID, ATTR)
    assert(proof.getProofHandle())
    assert.equal(proof.getSourceId(), sourceId)
  })

  it('has state of Initialized after creating', async () => {
    const sourceId = 'Proof ID'
    const proof = await Proof.create(sourceId, DID, ATTR)
    assert.equal(proof.getState(), StateType.Initialized)
  })

  it('can be created, then serialized, then deserialized and have the same sourceId, state, and claimHandle', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create(sourceId, DID, ATTR)
    const jsonProof = await proof.serialize()
    assert.equal(jsonProof.state, StateType.Initialized)
    const proof2 = await Proof.deserialize(jsonProof)
    assert.equal(proof.getProofHandle(), proof2.getProofHandle())
    // assert.equal(proof.getState(), proof2.getState())
  })

  it('will throw error on serialize when proof has been released', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create(sourceId, DID, ATTR)
    const jsonProof = await proof.serialize()
    assert.equal(await proof.getState(), StateType.Initialized)
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
})
