const assert = require('chai').assert
const ffi = require('ffi')
const vcx = require('../dist/index')
const { stubInitVCX, connectionCreateAndConnect } = require('./helpers')
const { Credential, rustAPI } = vcx

describe('A Credential', function () {
  this.timeout(30000)

  const OFFER = {
    msg_type: 'CLAIM_OFFER',
    version: '0.1',
    to_did: '8XFh8yBzrpJQmNyZzgoTqB',
    from_did: '8XFh8yBzrpJQmNyZzgoTqB',
    libindy_offer: '{}',
    cred_def_id: 'id',
    credential_attrs: {
      address1: ['101 Tela Lane'],
      address2: ['101 Wilson Lane'],
      city: ['SLC'],
      state: ['UT'],
      zip: ['87121']
    },
    schema_seq_no: 1487,
    claim_name: 'Credential',
    claim_id: 'defaultCredentialId',
    msg_ref_id: null
  }

  const SERIALIZED_CREDENTIAL = {
    source_id: 'wrapper_tests',
    state: 3,
    credential_name: null,
    credential_request: null,
    credential_offer: {
      msg_type: 'CLAIM_OFFER',
      version: '0.1',
      to_did: '8XFh8yBzrpJQmNyZzgoTqB',
      from_did: '8XFh8yBzrpJQmNyZzgoTqB',
      libindy_offer: '{}',
      cred_def_id: 'id',
      credential_attrs: {
        address1: ['101 Tela Lane'],
        address2: ['101 Wilson Lane'],
        city: ['SLC'],
        state: ['UT'],
        zip: ['87121']
      },
      schema_seq_no: 1487,
      claim_name: 'Credential',
      claim_id: 'defaultCredentialId',
      msg_ref_id: '123'
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
    const connection = await connectionCreateAndConnect()
    const obj = await Credential.create({ sourceId: 'Test', offer: JSON.stringify(OFFER), connection })
    assert(obj)
  })

  it(' can be created with a msgid.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await Credential.createWithMsgId({ connection, sourceId: 'Test', msgId: 'id' })
    assert(obj)
    assert(obj.credOffer)
  })

  // create tests

  it(' a call to create with no sourceId returns an error', async () => {
    try {
      const connection = await connectionCreateAndConnect()
      await Credential.create({ offer: JSON.stringify(OFFER), connection })
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it(' a call to create with no offer returns an error', async () => {
    try {
      const connection = await connectionCreateAndConnect()
      await Credential.create({ connection, sourceId: 'Test' })
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it(' a call to create with a bad offer returns an error', async () => {
    try {
      const connection = await connectionCreateAndConnect()
      await Credential.create({ connection, sourceId: 'Test', offer: '{}' })
    } catch (error) {
      assert.equal(error.vcxCode, 1016)
    }
  })

  // serialize/deserialize tests

  it('can be serialized.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await Credential.create({ connection, sourceId: 'Test', offer: JSON.stringify(OFFER) })
    assert(obj)
    const val = await obj.serialize()
    assert(val)
  })

  it('can be deserialized.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await Credential.create({ sourceId: 'Test', offer: JSON.stringify(OFFER), connection })
    assert(obj)
    const val = await obj.serialize()
    assert(val)
    const obj2 = await Credential.deserialize(val)
    assert(obj2)
  })

  // state tests

  it('can get state.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await Credential.create({ connection, sourceId: 'Test', offer: JSON.stringify(OFFER) })
    assert(obj)
    const state = await obj.getState()
    assert(state === 3)
  })

  it('can update state.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await Credential.create({ connection, sourceId: 'Test', offer: JSON.stringify(OFFER) })
    assert(obj)
    await obj.updateState()
    const state = await obj.getState()
    assert(state === 3)
  })

  // sendRequest tests

  it('can send a credential request.', async () => {
    const connection = await connectionCreateAndConnect()
    const obj = await Credential.deserialize(SERIALIZED_CREDENTIAL)
    console.log(1)
    await obj.sendRequest(connection)
    console.log(2)
    const state = await obj.getState()
    assert(state === 2)
  })

  it('can query for credential offers.', async () => {
    const connection = await connectionCreateAndConnect()
    let val = await Credential.getOffers(connection)
    assert(val)
  })

  const credentialCreateCheckAndDelete = async () => {
    const connection = await connectionCreateAndConnect()
    let credential = await Credential.create({ connection, sourceId: 'Test', offer: JSON.stringify(OFFER) })
    assert(credential)
    const val = await credential.serialize()
    assert(val)
    const serialize = rustAPI().vcx_credential_serialize
    const handle = credential._handle
    credential = null
    return {
      handle,
      serialize
    }
  }

  // Fix the GC issue
  it('credential and GC deletes object should return null when serialize is called ', async function () {
    this.timeout(30000)

    const { handle, serialize } = await credentialCreateCheckAndDelete()

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
