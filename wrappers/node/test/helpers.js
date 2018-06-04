const { stub } = require('sinon')
const assert = require('chai').assert
const vcx = require('../dist')
const { Connection } = vcx

let _initVCXCalled = false
let _spyInitVCX
const _stubInitVCX = () => {
  const initVCXOriginal = vcx.initVcx
  const stubInitVCX = stub(vcx, 'initVcx')
  stubInitVCX.callsFake(async function (...args) {
    if (_initVCXCalled) {
      console.log('calling a stub -> already called')
      return
    }
    console.log('calling a stub -> calling original')
    await initVCXOriginal(...args)
    _initVCXCalled = true
  })
  return stubInitVCX
}
const stubInitVCX = () => {
  if (!_spyInitVCX) {
    _spyInitVCX = _stubInitVCX()
  }
  return _spyInitVCX
}

const shouldThrow = (fn) => new Promise(async (resolve, reject) => {
  try {
    await fn()
    reject(new Error(`${fn.toString()} should have thrown!`))
  } catch (e) {
    resolve(e)
  }
})

const connectionCreateAndConnect = async () => {
  let connection = await Connection.create({ id: '234' })
  assert(connection)
  await connection.connect()
  return connection
}

module.exports = { stubInitVCX, shouldThrow, connectionCreateAndConnect }
