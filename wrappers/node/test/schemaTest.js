const assert = require('chai').assert
const ffi = require('ffi')
const vcx = require('../dist/index')
const { stubInitVCX } = require('./helpers')
const { Schema, Error, rustAPI } = vcx

const SCHEMA = {
  sourceId: 'sourceId',
  data: {
    name: 'data name',
    version: '1.1.1',
    attrNames: ['attr1', 'attr2', 'height', 'weight']
  },
  paymentHandle: 0
}

describe('A Schema', function () {
  this.timeout(30000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('can be created.', async () => {
    const schema = await Schema.create(SCHEMA)
    assert(schema)
  })

  it('can retrieve schema attrs', async () => {
    const schema = await Schema.create(SCHEMA)
    assert(schema)
    assert.equal(schema.getSchemaAttrs(), SCHEMA.data)
  })

  it('has a name of data name after instanstiated', async () => {
    const schema = await Schema.create(SCHEMA)
    const name = await schema._name
    assert.equal(name, 'data name')
  })

  it('can be created, then serialized, then deserialized and have the same sourceId, name, and handle', async () => {
    const schema = await Schema.create(SCHEMA)
    const jsonDef = await schema.serialize()
    assert.equal(jsonDef.source_id, 'sourceId')
    const schema2 = await Schema.deserialize(jsonDef)
    assert.equal(schema.name, schema2.name)
    assert.equal(schema.source_id, schema2.source_id)
    assert.equal(schema.getSchemaId, schema2.getSchemaId)
    // The attrNames are mocked values returned from sdk
    assert.equal(JSON.stringify(schema2.getSchemaAttrs()),
    JSON.stringify({name: 'data name', version: '1.1.1', attrNames: ['attr1', 'attr2', 'height', 'weight']}))
  })

  it('will throw error on serialize when schema has been released', async () => {
    const schema = await Schema.create(SCHEMA)
    const jsonDef = await schema.serialize()
    const data = await schema.serialize()
    assert(data)
    assert.equal(data.handle, jsonDef.handle)
    assert.equal(await schema.release(), Error.SUCCESS)
    try {
      await schema.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, 1042)
      assert.equal(error.vcxFunction, 'vcx_schema_serialize')
      assert.equal(error.message, 'Invalid Schema Handle')
    }
  })

  it('will return schema_id', async () => {
    const schema = await Schema.create(SCHEMA)
    assert(await schema.getSchemaId())
  })

  it('can be retrieved by calling lookup', async () => {
    const schema = await Schema.lookup({sourceId: 'test lookup', schemaId: 'schema_id1'})
    // The attrNames are mocked values returned from sdk
    assert.equal(
      JSON.stringify(schema.schemaAttrs),
      JSON.stringify({name: 'test-licence', version: '4.4.4', attrNames: ['height', 'name', 'sex', 'age']})
    )
  })

  it('can be retrieved by calling lookup and then serialized', async () => {
    const schema = await Schema.lookup({sourceId: '1cda', schemaId: 'schema_id123'})
    // The attrNames are mocked values returned from sdk
    assert.equal(
      JSON.stringify(schema.schemaAttrs),
      JSON.stringify({name: 'test-licence', version: '4.4.4', attrNames: ['height', 'name', 'sex', 'age']})
    )
    const serializedLookup = await schema.serialize()
    assert.equal(schema.sourceId, serializedLookup.source_id)
    const deserializedSchema = await Schema.deserialize(serializedLookup)
    assert(schema)
    assert.equal(schema.sourceId, deserializedSchema.sourceId)
  })

  const schemaCreateCheckAndDelete = async () => {
    let schema = await Schema.create(SCHEMA)
    const data = await schema.serialize()
    assert(data)
    const serialize = rustAPI().vcx_schema_serialize
    const handle = schema._handle
    schema = null
    return {
      handle,
      serialize
    }
  }

  // Fix the GC issue
  it('schema and GC deletes object should return null when serialize is called ', async function () {
    this.timeout(30000)

    const { handle, serialize } = await schemaCreateCheckAndDelete()

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

        if (rc === 1042) {
          resolve(null)
        }
      })
      if (!data) {
        isComplete = true
      }
    }

    // this will timeout if condition is never met
    // ill return "" because the schema object was released
    return isComplete
  })
})
