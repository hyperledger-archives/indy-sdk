from indy import signus
import json

from src.utils.errors import CliError


class SignusCommandHandler:
    @staticmethod
    async def generate_keys_command_handler(state, matched_vars):
        params = {
            'did': matched_vars.get('did'),
            'seed': matched_vars.get('seed'),
            'crypto_type': matched_vars.get('type')
        }
        (did, verkey, pk) = await signus.create_and_store_my_did(state.wallet.handle, json.dumps(params))
        state.wallet.list_did.append(did)
        print("DID: {}".format(did))
        print("Verification Key: {}".format(verkey))
        print("Public key {}".format(pk))

    @staticmethod
    async def replace_keys_start_command_handler(state, matched_vars):
        did = matched_vars.get('did')
        params = {
            'seed': matched_vars.get('seed'),
            'crypto_type': matched_vars.get('type')
        }
        (verkey, pk) = await signus.replace_keys_start(state.wallet.handle, did, json.dumps(params))
        print("Temporary verification Key: {}".format(verkey))
        print("Temporary public key {}".format(pk))

    @staticmethod
    async def replace_keys_apply_command_handler(state, matched_vars):
        await signus.replace_keys_apply(state.wallet.handle,
                                        matched_vars.get('did'))

    @staticmethod
    async def store_their_did_command_handler(state, matched_vars):
        params = {
            'did': matched_vars.get('did'),
            'seed': matched_vars.get('seed'),
            'crypto_type': matched_vars.get('type')
        }
        await signus.store_their_did(state.wallet.handle, json.dumps(params))

    @staticmethod
    async def use_did_command_handler(state, matched_vars):
        did = matched_vars.get('did')
        if did in state.wallet.list_did:
            state.did = did
            print("Did {} set as active".format(state.did))
        else:
            raise CliError("Did {} not found".format(did))

    @staticmethod
    async def list_did_command_handler(state, matched_vars):
        if state.wallet.list_did:
            print("Available DIDs:")
            print("    " + "\n    ".join(state.wallet.list_did))
        else:
            print("You haven't any DID")
