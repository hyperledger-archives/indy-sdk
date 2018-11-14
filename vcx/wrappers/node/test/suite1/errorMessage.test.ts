import '../module-resolver-helper'

import { assert } from 'chai'
import { initVcxTestMode } from 'helpers/utils'
import { errorMessage } from '../../src/utils/error-message'

describe.only('error message:', () => {
    before(() => initVcxTestMode())
    describe('error Message:', () => {
        it('success', async () => {
            assert.equal(errorMessage(1090), "Logging Error")
        })
    })
})