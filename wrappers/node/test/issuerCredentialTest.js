const assert = require('chai').assert
const expect = require('chai').expect
const ffi = require('ffi')
const vcx = require('../dist')
const { stubInitVCX, shouldThrow } = require('./helpers')

const { IssuerCredential, Connection, StateType, Error, rustAPI, VCXMock, VCXMockMessage } = vcx

const credentialConfigDefault = {
  sourceId: 'jsonCreation',
  schemaNum: 1234,
  issuerDid: 'arandomdidfoobar',
  attr: {
    key: 'value',
    key2: 'value2',
    key3: 'value3'
  },
  credentialName: 'Credential Name'
}
const connectionConfigDefault = {
  id: '123'
}
const formattedAttrs = {
  key: ['value'],
  key2: ['value2'],
  key3: ['value3']
}
const credentialDummyArgs = [
  'Dummy credential',
  {
    schemaNum: 1,
    issuerDid: 'arandomdidfoobar',
    credentialName: 'Credential Name'
  }
]
describe('An IssuerCredential', async function () {
  this.timeout(30000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('can be created.', async function () {
    const credential = new IssuerCredential(...credentialDummyArgs)
    assert(credential)
  })

  it('can have a source Id.', async function () {
    const credential = await new IssuerCredential(...credentialDummyArgs)
    assert.equal(credential.sourceId, credentialDummyArgs[0])
  })

  it('has a state of 0 after instanstiated', async function () {
    const credential = await new IssuerCredential(...credentialDummyArgs)
    const state = await credential.getState()
    assert.equal(state, 0)
  })

  it('has a credentialHandle and a sourceId after it is created', async function () {
    const sourceId = 'credential'
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    assert(credential.handle > 0)
    assert.equal(credential.sourceId, sourceId)
  })

  it('has state that can be found', async function () {
    const sourceId = 'TestState'
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    await credential.updateState()
    assert.equal(await credential.getState(), 1)
  })

  const sendCredentialOffer = async ({
    credentialConfig = credentialConfigDefault,
    connectionConfig = connectionConfigDefault
  } = {}) => {
    const connection = await Connection.create(connectionConfig)
    await connection.connect({ sms: true })
    const credential = await IssuerCredential.create(credentialConfig)
    await credential.sendOffer(connection)
    assert.equal(await credential.getState(), StateType.OfferSent)
    return {
      credential,
      connection
    }
  }
  it('can be sent with a valid connection', async function () {
    await sendCredentialOffer()
  })

  it('can be created, then serialized, then deserialized and have the same sourceId and state', async function () {
    const sourceId = 'SerializeDeserialize'
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    const jsonCredential = await credential.serialize()
    assert.equal(jsonCredential.state, StateType.Initialized)
    const credential2 = await IssuerCredential.deserialize(jsonCredential)
    assert.equal(await credential.getState(), await credential2.getState())
  })

  it('can be sent, then serialized, then deserialized', async function () {
    // create a connection, send the credential, serialize and then deserialize
    // and compare
    const connection = await Connection.create({ id: '234' })
    await connection.connect()

    const sourceId = 'SendSerializeDeserialize'
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })

    await credential.sendOffer(connection)
    const credentialData = await credential.serialize()

    const credential2 = await IssuerCredential.deserialize(credentialData)
    await credential.updateState()
    await credential2.updateState()
    assert.equal(await credential.getState(), StateType.OfferSent)
    assert.equal(await credential.getState(), await credential2.getState())
    assert.equal(credential.sourceId, credential2.sourceId)
  })

  it('serialize without correct handle throws error', async function () {
    const credential = new IssuerCredential(null, {})
    try {
      await credential.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, Error.INVALID_ISSUER_CREDENTIAL_HANDLE)
      assert.equal(error.message, 'Invalid Credential Issuer Handle')
    }
  })

  it('is created from a static method', async function () {
    const sourceId = 'staticMethodCreation'
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    assert(credential.sourceId, sourceId)
  })

  it('will have different credential handles even with the same sourceIds', async function () {
    const sourceId = 'sameSourceIds'
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    const credential2 = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    assert.notEqual(credential.handle, credential2.handle)
  })

  it('deserialize is a static method', async function () {
    const sourceId = 'deserializeStatic'
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    const serializedJson = await credential.serialize()

    const credentialDeserialized = await IssuerCredential.deserialize(serializedJson)
    assert.equal(await credentialDeserialized.getState(), StateType.Initialized)
  })

  it('accepts credential attributes and schema sequence number', async function () {
    const sourceId = 'attributesAndSequenceNumber'
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    assert.equal(credential.sourceId, sourceId)
    assert.equal(credential.schemaNum, credentialConfigDefault.schemaNum)
    assert.deepEqual(credential.attr, formattedAttrs)
  })

  it('throws exception for sending credential with invalid credential handle', async function () {
    let connection = await Connection.create(connectionConfigDefault)
    const credential = new IssuerCredential(null, {})
    try {
      await credential.sendCredential(connection)
    } catch (error) {
      assert.equal(error.vcxCode, Error.INVALID_ISSUER_CREDENTIAL_HANDLE)
      assert.equal(error.message, 'Invalid Credential Issuer Handle')
    }
  })

  it('throws exception for sending credential with invalid connection handle', async function () {
    let releasedConnection = await Connection.create({id: '123'})
    await releasedConnection.release()
    const sourceId = 'Credential'
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    try {
      await credential.sendCredential(releasedConnection)
    } catch (error) {
      assert.equal(error.vcxCode, Error.INVALID_CONNECTION_HANDLE)
      assert.equal(error.message, 'Invalid Connection Handle')
    }
  })

  it('sending credential with no credential offer should throw exception', async function () {
    let connection = await Connection.create({id: '123'})
    const sourceId = 'credential'
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    const error = await shouldThrow(() => credential.sendCredential(connection))
    assert.equal(error.vcxCode, Error.NOT_READY)
    assert.equal(error.vcxFunction, 'vcx_issuer_send_credential')
    assert.equal(error.message, 'Object not ready for specified action')
  })

  it('will throw error on serialize when issuer_credential has been released', async () => {
    const sourceId = 'SendSerializeDeserialize'
    const connection = await Connection.create({id: '123'})
    const credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    await credential.sendOffer(connection)
    try {
      await credential.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, 1015)
      assert.equal(error.vcxFunction, 'vcx_issuer_credential_serialize')
      assert.equal(error.message, 'Invalid Issuer Claim Handle')
    }
  })

  const acceptCredentialOffer = async ({ credential }) => {
    VCXMock.setVcxMock(VCXMockMessage.CredentialReq)
    VCXMock.setVcxMock(VCXMockMessage.UpdateCredential)
    await credential.updateState()
    const newState = await credential.getState()
    assert.equal(newState, StateType.RequestReceived)
  }
  it(`updating credential's state with mocked agent reply should return ${StateType.RequestReceived}`, async function () {
    const { credential } = await sendCredentialOffer()
    await acceptCredentialOffer({ credential })
  })

  const sendCredential = async ({ credential, connection }) => {
    await credential.sendCredential(connection)
    assert.equal(await credential.getState(), StateType.Accepted)
  }

  it('sending credential with valid credential offer should have state VcxStateAccepted', async function () {
    const { credential, connection } = await sendCredentialOffer()
    await acceptCredentialOffer({ credential })
    await sendCredential({ credential, connection })
  })

  it('can be created from a json', async function () {
    const credential = await IssuerCredential.create(credentialConfigDefault)
    expect(credential.sourceId).to.equal(credentialConfigDefault.sourceId)
  })

  const issuerCredentialOfferCheckAndDelete = async () => {
    let connection = await Connection.create({id: '123'})
    await connection.connect({ sms: true })
    const sourceId = 'credential'
    let credential = await IssuerCredential.create({ ...credentialConfigDefault, sourceId })
    await credential.sendOffer(connection)
    const serialize = rustAPI().vcx_issuer_credential_serialize
    const handle = credential._handle
    connection = null
    credential = null
    return {
      handle,
      serialize
    }
  }

  // Fix the GC issue
  it('issuer_credential and GC deletes object should return null when serialize is called ', async function () {
    this.timeout(30000)

    const { handle, serialize } = await issuerCredentialOfferCheckAndDelete()

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

        if (rc === Error.INVALID_ISSUER_CREDENTIAL_HANDLE) {
          resolve(null)
        }
      })
      if (!data) {
        isComplete = true
      }
    }

    // this will timeout if condition is never met
    // get_data will return "" because the connection object was released
    return isComplete
  })
})
