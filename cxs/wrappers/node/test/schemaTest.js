const assert = require('chai').assert
const cxs = require('../dist/index')
const { stubInitCXS } = require('./helpers')
const { Schema, Error } = cxs

const SCHEMA = {
  name: 'test',
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
    stubInitCXS()
    await cxs.initCxs('ENABLE_TEST_MODE')
  })

  it('can be created.', async () => {
    const schema = await Schema.create(SCHEMA)
    assert(schema)
  })

  it('has a name of test after instanstiated', async () => {
    const schema = await Schema.create(SCHEMA)
    const name = await schema._name
    assert.equal(name, 'test')
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
      assert.equal(error.toString(), 'Error: cxs_schema_serialize -> 1042')
    }
  })

  it('will return schemaNo greater than 0 when getSchemaNo is called', async () => {
    const schema = await Schema.create(SCHEMA)
    const schemaNo = await schema.getSchemaNo()
    assert(schemaNo)
  })
})
