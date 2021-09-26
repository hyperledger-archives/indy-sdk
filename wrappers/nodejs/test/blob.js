const test = require('ava')
const indy = require('../')
const tempy = require('tempy')

test('blob_storage', async function (t) {
  const config = {
    base_dir: tempy.directory(),
    uri_pattern: ''
  }

  const wh = await indy.openBlobStorageWriter('default', config)
  t.is(typeof wh, 'number')
  t.truthy(wh >= 0)

  const rh = await indy.openBlobStorageReader('default', config)
  t.is(typeof rh, 'number')
  t.truthy(rh >= 0)

  let error = await t.throwsAsync(indy.openBlobStorageWriter('foo', config))
  t.is(error.indyName, 'CommonInvalidStructure')

  error = await t.throwsAsync(indy.openBlobStorageWriter('default', null))
  t.is(error.indyName, 'CommonInvalidParam3')

  error = await t.throwsAsync(indy.openBlobStorageReader('foo', config))
  t.is(error.indyName, 'CommonInvalidStructure')
})
