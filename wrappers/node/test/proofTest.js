const assert = require('chai').assert
const vcx = require('../dist/index')
const { stubInitVCX } = require('./helpers')
const { Connection, Proof, StateType, Error, ProofState } = vcx

const ATTR = [{issuerDid: '8XFh8yBzrpJQmNyZzgoTqB', schemaSeqNo: 1, name: 'test'}]
const PROOF_MSG = '{"version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proofs":{"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"921....546","e":"158....756","v":"114....069","m":{"address2":"140....691","city":"209....294","address1":"111...738","zip":"149....066"},"m1":"777....518","m2":"515....229"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH"}},"aggregated_proof":{"c_hash":"25105671496406009212798488318112715144459298495509265715919744143493847046467","c_list":[[72,245,38,"....",46,195,18]]},"requested_proof":{"revealed_attrs":{"attr_key_id":["claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b","UT","96473275571522321025213415717206189191162"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}}}'

describe('A Proof', function () {
  this.timeout(30000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('can be created.', async () => {
    const proof = new Proof('Proof ID')
    assert(proof)
  })

  it('can have a source Id.', async () => {
    const proof = new Proof('Proof ID')
    assert.equal(proof.sourceId, 'Proof ID')
  })

  it('has a proofHandle and a sourceId after it is created', async () => {
    const sourceId = '1'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    assert(proof.handle)
    assert.equal(proof.sourceId, sourceId)
  })

  it('has state of Initialized after creating', async () => {
    const sourceId = 'Proof ID'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    assert.equal(await proof.getState(), StateType.Initialized)
  })

  it('can be created, then serialized, then deserialized and have the same sourceId and state', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    const jsonProof = await proof.serialize()
    assert.equal(jsonProof.state, StateType.Initialized)
    const proof2 = await Proof.deserialize(jsonProof)
    assert.equal(proof.sourceId, proof2.sourceId)
    assert.equal(await proof.getState(), await proof2.getState())
  })

  it('will throw error on serialize when proof has been released', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    const jsonProof = await proof.serialize()
    assert.equal(await proof.getState(), StateType.Initialized)
    let data = await proof.serialize()
    assert(data)
    assert.equal(data.handle, jsonProof.handle)
    assert.equal(await proof.release(), Error.SUCCESS)
    try {
      await proof.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, 1017)
      assert.equal(error.vcxFunction, 'vcx_proof_serialize')
      assert.equal(error.message, 'Invalid Proof Handle')
    }
  })

  it('has correct state after deserializing', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    const jsonProof = await proof.serialize()
    const proof2 = await Proof.deserialize(jsonProof)
    assert.equal(await proof2.getState(), StateType.Initialized)
  })

  it('has state of OfferSent after sending proof request', async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    await proof.requestProof(connection)
    assert.equal(await proof.getState(), StateType.OfferSent)
  })

  it('requesting a proof throws invalid connection error with released connection', async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    await connection.release()
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    try {
      await proof.requestProof(connection)
    } catch (error) {
      assert.equal(error.vcxCode, 1003)
      assert.equal(error.vcxFunction, 'vcx_proof_send_request')
      assert.equal(error.message, 'Invalid Connection Handle')
    }
  })

  it('requesting a proof throws invalid proof error with released proof', async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    await connection.release()
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    await proof.release()
    try {
      await proof.requestProof(connection)
    } catch (error) {
      assert.equal(error.vcxCode, 1017)
      assert.equal(error.vcxFunction, 'vcx_proof_send_request')
      assert.equal(error.message, 'Invalid Proof Handle')
    }
  })

  it('get proof has an invalid proof state with incorrect proof', async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    let jsonProof = await proof.serialize()
    // console.log(jsonProof)
    jsonProof.proof = JSON.parse(PROOF_MSG)
    jsonProof.state = StateType.Accepted
    jsonProof.proof_state = ProofState.Invalid
    jsonProof.handle = 8223
    const proof2 = await Proof.deserialize(jsonProof)
    await proof2.updateState()
    let proofData = await proof2.getProof(connection)
    assert.equal(proof2.getProofState(), ProofState.Invalid)
    const attrs = '[{"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH","claim_uuid":"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b","attr_info":{"name":"state","value":"UT","type":"revealed"}}]'
    const expectedData = {proofAttrs: attrs, proofState: ProofState.Invalid}
    assert.equal(JSON.stringify(proofData.proofAttrs), expectedData.proofAttrs)
    assert.equal(proofData.proofState, expectedData.proofState)
  })
})
