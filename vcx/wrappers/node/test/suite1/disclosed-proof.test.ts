import '../module-resolver-helper'

import { assert } from 'chai'
import {
  connectionCreateConnect,
  dataDisclosedProofCreateWithMsgId,
  dataDisclosedProofCreateWithRequest,
  disclosedProofCreateWithMsgId,
  disclosedProofCreateWithRequest
} from 'helpers/entities'
import { gcTest } from 'helpers/gc'
import { TIMEOUT_GC } from 'helpers/test-constants'
import { initVcxTestMode, shouldThrow } from 'helpers/utils'
import { mapValues } from 'lodash'
import { DisclosedProof, rustAPI, StateType, VCXCode } from 'src'

describe('DisclosedProof', () => {
  before(() => initVcxTestMode())

  describe('create:', () => {
    it('success', async () => {
      await disclosedProofCreateWithRequest()
    })

    it('throws: missing sourceId', async () => {
      const { connection, request } = await dataDisclosedProofCreateWithRequest()
      const error = await shouldThrow(() => DisclosedProof.create({ connection, request } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing request', async () => {
      const { connection, sourceId } = await dataDisclosedProofCreateWithRequest()
      const error = await shouldThrow(() => DisclosedProof.create({ connection, sourceId } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    // Enable when we start utilizing connection prop
    it.skip('throws: missing connection', async () => {
      const { request, sourceId } = await dataDisclosedProofCreateWithRequest()
      const error = await shouldThrow(() => DisclosedProof.create({ request, sourceId } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: invalid request', async () => {
      const { connection, sourceId } = await dataDisclosedProofCreateWithRequest()
      const error = await shouldThrow(() => DisclosedProof.create({ connection, request: 'invalid', sourceId }))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
    })
  })

  describe('createWithMsgId:', () => {
    it('success', async () => {
      await disclosedProofCreateWithMsgId()
    })

    it('throws: missing sourceId', async () => {
      const { connection, msgId } = await dataDisclosedProofCreateWithMsgId()
      const error = await shouldThrow(() => DisclosedProof.createWithMsgId({ connection, msgId } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing request', async () => {
      const { connection, sourceId } = await dataDisclosedProofCreateWithMsgId()
      const error = await shouldThrow(() => DisclosedProof.createWithMsgId({ connection, sourceId } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing connection', async () => {
      const { connection, ...data } = await dataDisclosedProofCreateWithMsgId()
      const error = await shouldThrow(() => DisclosedProof.createWithMsgId({ data } as any))
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
    })

    it('throws: missing connection handle', async () => {
      const { connection, ...data } = await dataDisclosedProofCreateWithMsgId()
      const error = await shouldThrow(() => DisclosedProof.createWithMsgId({ connection: {} as any, ...data }))
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })
  })

  describe('serialize:', () => {
    it('success', async () => {
      const disclosedProof = await disclosedProofCreateWithRequest()
      const data = await disclosedProof.serialize()
      assert.ok(data)
      assert.equal(data.source_id, disclosedProof.sourceId)
    })

    it('throws: not initialized', async () => {
      const disclosedProof = new (DisclosedProof as any)()
      const error = await shouldThrow(() => disclosedProof.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_DISCLOSED_PROOF_HANDLE)
    })

    it('throws: disclosedProof released', async () => {
      const disclosedProof = await disclosedProofCreateWithRequest()
      const data = await disclosedProof.serialize()
      assert.ok(data)
      assert.equal(data.source_id, disclosedProof.sourceId)
      assert.equal(await disclosedProof.release(), VCXCode.SUCCESS)
      const error = await shouldThrow(() => disclosedProof.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_DISCLOSED_PROOF_HANDLE)
    })
  })

  describe('deserialize:', () => {
    it('success', async () => {
      const disclosedProof1 = await disclosedProofCreateWithRequest()
      const data1 = await disclosedProof1.serialize()
      const disclosedProof2 = await DisclosedProof.deserialize(data1)
      assert.equal(disclosedProof2.sourceId, disclosedProof1.sourceId)
      const data2 = await disclosedProof2.serialize()
      assert.deepEqual(data1, data2)
    })

    it('throws: incorrect data', async () => {
      const error = await shouldThrow(async () => DisclosedProof.deserialize({ source_id: 'Invalid' } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
    })
  })

  describe('release:', () => {
    it('success', async () => {
      const disclosedProof = await disclosedProofCreateWithRequest()
      assert.equal(await disclosedProof.release(), VCXCode.SUCCESS)
      const errorSerialize = await shouldThrow(() => disclosedProof.serialize())
      assert.equal(errorSerialize.vcxCode, VCXCode.INVALID_DISCLOSED_PROOF_HANDLE)
    })

    // TODO: Enable once https://evernym.atlassian.net/browse/EN-668 is resolved
    it.skip('throws: not initialized', async () => {
      const disclosedProof = new (DisclosedProof as any)()
      const error = await shouldThrow(() => disclosedProof.release())
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
    })
  })

  describe('updateState:', () => {
    it(`returns ${StateType.None}: not initialized`, async () => {
      const disclosedProof = new (DisclosedProof as any)()
      await disclosedProof.updateState()
      assert.equal(await disclosedProof.getState(), StateType.None)
    })

    it(`returns ${StateType.RequestReceived}: created`, async () => {
      const disclosedProof = await disclosedProofCreateWithRequest()
      await disclosedProof.updateState()
      assert.equal(await disclosedProof.getState(), StateType.RequestReceived)
    })
  })

  describe('sendProof:', () => {
    it('success', async () => {
      const data = await dataDisclosedProofCreateWithRequest()
      const disclosedProof = await disclosedProofCreateWithRequest(data)
      await disclosedProof.sendProof(data.connection)
      assert.equal(await disclosedProof.getState(), StateType.Accepted)
    })
  })

  describe('getRequests:', async () => {
    it('success', async () => {
      const connection = await connectionCreateConnect()
      const requests = await DisclosedProof.getRequests(connection)
      assert.ok(requests)
      assert.ok(requests.length)
      const request = requests[0]
      const disclosedProof = await disclosedProofCreateWithRequest({
        connection,
        request: JSON.stringify(request),
        sourceId: 'disclosedProofTestSourceId'
      })
      await disclosedProof.updateState()
      assert.equal(await disclosedProof.getState(), StateType.RequestReceived)
    })
  })

  describe('getCredentials:', async () => {
    it('success', async () => {
      const disclosedProof = await disclosedProofCreateWithRequest()
      const creds = await disclosedProof.getCredentials()
      assert.ok(creds)
      assert.property(creds, 'attrs')
      assert.property(creds, 'predicates')
    })
  })

  // TODO: Play around with generate proof
  describe('generateProof:', async () => {
    it('success', async () => {
      const data = await dataDisclosedProofCreateWithRequest()
      const disclosedProof = await disclosedProofCreateWithRequest(data)
      const { attrs } = await disclosedProof.getCredentials()
      const valSelfAttested = 'testSelfAttestedVal'
      await disclosedProof.generateProof({
        selectedCreds: {},
        selfAttestedAttrs: mapValues(attrs, () => valSelfAttested)
      })
      await disclosedProof.sendProof(data.connection)
    })
  })

  describe('GC:', function () {
    this.timeout(TIMEOUT_GC)

    const disclosedProofCreateAndDelete = async () => {
      let disclosedProof: DisclosedProof | null = await disclosedProofCreateWithRequest()
      const handle = disclosedProof.handle
      disclosedProof = null
      return handle
    }
    it('calls release', async () => {
      const handle = await disclosedProofCreateAndDelete()
      await gcTest({
        handle,
        serialize: rustAPI().vcx_disclosed_proof_serialize,
        stopCode: VCXCode.INVALID_DISCLOSED_PROOF_HANDLE
      })
    })
  })
})
