import json

from indy import ledger

from src.utils.errors import CliError


class LedgerCommandHandler:
    @staticmethod
    async def send_message_command_handler(state, matched_vars):
        if not state.did:
            raise CliError("No active did")

        request = json.loads(matched_vars.get('request'))
        request['identifier'] = state.did

        response = await ledger.submit_request(state.pool.handle, json.dumps(request))
        response = json.loads(response)
        print(response['result']['data'])

    @staticmethod
    async def send_nym_command_handler(state, matched_vars):
        if not state.did:
            raise CliError("No active did")

        nym_request = await ledger.build_nym_request(state.did,
                                                     matched_vars.get('did'),
                                                     matched_vars.get('verkey'),
                                                     matched_vars.get('alias'),
                                                     matched_vars.get('role'))

        nym_response = await ledger.sign_and_submit_request(state.pool.handle, state.wallet, state.did, nym_request)
        print(nym_response)
