import '../module-resolver-helper'

import { assert } from 'chai'
import { connectionCreate, connectionCreateConnect, dataConnectionCreate } from 'helpers/entities'
import { gcTest } from 'helpers/gc'
import { TIMEOUT_GC } from 'helpers/test-constants'
import { initVcxTestMode, shouldThrow } from 'helpers/utils'
import { Connection, rustAPI, StateType, VCXCode, VCXMock, VCXMockMessage } from 'src'

describe('Connection:', () => {
  before(() => initVcxTestMode())

  describe('create:', () => {
    it('success', async () => {
      await connectionCreate()
    })

    // TODO: Enable me once https://evernym.atlassian.net/browse/EN-662 is resolved
    it.skip('success: parallel', async () => {
      const numConnections = 50
      const data = dataConnectionCreate()
      await Promise.all(new Array(numConnections).fill(0).map(() => connectionCreate(data)))
    })
  })

  describe('connect:', () => {
    it('success: without phone', async () => {
      const connection = await connectionCreate()
      const inviteDetails = await connection.connect()
      assert.notEqual(inviteDetails, '')
    })

    it('success: with phone', async () => {
      const connection = await connectionCreate()
      const inviteDetails = await connection.connect({ phone: '7202200000' })
      assert.notEqual(inviteDetails, '')
    })

    it('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const err = await shouldThrow(async () => connection.connect())
      assert.equal(err.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })
  })

  describe('serialize:', () => {
    it('success', async () => {
      const connection = await connectionCreate()
      const data = await connection.serialize()
      assert.ok(data)
      assert.equal(data.source_id, connection.sourceId)
      assert.equal(data.state, StateType.Initialized)
    })

    it('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const error = await shouldThrow(() => connection.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })

    it('throws: connection released', async () => {
      const connection = await connectionCreateConnect()
      const data = await connection.serialize()
      assert.ok(data)
      assert.equal(data.source_id, connection.sourceId)
      assert.equal(await connection.release(), VCXCode.SUCCESS)
      const error = await shouldThrow(() => connection.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })

    it('throws: connection deleted', async () => {
      const connection = await connectionCreate()
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
      const error = await shouldThrow(async () => Connection.deserialize({ source_id: 'Invalid' } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
    })
  })

  describe('release:', () => {
    it('success', async () => {
      const connection = await connectionCreateConnect()
      assert.equal(await connection.release(), VCXCode.SUCCESS)
      const errorSerialize = await shouldThrow(() => connection.serialize())
      assert.equal(errorSerialize.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })

    // TODO: Enable once https://evernym.atlassian.net/browse/EN-668 is resolved
    it.skip('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const error = await shouldThrow(() => connection.release())
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
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
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.OfferSent)
    })

    it(`returns ${StateType.Accepted}: mocked accepted`, async () => {
      const connection = await connectionCreateConnect()
      VCXMock.setVcxMock(VCXMockMessage.GetMessages)
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.Accepted)
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

  describe('GC:', function () {
    this.timeout(TIMEOUT_GC)

    const connectionCreateCheckAndDelete = async () => {
      let connection: Connection | null = await connectionCreateConnect()
      const handle = connection.handle
      connection = null
      return handle
    }
    it('calls release', async () => {
      const handle = await connectionCreateCheckAndDelete()
      await gcTest({
        handle,
        serialize: rustAPI().vcx_connection_serialize,
        stopCode: VCXCode.INVALID_CONNECTION_HANDLE
      })
    })
  })
})
