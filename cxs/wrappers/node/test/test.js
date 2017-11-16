const { assert } = require('chai')
const { initCxs } = require('../dist/index')
const { issuerClaimTests } = require('./issuerClaimTests')
const { proofTests } = require('./proof-test')
const { issuerClaimFfiTests } = require('./issuerClaimFfiTests.js')
const { connectionTests } = require('./connectionTests')

describe('Running tests', async function () {
  this.timeout(10000)
  before(async function () {
    await initCxs('ENABLE_TEST_MODE')
  })
  it('initCxs can only be called once', async function () {
    let onlyOnce = async function () {
      try {
        await initCxs('ENABLE_TEST_MODE')
        return 1
      } catch (error) {
        return 0
      }
    }
    assert.equal(await onlyOnce(), 0)
  })

  it('Issuer Claim tests', issuerClaimTests)
  it('Proof Tests', proofTests)
  it('Issuer Claim Ffi Tests', issuerClaimFfiTests)
  it('Connection Tests', connectionTests)
})
