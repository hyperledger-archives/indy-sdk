import asyncio
import json

from alice import init, connect, accept_offer, create_proof
from demo_utils import download_message, update_message_as_read

from vcx import logging
from vcx.api.credential import Credential
from vcx.api.disclosed_proof import DisclosedProof



async def main():
    await init()
    connection_to_faber = None
    while True:
        answer = input(
            "Would you like to do? \n "
            "0 - establish connection \n "
            "1 - check for credential offer \n "
            "2 - check for proof request \n "
            "else finish \n") \
            .lower().strip()
        if answer == '0':
            connection_to_faber = await connect()
        elif answer == '1':
            print("Check agency for a credential offer")
            pw_did = await connection_to_faber.get_my_pw_did()
            uid, offer, _ = await download_message(pw_did, 'credential-offer')
            credential = await Credential.create('credential', json.loads(offer))
            await accept_offer(connection_to_faber, credential)
            await update_message_as_read(pw_did, uid)
        elif answer == '2':
            print("Check agency for a proof request")
            pw_did = await connection_to_faber.get_my_pw_did()
            uid, request, _ = await download_message(pw_did, 'presentation-request')
            print("#23 Create a Disclosed proof object from proof request")
            proof = await DisclosedProof.create('proof', json.loads(request))
            await create_proof(connection_to_faber, proof)
            await update_message_as_read(pw_did, uid)
        else:
            pass
            # break
    print("Finished")


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
