const assert = require('chai').assert
// const cxs = require('../dist/index')
const cxs = require('../dist')
const { stubInitCXS } = require('./helpers')
// const Connection = require('../dist/api/connection').Connection

// const { Proof, StateType, Error } = cxs // Todo: Uncomment once update_state is working
const { Proof } = cxs
// const ATTR = '{"attr":"value"}'
// const DID = '8XFh8yBzrpJQmNyZzgoTqB'
describe('A Proof', async () => {
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
  // Todo: These tests can be uncommented out once proof_update_state is
  // it('has a proofHandle and a sourceId after it is created', async () => {
  //   const sourceId = 'Proof ID'
  //   const proof = await Proof.create(sourceId, DID, ATTR)
  //   assert(proof.handle)
  //   assert.equal(proof.sourceId, sourceId)
  // })

  // it('has state of Initialized after creating', async () => {
  //   const sourceId = 'Proof ID'
  //   const proof = await Proof.create(sourceId, DID, ATTR)
  //   assert.equal(proof.state, StateType.Initialized)
  // })

  // it('can be created, then serialized, then deserialized and have the same sourceId, state, and claimHandle', async () => {
  //   const sourceId = 'SerializeDeserialize'
  //   const proof = await Proof.create(sourceId, DID, ATTR)
  //   const jsonProof = await proof.serialize()
  //   assert.equal(jsonProof.state, StateType.Initialized)
  //   const proof2 = await Proof.deserialize(jsonProof)
  //   assert.equal(proof.handle, proof2.handle)
  //   // assert.equal(proof.state, proof2.state)
  // })

  // it('will throw error on serialize when proof has been released', async () => {
  //   const sourceId = 'SerializeDeserialize'
  //   const proof = await Proof.create(sourceId, DID, ATTR)
  //   const jsonProof = await proof.serialize()
  //   assert.equal(await proof.state, StateType.Initialized)
  //   let data = await proof.serialize()
  //   assert(data)
  //   assert.equal(data.handle, jsonProof.handle)
  //   assert.equal(await proof.release(), Error.SUCCESS)
  //   try {
  //     await proof.serialize()
  //   } catch (error) {
  //     assert.equal(error.toString(), 'Error: cxs_proof_serialize -> 1017')
  //   }
  // })
})
