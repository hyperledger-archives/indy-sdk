const assert = require('chai').assert
const vcx = require('../dist')
const { stubInitVCX } = require('./helpers')

describe('provisioning and updating agents ', function () {
  this.timeout(10000)

  const provisionString = '{"agency_url":"https://enym-eagency.pdev.evernym.com","agency_did":"Ab8TvZa3Q19VNkQVzAWVL7","agency_verkey":"5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf","wallet_name":"test_provision_agent","agent_seed":null,"enterprise_seed":null,"wallet_key":null}'

  const agentUpdateString = '{"id":"123","value":"value"}'

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('a call to provisionAgent fails', async () => {
    try {
      await vcx.provisionAgent('')
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it('a call to provisionAgent succeeds', async () => {
    let obj = await vcx.provisionAgent(provisionString)
    assert(obj)
  })

  it('a call to updateAgentInfo fails', async() => {
    try {
      await vcx.updateAgentInfo('')
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it('a call to updateAgentInfo succeeds', async () => {
    let obj = await vcx.updateAgentInfo(agentUpdateString)
    assert(obj)
  })

  it('can get the version', () => {
    let version = vcx.getVersion()
    assert(version)
  })
})
