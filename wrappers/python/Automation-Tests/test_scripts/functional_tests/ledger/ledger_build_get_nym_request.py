"""
Created on Dec 12, 2017

@author: khoi.ngo

Implementing test case BuildGetNymRequest with valid value.
"""
import json

from indy import signus, ledger

from libraries import common, constant
from libraries.constant import JsonTemplate
from libraries.test_scenario_base import TestScenarioBase
from libraries.utils import perform, verify_json


class BuildGetNymRequest(TestScenarioBase):
    async def execute_test_steps(self):
        # 1. Prepare pool and wallet. Get pool_hanlde, wallet_hanlde
        self.steps.add_step("Prepare pool and wallet")
        self.pool_handle, self.wallet_handle = \
            await perform(self.steps,
                          common.prepare_pool_and_wallet,
                          self.pool_name,
                          self.wallet_name,
                          self.pool_genesis_txn_file)

        # 2. Create and store did
        seed_trustee_2 = "000000000000000000000000Trustee2"
        self.steps.add_step("Create DIDs")
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

        # 3. build and send nym request
        self.steps.add_step("Prepare and send NYM transaction")
        await perform(self.steps, common.build_and_send_nym_request,
                      self.pool_handle,
                      self.wallet_handle, submitter_did, target_did,
                      target_verkey, None, None)

        # 4. build get nym request
        self.steps.add_step("build GET_NYM request")
        get_nym_req = json.loads(
            await perform(self.steps, ledger.build_get_nym_request,
                          submitter_did, target_did))

        # 5. Verifying get_nym_request json correctly.
        self.steps.add_step("Verifying get_nym_request json")
        expected_response = json.loads(
            JsonTemplate.get_nym_response.format(submitter_did,
                                                 "105", target_did))
        verify_json(self.steps, expected_response, get_nym_req)


if __name__ == '__main__':
    BuildGetNymRequest().execute_scenario()
