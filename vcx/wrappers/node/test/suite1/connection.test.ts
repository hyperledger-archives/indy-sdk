import '../module-resolver-helper'

import { assert } from 'chai'
import { connectionCreate, connectionCreateConnect, dataConnectionCreate } from 'helpers/entities'
import { INVITE_ACCEPTED_MESSAGE } from 'helpers/test-constants'
import { initVcxTestMode, shouldThrow, sleep } from 'helpers/utils'
import { Connection, StateType, VCXCode, VCXMock, VCXMockMessage } from 'src'

describe('Connection:', () => {
  before(() => initVcxTestMode())

  describe('create:', () => {
    it('success', async () => {
      await connectionCreate()
    })

    it('success: parallel', async () => {
      const numConnections = 50
      const data = dataConnectionCreate()
      await Promise.all(new Array(numConnections).fill(0).map(() => connectionCreate(data)))
    })
  })

  describe('connect:', () => {
    it('success: without phone', async () => {
      const connection = await connectionCreate()
      const inviteDetails = await connection.connect({ data: '{"connection_type":"QR"}' })
      assert.notEqual(inviteDetails, '')
    })

    it('success: with phone', async () => {
      const connection = await connectionCreate()
      const inviteDetails = await connection.connect({ data: '{"connection_type":"SMS","phone":"7202200000"}' })
      assert.notEqual(inviteDetails, '')
    })

    it('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const err = await shouldThrow(async () => connection.connect({ data: '{"connection_type":"QR"}' }))
      assert.equal(err.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })
  })

  describe('sendMessage:', () => {
    it('success: sends message', async () => {
      const connection = await connectionCreate()
      await connection.connect({ data: '{"connection_type":"QR"}' })
      const error = await shouldThrow(() => connection.sendMessage({ msg: 'msg', type: 'msg', title: 'title' }))
      assert.equal(error.vcxCode, VCXCode.NOT_READY)
    })
  })

  describe('signData:', () => {
    it('success: signs data', async () => {
      const connection = await connectionCreate()
      await connection.connect({ data: '{"connection_type":"QR"}' })
      const signature = await connection.signData(new Buffer('random string'))
      assert(signature)
    })
  })

  describe('verifySignature', () => {
    it('success: verifies the signature', async () => {
      const connection = await connectionCreate()
      await connection.connect({ data: '{"connection_type":"QR"}' })
      const valid = await connection.verifySignature({data: new Buffer('random string'),
        signature: new Buffer('random string')})
      assert(valid)
    })
  })
  describe('serialize:', () => {
    it('success', async () => {
      const connection = await connectionCreate()
      const serialized = await connection.serialize()
      assert.ok(serialized)
      assert.property(serialized, 'version')
      assert.property(serialized, 'data')
      const { data, version } = serialized
      assert.ok(data)
      assert.ok(version)
      assert.equal(data.source_id, connection.sourceId)
      assert.equal(data.state, StateType.Initialized)
    })

    it('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const error = await shouldThrow(() => connection.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })

    it('throws: connection deleted', async () => {
      const connection = await connectionCreate()
      await connection.connect({ data: '{"connection_type":"QR"}' })
      await connection.delete()
      const error = await shouldThrow(() => connection.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })
  })

  describe('deserialize:', () => {
    it('success', async () => {
      const connection1 = await connectionCreate()
      const data1 = await connection1.serialize()
      const connection2 = await Connection.deserialize(data1)
      assert.equal(connection2.sourceId, connection1.sourceId)
      const data2 = await connection2.serialize()
      assert.deepEqual(data1, data2)
    })

    it('throws: incorrect data', async () => {
      const error = await shouldThrow(async () => Connection.deserialize({ data:
        { source_id: 'Invalid' } } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
    })
  })

  describe('updateState:', () => {
    it(`returns ${StateType.None}: not initialized`, async () => {
      const connection = new (Connection as any)()
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.None)
    })

    it(`returns ${StateType.Initialized}: not connected`, async () => {
      const connection = await connectionCreate()
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.Initialized)
    })

    it(`returns ${StateType.OfferSent}: connected`, async () => {
      const connection = await connectionCreateConnect()
      VCXMock.setVcxMock(VCXMockMessage.AcceptInvite)
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.Accepted)
    })

    it(`returns ${StateType.Accepted}: mocked accepted`, async () => {
      const connection = await connectionCreateConnect()
      VCXMock.setVcxMock(VCXMockMessage.GetMessages)
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.Accepted)
    })

    it(`returns ${StateType.Accepted}: mocked accepted`, async () => {
      const connection = await connectionCreateConnect()
      await connection.updateStateWithMessage(INVITE_ACCEPTED_MESSAGE)
      assert.equal(await connection.getState(), StateType.Accepted)
    })

    it.skip(`returns ${StateType.Accepted}: mocked accepted in parallel`, async () => {
      const numConnections = 50
      const interval = 50
      const sleepTime = 100
      const connectionsWithTimers = await Promise.all(new Array(numConnections).fill(0).map(async () => {
        const connection = await connectionCreate()
        const timer = setInterval(() => connection.updateState(), interval)
        return { connection, timer }
      }))
      let cond = false
      while (cond) {
        const states = await Promise.all(connectionsWithTimers.map(({ connection }) => connection.getState()))
        cond = states.every((state) => state === StateType.Accepted)
        VCXMock.setVcxMock(VCXMockMessage.GetMessages)
        await sleep(sleepTime)
      }
      connectionsWithTimers.forEach(({ timer }) => clearInterval(timer))
    })
  })

  describe('inviteDetails:', () => {
    it('success: with abbr', async () => {
      const connection = await connectionCreateConnect()
      const details = await connection.inviteDetails(true)
      assert.include(details, '"dp":')
    })

    it('success: without abbr', async () => {
      const connection = await connectionCreateConnect()
      const details = await connection.inviteDetails()
      assert.include(details, '"senderAgencyDetail":')
    })
  })

})
