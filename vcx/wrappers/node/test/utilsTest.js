const assert = require('chai').assert
const vcx = require('../dist')
const { stubInitVCX, shouldThrow } = require('./helpers')

describe('provisioning and updating agents and updating configs', function () {
  this.timeout(10000)

  const provisionString = '{"agency_url":"https://enym-eagency.pdev.evernym.com","agency_did":"Ab8TvZa3Q19VNkQVzAWVL7","agency_verkey":"5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf","wallet_name":"test_provision_agent","agent_seed":null,"enterprise_seed":null,"wallet_key":null}'

  const agentUpdateString = '{"id":"123","value":"value"}'

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('a call to provisionAgent fails', async () => {
    const error = await shouldThrow(() => vcx.provisionAgent(''))
    assert.equal(error.vcxCode, 1007)
  })

  it('a call to provisionAgent succeeds', async () => {
    let obj = await vcx.provisionAgent(provisionString)
    assert(obj)
  })

  it('a call to updateAgentInfo fails', async() => {
    const error = await shouldThrow(() => vcx.updateAgentInfo(''))
    assert.equal(error.vcxCode, 1007)
  })

  it('a call to updateAgentInfo succeeds', async () => {
    let obj = await vcx.updateAgentInfo(agentUpdateString)
    assert(obj)
  })

  it('can get the version', () => {
    let version = vcx.getVersion()
    assert(version)
  })

  it('can update the name and logo for the config', () => {
    let rc = vcx.updateInstitutionConfigs('new name', 'http://www.google.com')
    assert.equal(rc, 0)
  })

  it('can retrieve ledger fees', async () => {
    let fees = await vcx.getLedgerFees()
    assert.equal(JSON.stringify(fees), '{"0":1,"1":1,"101":2,"102":42,"103":1999998889,"104":0,"105":0,"106":0,"107":0,"108":0,"109":0,"110":0,"111":0,"112":0,"113":0,"114":0,"115":0,"116":0,"117":0,"118":0,"119":0}')
  })
})
