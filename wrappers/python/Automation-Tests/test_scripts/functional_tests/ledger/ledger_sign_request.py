"""
Created on Dec 11, 2017

@author: khoi.ngo

Implementing test case SignRequest with valid value.
"""
import json

from indy import signus, ledger

from utilities import common, constant
from utilities.result import Status
from utilities.test_scenario_base import TestScenarioBase
from utilities.utils import perform


class SignRequest(TestScenarioBase):
    async def execute_postcondition_steps(self):
        pass

    async def execute_test_steps(self):
        # Prepare data to test.
        message = json.dumps(
            {
                "reqId": 1496822211362017764,
                "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                "operation":
                    {
                        "type": "1",
                        "dest": "VsKV7grR1BUE29mG2Fm2kX",
                        "verkey": "GjZWsBLgZCR18aL468JAT7"
                                  "w9CZRiBnpxUPPgyQxh4voa"
                    }
            })
        expected_signature = "65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59" \
                             "p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW"

        # 1. Prepare pool and wallet. Get pool_hanlde, wallet_hanlde
        self.steps.add_step("Prepare pool and wallet")
        self.pool_handle, self.wallet_handle = \
            await perform(self.steps, common.prepare_pool_and_wallet,
                          self.pool_name, self.wallet_name,
                          self.pool_genesis_txn_file)

        # 2. Create and store did
        self.steps.add_step("Create DID")
        (did, _) = \
            await perform(self.steps, signus.create_and_store_my_did,
                          self.wallet_handle,
                          json.dumps({"seed": constant.seed_default_trustee}))

        # 3. sign request
        self.steps.add_step("sign the request")
        json_response = \
            await perform(self.steps, ledger.sign_request, self.wallet_handle,
                          did, message)

        # 4. verify the signature is correct.
        self.steps.add_step("verify the signature is correct.")
        signed_msg = json.loads(json_response)
        actual_signature = signed_msg['signature']
        if actual_signature == expected_signature:
            self.steps.get_last_step().set_status(Status.PASSED)
        else:
            message = ("Failed. Expected signature is [%s] "
                       "but actual signature is [%s]"
                       % (expected_signature, actual_signature))
            self.steps.get_last_step().set_status(Status.FAILED, message)
#             self.steps.get_last_step().set_message(
#                 "Failed. Expected signature is [%s] "
#                 "but actual signature is [%s]"
#                 % (expected_signature, actual_signature))


if __name__ == '__main__':
    SignRequest().execute_scenario()
