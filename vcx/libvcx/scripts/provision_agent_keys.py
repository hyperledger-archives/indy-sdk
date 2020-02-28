#!/usr/bin/env python3
# Provided by The Python Standard Library
import json
import argparse
import asyncio
import time
import urllib.request
import sys
from ctypes import *

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("AGENCY_URL")
    parser.add_argument("WALLET_KEY")
    parser.add_argument("--wallet-name", help="optional name for libindy wallet")
    parser.add_argument("--wallet-type", help="optional type of libindy wallet")
    parser.add_argument("--agent-seed", help="optional seed used to create enterprise->agent DID/VK")
    parser.add_argument("--enterprise-seed", help="optional seed used to create enterprise DID/VK")
    parser.add_argument("--pool-config", help="optional additional config for connection to pool nodes ({timeout: Opt<int>, extended_timeout: Opt<int>, preordered_nodes: Opt<array<string>>})")
    parser.add_argument("-v", "--verbose", action="store_true")
    return parser.parse_args()

def get_agency_info(agency_url):
    agency_info = {}
    agency_resp = ''
    #Get agency's did and verkey:
    try:
        agency_req=urllib.request.urlopen('{}/agency'.format(agency_url))
    except:
        exc_type, exc_value, exc_traceback = sys.exc_info()
        sys.stderr.write("Failed looking up agency did/verkey: '{}': {}\n".format(exc_type.__name__,exc_value))
        print(json.dumps({
            'provisioned': False,
            'provisioned_status': "Failed: Could not retrieve agency info from: {}/agency: '{}': {}".format(agency_url,exc_type.__name__,exc_value)
        },indent=2))
        sys.exit(1)
    agency_resp = agency_req.read()
    try:
        agency_info = json.loads(agency_resp.decode())
    except:
        exc_type, exc_value, exc_traceback = sys.exc_info()
        sys.stderr.write("Failed parsing response from agency endpoint: {}/agency: '{}': {}\n".format(agency_url,exc_type.__name__,exc_value))
        sys.stderr.write("RESPONSE: {}".format(agency_resp))
        print(json.dumps({
            'provisioned': False,
            'provisioned_status': "Failed: Could not parse response from agency endpoint from: {}/agency: '{}': {}".format(agency_url,exc_type.__name__,exc_value)
        },indent=2))
        sys.exit(1)
    return agency_info

def register_agent(args):
    vcx = CDLL("/usr/lib/libvcx.so")

    if args.verbose:
            c_debug = c_char_p('debug'.encode('utf-8'))
            vcx.vcx_set_default_logger(c_debug)

    agency_info = get_agency_info(args.AGENCY_URL)
    json_str = json.dumps({'agency_url':args.AGENCY_URL,
        'agency_did':agency_info['DID'],
        'agency_verkey':agency_info['verKey'],
        'wallet_key':args.WALLET_KEY,
        'wallet_name':args.wallet_name,
        'wallet_type':args.wallet_type,
        'pool_config':args.pool_config,
        'agent_seed':args.agent_seed,
        'enterprise_seed':args.enterprise_seed})

    c_json = c_char_p(json_str.encode('utf-8'))

    rc = vcx.vcx_provision_agent(c_json)

    if rc == 0:
        sys.stderr.write("could not register agent.  Try again with '-v' for more details\n")
        print(json.dumps({
            'provisioned': False,
            'provisioned_status': "Failed: Could not register agent.  Try again with '-v' for more details"
        },indent=2))
    else:
        pointer = c_int(rc)
        string = cast(pointer.value, c_char_p)
        new_config = json.loads(string.value.decode('utf-8'))

        if 'payment_method' not in new_config:
            new_config['payment_method'] = 'null'

        print(json.dumps(new_config, indent=2, sort_keys=True))


async def main():
    args = parse_args()

    register_agent(args)


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    time.sleep(.1)
