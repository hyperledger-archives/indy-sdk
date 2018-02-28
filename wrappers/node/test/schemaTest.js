const assert = require('chai').assert
const vcx = require('../dist/index')
const { stubInitVCX } = require('./helpers')
const { Schema, Error } = vcx

const SCHEMA = {
  sourceId: 'sourceId',
  data: {
    name: 'data name',
    version: '1.1.1',
    attrNames: ['attr1', 'attr2', 'height', 'weight']
  }
}

describe('A Shema', function () {
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
    assert.equal(schema.handle, schema2.handle)
    assert.equal(schema.name, schema2.name)
    assert.equal(schema.source_id, schema2.source_id)
    assert.equal(schema.getSchemaNo, schema2.getSchemaNo)
    // The attrNames are mocked values returned from sdk
    assert.equal(JSON.stringify(schema2.getSchemaAttrs()),
    JSON.stringify({name: 'name', version: '1.0', attrNames: ['name', 'male']}))
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

  it('will return schemaNo greater than 0 when getSchemaNo is called', async () => {
    const schema = await Schema.create(SCHEMA)
    const schemaNo = await schema.getSchemaNo()
    assert(schemaNo)
  })

  it('can be retrieved by calling lookup', async () => {
    const schema = await Schema.lookup({sourceId: 'test lookup', seqNo: 999})
    // The attrNames are mocked values returned from sdk
    assert.equal(JSON.stringify(schema.getSchemaAttrs()),
    JSON.stringify({name: 'get schema attrs', version: '1.0', attrNames: ['test', 'get', 'schema', 'attrs']}))
  })

  it('can be retrieved by calling lookup and then serialized', async () => {
    const schema = await Schema.lookup({sourceId: '1cda', seqNo: 58})
    // The attrNames are mocked values returned from sdk
    assert.equal(JSON.stringify(schema.getSchemaAttrs()),
    JSON.stringify({name: 'get schema attrs', version: '1.0', attrNames: ['test', 'get', 'schema', 'attrs']}))
    const serializedLookup = await schema.serialize()
    assert.equal(schema.handle, serializedLookup.handle)
    const deserializedSchema = await Schema.deserialize(serializedLookup)
    assert(schema)
    assert.equal(schema.handle, deserializedSchema.handle)
  })
})
