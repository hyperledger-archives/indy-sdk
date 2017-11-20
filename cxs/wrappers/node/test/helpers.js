const { stub } = require('sinon')

const cxs = require('../dist')

let _initCXSCalled = false
let _spyInitCXS
const _stubInitCXS = () => {
  const initCXSOriginal = cxs.initCxs
  const stubInitCXS = stub(cxs, 'initCxs')
  stubInitCXS.callsFake(async function (...args) {
    if (_initCXSCalled) {
      console.log('calling a stub -> already called')
      return
    }
    console.log('calling a stub -> calling original')
    await initCXSOriginal(...args)
    _initCXSCalled = true
  })
  return stubInitCXS
}
const stubInitCXS = () => {
  if (!_spyInitCXS) {
    _spyInitCXS = _stubInitCXS()
  }
  return _spyInitCXS
}

const shouldThrow = (fn) => new Promise(async (resolve, reject) => {
  try {
    await fn()
    reject(new Error(`${fn.toSting()} should have thrown!`))
  } catch (e) {
    resolve(e)
  }
})

module.exports = { stubInitCXS, shouldThrow }
