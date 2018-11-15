import '../module-resolver-helper'

import { assert } from 'chai'
import { initVcxTestMode } from 'helpers/utils'
import { errorMessage } from '../../src/utils/error-message'

describe('errorMessage:', () => {
  before(() => initVcxTestMode())
  describe('error Message:', () => {
    it('success on error code 1090', async () => {
      assert.equal(errorMessage(1090), 'Logging Error')
    })
    it('gives correct error message when buffer size is too small', () => {
      assert.equal(errorMessage(1090, 1), 'Internal Vcx Error: Invalid buffer size')
    })
  })
})
