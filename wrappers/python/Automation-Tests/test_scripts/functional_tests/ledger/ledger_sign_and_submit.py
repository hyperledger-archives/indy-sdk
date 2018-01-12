"""
Created on Dec 11, 2017

@author: khoi.ngo

Implementing test case SignAndSubmitRequest with valid value.
"""
import json

from indy import signus, ledger

from utilities import common, constant
from utilities.result import Status
from utilities.test_scenario_base import TestScenarioBase
from utilities.utils import perform


class SignAndSubmitRequest(TestScenarioBase):
    async def execute_test_steps(self):
        # 1. Prepare pool and wallet. Get pool_hanlde, wallet_hanlde
        self.steps.add_step("Prepare pool and wallet")
        self.pool_handle, self.wallet_handle = \
            await perform(self.steps, common.prepare_pool_and_wallet,
                          self.pool_name, self.wallet_name,
                          self.pool_genesis_txn_file)

        # 2. Create and store did
        seed_trustee_2 = "000000000000000000000000Trustee2"
        self.steps.add_step("Create DID")
        (submitter_did, _) = \
            await perform(self.steps, signus.create_and_store_my_did,
                          self.wallet_handle,
                          json.dumps({"seed": constant.seed_default_trustee}))
        (target_did, _) = \
            await perform(self.steps, signus.create_and_store_my_did,
                          self.wallet_handle,
                          json.dumps({"seed": seed_trustee_2}))

        # 3. build nym request
        # 4. sign and submit request
        self.steps.add_step("Prepare and send NYM transaction")
        await perform(self.steps, common.build_and_send_nym_request,
                      self.pool_handle,
                      self.wallet_handle, submitter_did, target_did, None,
                      None, None)

        # 5. build nym request
        # 6. submit request
        self.steps.add_step("Prepare and send GET_NYM request")
        get_nym_req = await perform(self.steps, ledger.build_get_nym_request,
                                    submitter_did, target_did)
        nym_response = await perform(self.steps, ledger.submit_request,
                                     self.pool_handle, get_nym_req)

        # 7. Verify GET_NYM request
        self.steps.add_step("Verify GET_NYM request")
        json_response = json.loads(nym_response)
        did_response = json_response["result"]["dest"]
        if did_response == target_did:
            self.steps.get_last_step().set_status(Status.PASSED)
        else:
            message = ("Failed. Expected did is [%s] but actual did is [%s]"
                       % (target_did, did_response))
            self.steps.get_last_step().set_status(Status.FAILED, message)
#             self.steps.get_last_step().set_message(
#                 "Failed. Expected did is [%s] but actual did is [%s]"
#                 % (target_did, did_response))


if __name__ == '__main__':
    SignAndSubmitRequest().execute_scenario()
