import asyncio


async def wait_for_state(obj, target_state):
        await obj.update_state()
        state = await obj.get_state()
        while state != target_state:
            print('waiting for current state %s to become [%s]' % (state, target_state))
            asyncio.sleep(5)
            await obj.update_state()
            state = await obj.get_state()
        print('Done waiting for state [%s]' % state)
