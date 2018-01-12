"""
Created on Dec 12, 2017

@author: khoi.ngo

Implementing test case SchemaRequest with valid value.
"""
import json

from indy import signus, ledger

from utilities import common, constant
from utilities.test_scenario_base import TestScenarioBase
from utilities.utils import perform, verify_json


class SchemaRequest(TestScenarioBase):
    async def execute_test_steps(self):
        # 1. Prepare pool and wallet. Get pool_hanlde, wallet_hanlde
        self.steps.add_step("Prepare pool and wallet")
        self.pool_handle, self.wallet_handle = \
            await perform(self.steps, common.prepare_pool_and_wallet,
                          self.pool_name, self.wallet_name,
                          self.pool_genesis_txn_file)

        # 2. Create and store did
        self.steps.add_step("Create DID")
        (submitter_did, _) = \
            await perform(self.steps, signus.create_and_store_my_did,
                          self.wallet_handle,
                          json.dumps({"seed": constant.seed_default_trustee}))

        # 3. build schema request
        self.steps.add_step("Build schema request")
        data = '{"name":"name", "version":"1.0", "attr_names":["name","male"]}'
        response = json.loads(
            await perform(self.steps, ledger.build_schema_request,
                          submitter_did, data))

        # 4. Verifying build schema successfully by checking data response
        self.steps.add_step(
            "Verifying build schema successfully by checking data response")
        expected_response = {
            "operation": {"type": "101", "data": json.loads(data)}}
        verify_json(self.steps, expected_response, response)


if __name__ == '__main__':
    SchemaRequest().execute_scenario()
