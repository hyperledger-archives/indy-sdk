"""
Created on Dec 11, 2017

@author: khoi.ngo

Implementing test case SubmitRequest with valid value.
"""
import json

from indy import signus, ledger

from libraries import common, constant
from libraries.constant import JsonTemplate
from libraries.result import Status
from libraries.test_scenario_base import TestScenarioBase
from libraries.utils import perform


class SubmitRequest(TestScenarioBase):
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
        (target_did, target_verkey) = \
            await perform(self.steps,
                          signus.create_and_store_my_did,
                          self.wallet_handle,
                          json.dumps({
                              "seed": seed_trustee_2}))

        # 3. Prepare the request.
        self.steps.add_step("sign the message")
        message = JsonTemplate.message.format(1496822211362017764,
                                              submitter_did, "1", target_did,
                                              target_verkey)
        response = await perform(self.steps, ledger.sign_request,
                                 self.wallet_handle, submitter_did, message)

        # get signature
        signed_msg = json.loads(response)
        signature = signed_msg['signature']
        type_request = "105"
        request_json = JsonTemplate.submit_request.format(1491566332010860,
                                                          submitter_did,
                                                          type_request,
                                                          target_did,
                                                          signature)
        data = ""
        expected_response = json.loads(
            JsonTemplate.submit_response.format(1491566332010860,
                                                submitter_did, target_did,
                                                data, type_request, "REPLY"))

        # 4. Submit request
        response = json.loads(
            await perform(self.steps, ledger.submit_request, self.pool_handle,
                          request_json))

        # 5. Verify json response is correct.
        self.steps.add_step("verify json response is correct.")
        r1 = response["op"] == expected_response["op"]
        r2 = response["result"]["identifier"] == expected_response["result"][
            "identifier"]
        r3 = response["result"]["dest"] == expected_response["result"]["dest"]
        r4 = response["result"]["type"] == expected_response["result"]["type"]
        if (r1 and r2 and r3 and r4) is True:
            self.steps.get_last_step().set_status(Status.PASSED)
        else:
            self.steps.get_last_step().set_message(
                constant.JSON_INCORRECT.format(""))


if __name__ == '__main__':
    SubmitRequest().execute_scenario()
