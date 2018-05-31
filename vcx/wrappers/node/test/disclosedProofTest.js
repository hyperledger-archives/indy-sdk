const assert = require('chai').assert
const vcx = require('../dist/index')
const { stubInitVCX, connectionCreateAndConnect } = require('./helpers')
const { DisclosedProof } = vcx

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
      "requested_attributes":{
        "email_1":{
          "name":"email"
        },
        "business_2":{
          "name":"business"
        },
        "name_0":{
          "name":"name"
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
    const connection = await connectionCreateAndConnect()
    const obj = await DisclosedProof.create({ connection, sourceId: 'Test', request: PROOF_REQ })
    assert(obj)
  })

  it(' can be created with a msgid.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await DisclosedProof.createWithMsgId({ connection, sourceId: 'Test', msgId: 'id' })
    assert(obj.proofRequest)
    assert(obj)
  })

  it(' a call to create with no sourceId returns an error', async () => {
    try {
      const connection = await connectionCreateAndConnect()
      await DisclosedProof.create({ request: PROOF_REQ, connection })
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it(' a call to create with no request returns an error', async () => {
    try {
      const connection = await connectionCreateAndConnect()
      await DisclosedProof.create({ connection, sourceId: 'Test' })
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it(' a call to create with a bad request returns an error', async () => {
    try {
      const connection = await connectionCreateAndConnect()
      await DisclosedProof.create({ connection, sourceId: 'Test', request: '{}' })
    } catch (error) {
      assert.equal(error.vcxCode, 1016)
    }
  })

  // serialize/deserialize tests

  it('can be serialized.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await DisclosedProof.create({ connection, sourceId: 'Test', request: PROOF_REQ })
    assert(obj)
    const val = await obj.serialize()
    assert(val)
  })

  it('can be deserialized.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await DisclosedProof.create({ connection, sourceId: 'Test', request: PROOF_REQ })
    assert(obj)
    const val = await obj.serialize()
    assert(val)
    const obj2 = await DisclosedProof.deserialize(val)
    assert(obj2)
  })

  // state tests

  it('can get state.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await DisclosedProof.create({ connection, sourceId: 'Test', request: PROOF_REQ })
    assert(obj)
    const state = await obj.getState()
    assert(state === 3)
  })

  it('can update state.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await DisclosedProof.create({ connection, sourceId: 'Test', request: PROOF_REQ })
    assert(obj)
    await obj.updateState()
    const state = await obj.getState()
    assert(state === 3)
  })

  // sendProof tests

  it('can send a proof.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await DisclosedProof.create({ connection, sourceId: 'Test', request: PROOF_REQ })
    assert(obj)
    await obj.sendProof(connection)
    const state = await obj.getState()
    assert(state === 4)
  })

  it('can query for proof requests.', async () => {
    const connection = await connectionCreateAndConnect()
    let val = await DisclosedProof.getRequests(connection)
    assert(val)
  })

  it('retrieve credentials associated with a proof request', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await DisclosedProof.create({ sourceId: 'Test', request: PROOF_REQ, connection })
    let creds = await obj.getCredentials()
    assert(JSON.stringify(creds) === `{"attrs":{"height_1":[{"cred_info":{"referent":"92556f60-d290-4b58-9a43-05c25aac214e","attrs":{"name":"Bob","height":"4'11","sex":"male","age":"111"},"schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4","cred_def_id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:2471","rev_reg_id":null,"cred_rev_id":null},"interval":null}],"zip_2":[{"cred_info":{"referent":"2dea21e2-1404-4f85-966f-d03f403aac71","attrs":{"address2":"101 Wilson Lane","city":"SLC","state":"UT","zip":"87121","address1":"101 Tela Lane"},"schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:Home Address:5.5.5","cred_def_id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:2479","rev_reg_id":null,"cred_rev_id":null},"interval":null}]},"predicates":{}}`)
  })

  it('generate a proof', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await DisclosedProof.create({ connection, sourceId: 'Test', request: PROOF_REQ })
    let retrievedCreds = await obj.getCredentials()
    let selectedCreds = {'height_1': retrievedCreds['attrs']['height_1'][0]}
    // Acception will be thrown if this doesn't work rather than undefined
    assert(await obj.generateProof({ selectedCreds, selfAttestedAttrs: retrievedCreds }) === undefined)
  })
})
