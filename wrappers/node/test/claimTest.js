const assert = require('chai').assert
const ffi = require('ffi')
const vcx = require('../dist/index')
const { stubInitVCX } = require('./helpers')
const { Claim, Connection, rustAPI } = vcx

describe('A Claim', function () {
  this.timeout(30000)

  const OFFER = {
    msg_type: 'CLAIM_OFFER',
    version: '0.1',
    to_did: 'LtMgSjtFcyPwenK9SHCyb8',
    from_did: 'LtMgSjtFcyPwenK9SHCyb8',
    claim: {
      account_num: [
        '8BEaoLf8TBmK4BUyX8WWnA'
      ],
      name_on_account: [
        'Alice'
      ]
    },
    schema_seq_no: 48,
    issuer_did: 'Pd4fnFtRBcMKRVC2go5w3j',
    claim_name: 'Account Certificate',
    claim_id: '3675417066',
    msg_ref_id: null
  }

  const SERIALIZED_CLAIM = {
    source_id: 'wrapper_tests',
    state: 3,
    claim_name: null,
    claim_request: null,
    claim_offer: {
      msg_type: 'CLAIM_OFFER',
      version: '0.1',
      to_did: 'LtMgSjtFcyPwenK9SHCyb8',
      from_did: 'LtMgSjtFcyPwenK9SHCyb8',
      claim: {'account_num': ['8BEaoLf8TBmK4BUyX8WWnA'], 'name_on_account': ['Alice']},
      schema_seq_no: 48,
      issuer_did: 'Pd4fnFtRBcMKRVC2go5w3j',
      claim_name: 'Account Certificate',
      claim_id: '3675417066',
      msg_ref_id: 'ymy5nth'
    },
    link_secret_alias: 'main',
    msg_uid: null,
    agent_did: null,
    agent_vk: null,
    my_did: null,
    my_vk: null,
    their_did: null,
    their_vk: null
  }

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('can be created.', async () => {
    const obj = await Claim.create({sourceId: 'Test', offer: JSON.stringify(OFFER)})
    assert(obj)
  })

  // create tests

  it(' a call to create with no sourceId returns an error', async () => {
    try {
      await Claim.create({offer: JSON.stringify(OFFER)})
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it(' a call to create with no offer returns an error', async () => {
    try {
      await Claim.create({sourceId: 'Test'})
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it(' a call to create with a bad offer returns an error', async () => {
    try {
      await Claim.create({sourceId: 'Test', offer: '{}'})
    } catch (error) {
      assert.equal(error.vcxCode, 1016)
    }
  })

  // serialize/deserialize tests

  it('can be serialized.', async () => {
    const obj = await Claim.create({sourceId: 'Test', offer: JSON.stringify(OFFER)})
    assert(obj)
    const val = await obj.serialize()
    assert(val)
  })

  it('can be deserialized.', async () => {
    const obj = await Claim.create({sourceId: 'Test', offer: JSON.stringify(OFFER)})
    assert(obj)
    const val = await obj.serialize()
    assert(val)
    const obj2 = await Claim.deserialize(val)
    assert(obj2)
  })

  // state tests

  it('can get state.', async () => {
    const obj = await Claim.create({sourceId: 'Test', offer: JSON.stringify(OFFER)})
    assert(obj)
    const state = await obj.getState()
    assert(state === 3)
  })

  it('can update state.', async () => {
    const obj = await Claim.create({sourceId: 'Test', offer: JSON.stringify(OFFER)})
    assert(obj)
    await obj.updateState()
    const state = await obj.getState()
    assert(state === 3)
  })

  // sendRequest tests

  it('can send a claim request.', async () => {
    let connection = await Connection.create({ id: '234' })
    assert(connection)
    await connection.connect()
    const obj = await Claim.deserialize(SERIALIZED_CLAIM)
    await obj.sendRequest(connection)
    const state = await obj.getState()
    assert(state === 2)
  })

  it('can query for claim offers.', async () => {
    let connection = await Connection.create({ id: '234' })
    assert(connection)
    await connection.connect()
    let val = await Claim.get_offers(connection)
    assert(val)
  })

  const claimCreateCheckAndDelete = async () => {
    let claim = await Claim.create({sourceId: 'Test', offer: JSON.stringify(OFFER)})
    assert(claim)
    const val = await claim.serialize()
    assert(val)
    const serialize = rustAPI().vcx_claim_serialize
    const handle = claim._handle
    claim = null
    return {
      handle,
      serialize
    }
  }

  // Fix the GC issue
  it('claim and GC deletes object should return null when serialize is called ', async function () {
    this.timeout(30000)

    const { handle, serialize } = await claimCreateCheckAndDelete()

    global.gc()

    let isComplete = false
    //  hold on to callbacks so it doesn't become garbage collected
    const callbacks = []

    while (!isComplete) {
      const data = await new Promise(function (resolve, reject) {
        const callback = ffi.Callback('void', ['uint32', 'uint32', 'string'],
            function (handle, err, data) {
              if (err) {
                reject(err)
                return
              }
              resolve(data)
            })
        callbacks.push(callback)
        const rc = serialize(
            0,
            handle,
            callback
        )

        if (rc === 1053) {
          resolve(null)
        }
      })
      if (!data) {
        isComplete = true
      }
    }

    // this will timeout if condition is never met
    // ill return "" because the proof object was released
    return isComplete
  })
})
