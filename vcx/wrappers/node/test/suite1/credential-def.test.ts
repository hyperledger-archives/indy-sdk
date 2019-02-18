import '../module-resolver-helper'

import { assert } from 'chai'
import { validatePaymentTxn } from 'helpers/asserts'
import { credentialDefCreate } from 'helpers/entities'
import { initVcxTestMode, shouldThrow } from 'helpers/utils'
import { CredentialDef, CredentialDefPaymentManager, VCXCode } from 'src'

describe('CredentialDef:', () => {
  before(() => initVcxTestMode())

  describe('create:', () => {
    it('success', async () => {
      await credentialDefCreate()
    })
  })

  describe('serialize:', () => {
    it('success', async () => {
      const credentialDef = await credentialDefCreate()
      const serialized = await credentialDef.serialize()
      assert.ok(serialized)
      assert.property(serialized, 'version')
      assert.property(serialized, 'data')
      const { data, version } = serialized
      assert.ok(data)
      assert.ok(version)
      assert.equal(data.source_id, credentialDef.sourceId)
    })

    it('throws: not initialized', async () => {
      const credentialDef = new CredentialDef(null as any, {} as any)
      const error = await shouldThrow(() => credentialDef.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CREDENTIAL_DEF_HANDLE)
    })

  })

  describe('deserialize:', () => {
    it('success', async () => {
      const credentialDef1 = await credentialDefCreate()
      const data1 = await credentialDef1.serialize()
      const credentialDef2 = await CredentialDef.deserialize(data1)
      assert.equal(credentialDef2.sourceId, credentialDef1.sourceId)
      const data2 = await credentialDef2.serialize()
      assert.deepEqual(data1, data2)
    })

    it('throws: incorrect data', async () => {
      const error = await shouldThrow(async () => CredentialDef.deserialize({ data: { source_id: 'Invalid' } } as any))
      assert.equal(error.vcxCode, VCXCode.CREATE_CREDENTIAL_DEF_ERR)
    })
  })

  describe('getCredDefId:', () => {
    it('success', async () => {
      const credentialDef = await credentialDefCreate()
      assert.equal(await credentialDef.getCredDefId(), '2hoqvcwupRTUNkXn6ArYzs:3:CL:2471')
    })

    it('throws: not initialized', async () => {
      const credentialDef = new CredentialDef(null as any, {} as any)
      const error = await shouldThrow(() => credentialDef.getCredDefId())
      assert.equal(error.vcxCode, VCXCode.INVALID_CREDENTIAL_DEF_HANDLE)
    })
  })

  describe('paymentManager:', () => {
    it('exists', async () => {
      const credentialDef = await credentialDefCreate()
      assert.instanceOf(credentialDef.paymentManager, CredentialDefPaymentManager)
      assert.equal(credentialDef.paymentManager.handle, credentialDef.handle)
    })

    describe('getPaymentTxn:', () => {
      it('success', async () => {
        const credentialDef = await credentialDefCreate()
        const paymentTxn = await credentialDef.paymentManager.getPaymentTxn()
        validatePaymentTxn(paymentTxn)
      })
    })
  })

})
