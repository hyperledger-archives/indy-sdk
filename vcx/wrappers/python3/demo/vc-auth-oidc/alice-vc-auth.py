import asyncio
import json
from time import sleep

from demo_utils import  load_payment_library
from vcx.api.disclosed_proof import DisclosedProof
from vcx.api.vcx_init import vcx_init_with_config
import re
import base64

import requests


async def main():
    load_payment_library()

    config = input('Alice Config: ').strip()
    print("Initialize libvcx with new configuration")
    await vcx_init_with_config(config)

    proof_request = {}
    try:
        url = input('Enter URL: ').strip()
        response = requests.get(url, allow_redirects=False)
        location = response.headers['location']

        groups = re.match(".*?m=(.*)", location)
        coded_proof_request = groups.group(1)

        proof_request = json.loads(base64.b64decode(coded_proof_request).decode('utf-8'))
    except Exception as err:
        print("Error occurred during getting Presentation Request: " + str(err))

    print("Presentation Request: " + json.dumps(proof_request))

    print("Create a proof object from proof request")
    proof = await DisclosedProof.create('proof', proof_request)

    print("Query for credentials in the wallet that satisfy the proof request")
    credentials = await proof.get_creds()

    # Use the first available credentials to satisfy the proof request
    for attr in credentials['attrs']:
        credentials['attrs'][attr] = {
            'credential': credentials['attrs'][attr][0]
        }

    print("Generate the proof")
    await proof.generate_proof(credentials, {})

    print("Send the proof to faber")
    await proof.send_proof()

    print("proof is sent!!")


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
