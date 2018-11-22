import '../module-resolver-helper'

import { VCX_CONFIG_TEST_MODE } from 'helpers/test-constants'
import { SinonStub, stub } from 'sinon'
import * as vcx from 'src'

let _initVCXCalled = false
let _patchInitVCXObj: SinonStub | undefined
const _patchInitVCX = () => {
  const initVCXOriginal = vcx.initVcx as any
  const stubInitVCX = stub(vcx, 'initVcx')
  // tslint:disable-next-line only-arrow-functions
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
export const patchInitVCX = () => {
  if (!_patchInitVCXObj) {
    _patchInitVCXObj = _patchInitVCX()
  }
  return _patchInitVCXObj
}

export const initVcxTestMode = async () => {
  patchInitVCX()
  await vcx.initVcx(VCX_CONFIG_TEST_MODE)
}

export const shouldThrow = (fn: () => any): Promise<vcx.VCXInternalError> => new Promise(async (resolve, reject) => {
  try {
    await fn()
    reject(new Error(`${fn.toString()} should have thrown!`))
  } catch (e) {
    resolve(e)
  }
})

export const sleep = (timeout: number) => new Promise((resolve, reject) => setTimeout(resolve, timeout))
