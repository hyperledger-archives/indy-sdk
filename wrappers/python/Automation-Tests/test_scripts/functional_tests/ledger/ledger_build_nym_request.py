"""
Created on Dec 12, 2017

@author: khoi.ngo

Implementing test case BuildNymRequest with valid value.
"""
import json

from indy import signus, ledger

from utilities import common, constant
from utilities.result import Status
from utilities.test_scenario_base import TestScenarioBase
from utilities.utils import perform


class BuildNymRequest(TestScenarioBase):
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
            await perform(self.steps,
                          signus.create_and_store_my_did,
                          self.wallet_handle,
                          json.dumps({
                              "seed": constant.seed_default_trustee}))
        (target_did, target_verkey) = \
            await perform(self.steps,
                          signus.create_and_store_my_did,
                          self.wallet_handle,
                          json.dumps({
                              "seed": seed_trustee_2}))

        # 3. build nym request
        self.steps.add_step("build NYM request")
        nym_req_txn = json.loads(
            await perform(self.steps, ledger.build_nym_request, submitter_did,
                          target_did,
                          target_verkey, None, None))

        # 4. Verifying json nym request response correctly.
        self.steps.add_step("Verifying nym request")
        expected_response = json.loads(
            constant.message.format(0, submitter_did, "1",
                                    target_did, target_verkey))
        # We do not check reqId because It is dynamic number.
        try:
            assert nym_req_txn["identifier"] == expected_response["identifier"]
            assert nym_req_txn["operation"]["type"] == \
                expected_response["operation"]["type"]
            assert nym_req_txn["operation"]["dest"] == \
                expected_response["operation"]["dest"]
            assert nym_req_txn["operation"]["verkey"] == \
                expected_response["operation"]["verkey"]
            self.steps.get_last_step().set_status(Status.PASSED)
        except AssertionError:
            message = constant.JSON_INCORRECT.format("")
            self.steps.get_last_step().set_status(Status.FAILED, message)
#             self.steps.get_last_step().set_message(
#                 constant.JSON_INCORRECT.format(""))


if __name__ == '__main__':
    BuildNymRequest().execute_scenario()
