import json
import re
import base64

import requests

async def handle_challenge():
    try:
        url = input('Enter URL: ').strip()
        response = requests.get(url, allow_redirects=False)
        location = response.headers['location']

        groups = re.match(".*?m=(.*)", location)
        coded_proof_request = groups.group(1)

        proof_request = json.loads(base64.b64decode(coded_proof_request).decode('utf-8'))
        return proof_request
    except Exception as err:
        print("Error occurred during getting Presentation Request: " + str(err))
