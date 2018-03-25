const assert = require('chai').assert
const vcx = require('../dist/index')
const { stubInitVCX } = require('./helpers')
const { Connection, DisclosedProof } = vcx

describe('A disclosedProof', function () {
  this.timeout(30000)
  const PROOF_REQ = `{
    "@type":{
      "name":"PROOF_REQUEST",
      "version":"1.0"
    },
    "@topic":{
      "mid":9,
      "tid":1
    },
    "proof_request_data":{
      "nonce":"838186471541979035208225",
      "name":"Account Certificate",
      "version":"0.1",
      "requested_attrs":{
        "email_1":{
          "name":"email",
          "schema_seq_no":52
        },
        "business_2":{
          "name":"business",
          "schema_seq_no":52
        },
        "name_0":{
          "name":"name",
          "schema_seq_no":52
        }
      },
      "requested_predicates":{
  
      }
    },
    "msg_ref_id":"abcd"
  }`

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  // create tests

  it('can be created.', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
  })

  it(' a call to create with no sourceId returns an error', async () => {
    try {
      await DisclosedProof.create({request: PROOF_REQ})
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it(' a call to create with no request returns an error', async () => {
    try {
      await DisclosedProof.create({sourceId: 'Test'})
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it(' a call to create with a bad request returns an error', async () => {
    try {
      await DisclosedProof.create({sourceId: 'Test', request: '{}'})
    } catch (error) {
      assert.equal(error.vcxCode, 1016)
    }
  })

  // serialize/deserialize tests

  it('can be serialized.', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
    const val = await obj.serialize()
    assert(val)
  })

  it('can be deserialized.', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
    const val = await obj.serialize()
    assert(val)
    const obj2 = await DisclosedProof.deserialize(val)
    assert(obj2)
  })

  // state tests

  it('can get state.', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
    const state = await obj.getState()
    assert(state === 3)
  })

  it('can update state.', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
    await obj.updateState()
    const state = await obj.getState()
    assert(state === 3)
  })

  // sendProof tests

  it('can send a proof.', async () => {
    let connection = await Connection.create({ id: '234' })
    assert(connection)
    await connection.connect()
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
    await obj.sendProof(connection)
    const state = await obj.getState()
    assert(state === 4)
  })

  it('can query for proof requests.', async () => {
    let connection = await Connection.create({ id: '234' })
    assert(connection)
    await connection.connect()
    let val = await DisclosedProof.get_requests(connection)
    assert(val)
  })
})
