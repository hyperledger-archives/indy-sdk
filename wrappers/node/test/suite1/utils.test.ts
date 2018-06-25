import '../module-resolver-helper'

import { assert } from 'chai'
import { initVcxTestMode, shouldThrow } from 'helpers/utils'
import {
  getLedgerFees,
  getVersion,
  provisionAgent,
  updateAgentInfo,
  updateInstitutionConfigs,
  VCXCode
} from 'src'

describe('utils:', () => {
  before(() => initVcxTestMode())

  // tslint:disable-next-line max-line-length
  const provisionString = '{"agency_url":"https://enym-eagency.pdev.evernym.com","agency_did":"Ab8TvZa3Q19VNkQVzAWVL7","agency_verkey":"5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf","wallet_name":"test_provision_agent","agent_seed":null,"enterprise_seed":null,"wallet_key":"123"}'
  const agentUpdateString = '{"id":"123","value":"value"}'
  const updateInstitutionConfigsData = {
    logoUrl: 'https://google.com',
    name: 'New Name'
  }

  describe('provisionAgent:', () => {
    it('success', async () => {
      const res = await provisionAgent(provisionString)
      assert.ok(res)
    })

    it('throws: invalid input', async () => {
      const error = await shouldThrow(() => provisionAgent(''))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })
  })

  describe('updateAgentInfo:', () => {
    it('success', async () => {
      const res = await updateAgentInfo(agentUpdateString)
      assert.ok(res)
    })

    it('throws: invalid input', async () => {
      const error = await shouldThrow(() => updateAgentInfo(''))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })
  })

  describe('getVersion:', () => {
    it('success', async () => {
      const version = getVersion()
      assert.ok(version)
    })
  })

  describe('updateInstitutionConfigs:', () => {
    it('success', async () => {
      const res = updateInstitutionConfigs(updateInstitutionConfigsData)
      assert.equal(res, 0)
    })

    it('throws: missing name', async () => {
      const { name, ...data } = updateInstitutionConfigsData
      const error = await shouldThrow(() => updateInstitutionConfigs(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_CONFIGURATION)
    })

    it('throws: missing logoUrl', async () => {
      const { logoUrl, ...data } = updateInstitutionConfigsData
      const error = await shouldThrow(() => updateInstitutionConfigs(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_CONFIGURATION)
    })
  })

  describe('getLedgerFees:', () => {
    it('success', async () => {
      const fees = await getLedgerFees()
      assert.ok(fees)
    })
  })
})
