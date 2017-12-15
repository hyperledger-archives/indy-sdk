"""
Created on Dec 12, 2017

@author: khoi.ngo

Implementing test case GetSchemaRequest with valid value.
"""
import json

from indy import signus, ledger

from libraries import common, constant
from libraries.constant import JsonTemplate
from libraries.test_scenario_base import TestScenarioBase
from libraries.utils import perform, verify_json, generate_random_string


class GetSchemaRequest(TestScenarioBase):
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
        self.steps.add_step("Create DID")
        (submitter_did, _) = \
            await perform(self.steps,
                          signus.create_and_store_my_did,
                          self.wallet_handle,
                          json.dumps({
                              "seed": constant.seed_default_trustee}))
        (target_did, _) = await perform(self.steps,
                                        signus.create_and_store_my_did,
                                        self.wallet_handle,
                                        json.dumps({"seed": seed_trustee_2}))
        # 3. build schema request
        self.steps.add_step("Build schema request")
        name = generate_random_string(size=4)
        version = "1.1.1"
        data_request = (
            '{"name":"%s", "version":"%s", "attr_names":["name","male"]}' % (
                name, version))
        data_response = ('{"name":"%s", "version":"%s"}' % (name, version))
        schema_req = await perform(self.steps, ledger.build_schema_request,
                                   submitter_did, data_request)

        # 4. send schema request
        self.steps.add_step("send schema request")
        await perform(self.steps, ledger.sign_and_submit_request,
                      self.pool_handle,
                      self.wallet_handle, submitter_did, schema_req)

        # 5. Prepare data to check and build get schema request
        self.steps.add_step("build get schema request")
        expected_response = json.loads(
            JsonTemplate.get_schema_response.format(submitter_did, "107",
                                                    target_did, data_response))

        get_schema_req = json.loads(
            await perform(self.steps, ledger.build_get_schema_request,
                          submitter_did,
                          target_did, data_response))

        # 6. Verify json get schema request is correct.
        self.steps.add_step("Verify json get schema request is correct.")
        verify_json(self.steps, expected_response, get_schema_req)


if __name__ == '__main__':
    GetSchemaRequest().execute_scenario()
