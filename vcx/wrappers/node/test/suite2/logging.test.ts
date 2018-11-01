import 'module-resolver-helper'
// import * as ref from 'ref'

import { assert } from 'chai'
import { Logger } from 'src'
import { initRustAPI } from '../../src/rustlib'
import { errorMessage } from '../../src/utils/error-message'

describe.only('Logger:', () => {
  before(() => initRustAPI())
  it('success: Set Default Logger', async () => {
    const pattern = 'info'
    Logger.setDefaultLogger(pattern)
  })
  it('throws: VcxLoggerError when setDefaultLogger again', async () => {
    try {
      const pattern = 'info'
      Logger.setDefaultLogger(pattern)
    } catch (err) {
      assert(errorMessage(err) === errorMessage(1090))
    }
  })
})
