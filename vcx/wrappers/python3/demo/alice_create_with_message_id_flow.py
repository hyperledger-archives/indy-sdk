import asyncio

from alice import connect, accept_offer, create_proof, init
from demo_utils import download_message

from vcx.api.credential import Credential
from vcx.api.disclosed_proof import DisclosedProof


# logging.basicConfig(level=0)

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
            print(uid)
            credential = await Credential.create_with_msgid('credential', connection_to_faber, uid)
            print("Offer")
            print(credential.cred_offer)
            await accept_offer(connection_to_faber, credential)
        elif answer == '2':
            print("Check agency for a proof request")
            pw_did = await connection_to_faber.get_my_pw_did()
            uid, request, _ = await download_message(pw_did, 'presentation-request')
            print("#23 Create a Disclosed proof object from proof request")
            proof = await DisclosedProof.create_with_msgid('proof', connection_to_faber, uid)
            await create_proof(connection_to_faber, proof)
        else:
            pass
            # break
    print(answer)
    print("Finished")


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
