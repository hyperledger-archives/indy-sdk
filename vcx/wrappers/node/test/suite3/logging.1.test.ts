import '../module-resolver-helper'

import { assert } from 'chai'
import { defaultLogger } from 'src'
import { initRustAPI } from '../../src/rustlib'
import { errorMessage } from '../../src/utils/error-message'

describe('Logger:', () => {
  before(() => initRustAPI())
  it('success: Set Default Logger', async () => {
    const pattern = 'info'
    defaultLogger(pattern)
  })
  it('throws: VcxLoggerError when setDefaultLogger again', async () => {
    try {
      const pattern = 'info'
      defaultLogger(pattern)
    } catch (err) {
      assert(errorMessage(err) === errorMessage(1090))
    }
  })
})
