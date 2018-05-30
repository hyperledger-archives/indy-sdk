const assert = require('chai').assert
const ffi = require('ffi')
const vcx = require('../dist/index')
const { stubInitVCX } = require('./helpers')
const { CredentialDef, Error, rustAPI } = vcx

const CREDENTIAL_DEF = {
  name: 'test',
  revocation: false,
  schemaId: 'schema id1',
  sourceId: 'sourceId'
}

describe('A CredentialDef', function () {
  this.timeout(30000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('can be created.', async () => {
    const credentialDef = await CredentialDef.create(CREDENTIAL_DEF)
    assert(credentialDef)
  })

  it('has a name of test after instanstiated', async () => {
    const credentialDef = await CredentialDef.create(CREDENTIAL_DEF)
    const name = await credentialDef._name
    assert.equal(name, 'test')
  })

  it('can be created, then serialized, then deserialized and have the same sourceId and name', async () => {
    const credentialDef = await CredentialDef.create(CREDENTIAL_DEF)
    const jsonDef = await credentialDef.serialize()
    assert.equal(jsonDef.source_id, 'sourceId')
    const credentialDef2 = await CredentialDef.deserialize(jsonDef)
    assert.equal(credentialDef.name, credentialDef2.name)
    assert.equal(credentialDef.source_id, credentialDef2.source_id)
  })

  it('will throw error on serialize when credentialDef has been released', async () => {
    const credentialDef = await CredentialDef.create(CREDENTIAL_DEF)
    const jsonDef = await credentialDef.serialize()
    let data = await credentialDef.serialize()
    assert(data)
    assert.equal(data.handle, jsonDef.handle)
    assert.equal(await credentialDef.release(), Error.SUCCESS)
    try {
      await credentialDef.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, 1037)
      assert.equal(error.vcxFunction, 'vcx_credentialdef_serialize')
      assert.equal(error.message, 'Invalid Credential Definition handle')
    }
  })

  it('will return cred_def_id', async () => {
    const credentialDef = await CredentialDef.create(CREDENTIAL_DEF)
    assert(await credentialDef.getCredDefId(), '2hoqvcwupRTUNkXn6ArYzs:3:CL:1766')
  })

  const credentialDefCreateCheckAndDelete = async () => {
    let credentialDef = await CredentialDef.create(CREDENTIAL_DEF)
    let data = await credentialDef.serialize()
    assert(data)
    const serialize = rustAPI().vcx_credentialdef_serialize
    const handle = credentialDef._handle
    credentialDef = null
    return {
      handle,
      serialize
    }
  }

  // Fix the GC issue
  it('credentialdef and GC deletes object should return null when serialize is called ', async function () {
    this.timeout(30000)

    const { handle, serialize } = await credentialDefCreateCheckAndDelete()

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

        if (rc === 1037) {
          resolve(null)
        }
      })
      if (!data) {
        isComplete = true
      }
    }

    // this will timeout if condition is never met
    // ill return "" because the credentialdef object was released
    return isComplete
  })
})
