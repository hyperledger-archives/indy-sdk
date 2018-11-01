import * as ffi from 'ffi'
import 'module-resolver-helper'

import { assert } from 'chai'
import { Logger } from 'src'
import { initRustAPI } from '../../src/rustlib'
import { errorMessage } from '../../src/utils/error-message'

describe('Logger:', () => {
  before(() => initRustAPI())
  it('success: Set Logger', () => {
    let nEntries = 0
    const logFn = ffi.Callback('void', ['pointer', 'uint32', 'string', 'string', 'string', 'string', 'uint32'],
     (_context: any, level: number, target: string, message: string ,
      modulePath: string, file: string, line: number) => {
      nEntries++
      assert(typeof level, 'number')
      assert(typeof target, 'string')
      assert(typeof message, 'string')
      assert(typeof modulePath, 'string')
      assert(typeof file, 'string')
      assert(typeof line, 'number')
    })
    Logger.setLogger(logFn)
    assert(nEntries === 0)
    errorMessage(1090)
    assert(nEntries > 0)
  })
})
