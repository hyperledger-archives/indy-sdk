import '../module-resolver-helper'

import { assert } from 'chai'
import { validatePaymentTxn } from 'helpers/asserts'
import {
  connectionCreateConnect,
  dataIssuerCredentialCreate,
  issuerCredentialCreate
} from 'helpers/entities'
import { gcTest } from 'helpers/gc'
import { TIMEOUT_GC } from 'helpers/test-constants'
import { initVcxTestMode, shouldThrow } from 'helpers/utils'
import { Connection, IssuerCredential, rustAPI, StateType, VCXCode, VCXMock, VCXMockMessage } from 'src'

describe('IssuerCredential:', () => {
  before(() => initVcxTestMode())

  describe('create:', () => {
    it('success', async () => {
      await issuerCredentialCreate()
    })

    it('throws: missing sourceId', async () => {
      const { sourceId, ...data } = dataIssuerCredentialCreate()
      const error = await shouldThrow(() => IssuerCredential.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing credDefId', async () => {
      const { credDefId, ...data } = dataIssuerCredentialCreate()
      const error = await shouldThrow(() => IssuerCredential.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing credDefId', async () => {
      const { credDefId, ...data } = dataIssuerCredentialCreate()
      const error = await shouldThrow(() => IssuerCredential.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing attr', async () => {
      const { attr, ...data } = dataIssuerCredentialCreate()
      const error = await shouldThrow(() => IssuerCredential.create(data as any))
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
    })

    it('throws: missing credentialName', async () => {
      const { credentialName, ...data } = dataIssuerCredentialCreate()
      const error = await shouldThrow(() => IssuerCredential.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    // TODO: Enable once https://evernym.atlassian.net/browse/EN-665 is resolved
    it.skip('throws: missing price', async () => {
      const { price, ...data } = dataIssuerCredentialCreate()
      const error = await shouldThrow(() => IssuerCredential.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: invalid attr', async () => {
      const { attr, ...data } = dataIssuerCredentialCreate()
      const error = await shouldThrow(() => IssuerCredential.create({ attr: null as any, ...data }))
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
    })
  })

  describe('serialize:', () => {
    it('success', async () => {
      const issuerCredential = await issuerCredentialCreate()
      const data = await issuerCredential.serialize()
      assert.ok(data)
      assert.equal(data.source_id, issuerCredential.sourceId)
    })

    it('throws: not initialized', async () => {
      const issuerCredential = new IssuerCredential(null as any, {} as any)
      const error = await shouldThrow(() => issuerCredential.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_ISSUER_CREDENTIAL_HANDLE)
    })

    it('throws: issuerCredential released', async () => {
      const issuerCredential = await issuerCredentialCreate()
      const data = await issuerCredential.serialize()
      assert.ok(data)
      assert.equal(data.source_id, issuerCredential.sourceId)
      assert.equal(await issuerCredential.release(), VCXCode.SUCCESS)
      const error = await shouldThrow(() => issuerCredential.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_ISSUER_CREDENTIAL_HANDLE)
    })
  })

  describe('deserialize:', () => {
    it('success', async () => {
      const issuerCredential1 = await issuerCredentialCreate()
      const data1 = await issuerCredential1.serialize()
      const issuerCredential2 = await IssuerCredential.deserialize(data1)
      assert.equal(issuerCredential2.sourceId, issuerCredential1.sourceId)
      const data2 = await issuerCredential2.serialize()
      assert.deepEqual(data1, data2)
    })

    it('throws: incorrect data', async () => {
      const error = await shouldThrow(async () => IssuerCredential.deserialize({ source_id: 'Invalid' } as any))
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
    })

    it('throws: incomplete data', async () => {
      const error = await shouldThrow(async () => IssuerCredential.deserialize({
        cred_def_id: 'Invalid',
        credential_attributes: '{}',
        credential_name: 'Invalid',
        price: 'Invalid',
        source_id: 'Invalid'
      } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
    })
  })

  describe('release:', () => {
    it('success', async () => {
      const issuerCredential = await issuerCredentialCreate()
      assert.equal(await issuerCredential.release(), VCXCode.SUCCESS)
      const errorSerialize = await shouldThrow(() => issuerCredential.serialize())
      assert.equal(errorSerialize.vcxCode, VCXCode.INVALID_ISSUER_CREDENTIAL_HANDLE)
    })

    // TODO: Enable once https://evernym.atlassian.net/browse/EN-668 is resolved
    it.skip('throws: not initialized', async () => {
      const issuerCredential = new IssuerCredential(null as any, {} as any)
      const error = await shouldThrow(() => issuerCredential.release())
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
    })
  })

  describe('updateState:', () => {
    it(`returns ${StateType.None}: not initialized`, async () => {
      const issuerCredential = new IssuerCredential(null as any, {} as any)
      await issuerCredential.updateState()
      assert.equal(await issuerCredential.getState(), StateType.None)
    })

    it(`returns ${StateType.Initialized}: created`, async () => {
      const issuerCredential = await issuerCredentialCreate()
      await issuerCredential.updateState()
      assert.equal(await issuerCredential.getState(), StateType.Initialized)
    })
  })

  describe('sendOffer:', () => {
    it('success', async () => {
      const connection = await connectionCreateConnect()
      const issuerCredential = await issuerCredentialCreate()
      await issuerCredential.sendOffer(connection)
      assert.equal(await issuerCredential.getState(), StateType.OfferSent)
    })

    it('throws: not initialized', async () => {
      const connection = await connectionCreateConnect()
      const issuerCredential = new IssuerCredential(null as any, {} as any)
      const error = await shouldThrow(() => issuerCredential.sendOffer(connection))
      assert.equal(error.vcxCode, VCXCode.INVALID_ISSUER_CREDENTIAL_HANDLE)
    })

    it('throws: connection not initialized', async () => {
      const connection = new (Connection as any)()
      const issuerCredential = await issuerCredentialCreate()
      const error = await shouldThrow(() => issuerCredential.sendOffer(connection))
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })
  })

  describe('sendCredential:', () => {
    it('success', async () => {
      const connection = await connectionCreateConnect()
      const issuerCredential = await issuerCredentialCreate()
      await issuerCredential.sendOffer(connection)
      VCXMock.setVcxMock(VCXMockMessage.IssuerCredentialReq)
      VCXMock.setVcxMock(VCXMockMessage.UpdateIssuerCredential)
      await issuerCredential.updateState()
      assert.equal(await issuerCredential.getState(), StateType.RequestReceived)
      await issuerCredential.sendCredential(connection)
      assert.equal(await issuerCredential.getState(), StateType.Accepted)
    })

    it('throws: not initialized', async () => {
      const connection = await connectionCreateConnect()
      const issuerCredential = new IssuerCredential(null as any, {} as any)
      const error = await shouldThrow(() => issuerCredential.sendCredential(connection))
      assert.equal(error.vcxCode, VCXCode.INVALID_ISSUER_CREDENTIAL_HANDLE)
    })

    it('throws: no offer', async () => {
      const connection = await connectionCreateConnect()
      const issuerCredential = await issuerCredentialCreate()
      const error = await shouldThrow(() => issuerCredential.sendCredential(connection))
      assert.equal(error.vcxCode, VCXCode.NOT_READY)
    })

    it('throws: no request', async () => {
      const connection = await connectionCreateConnect()
      const issuerCredential = await issuerCredentialCreate()
      await issuerCredential.sendOffer(connection)
      const error = await shouldThrow(() => issuerCredential.sendCredential(connection))
      assert.equal(error.vcxCode, VCXCode.NOT_READY)
    })
  })

  describe('getPaymentTxn:', () => {
    it('success', async () => {
      const connection = await connectionCreateConnect()
      const issuerCredential = await issuerCredentialCreate()
      await issuerCredential.sendOffer(connection)
      VCXMock.setVcxMock(VCXMockMessage.IssuerCredentialReq)
      VCXMock.setVcxMock(VCXMockMessage.UpdateIssuerCredential)
      await issuerCredential.updateState()
      assert.equal(await issuerCredential.getState(), StateType.RequestReceived)
      await issuerCredential.sendCredential(connection)
      assert.equal(await issuerCredential.getState(), StateType.Accepted)
      const paymentTxn = await issuerCredential.getPaymentTxn()
      validatePaymentTxn(paymentTxn)
    })
  })

  describe('GC:', function () {
    this.timeout(TIMEOUT_GC)

    const issuerCredentialCreateAndDelete = async () => {
      let issuerCredential: IssuerCredential | null = await issuerCredentialCreate()
      const handle = issuerCredential.handle
      issuerCredential = null
      return handle
    }
    it('calls release', async () => {
      const handle = await issuerCredentialCreateAndDelete()
      await gcTest({
        handle,
        serialize: rustAPI().vcx_issuer_credential_serialize,
        stopCode: VCXCode.INVALID_ISSUER_CREDENTIAL_HANDLE
      })
    })
  })
})
