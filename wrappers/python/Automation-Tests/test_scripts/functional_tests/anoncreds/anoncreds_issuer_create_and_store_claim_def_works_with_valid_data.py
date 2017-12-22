"""
Created on Dec 15, 2017

@author: nhan.nguyen
"""

import json
from indy import anoncreds, signus
from utilities import utils, common, constant
from test_scripts.functional_tests.anoncreds.anoncreds_test_base \
    import AnoncredsTestBase


class TestIssuerCreateAndStoreClaimDefWithValidData(AnoncredsTestBase):
    async def execute_test_steps(self):
        # 1. Create wallet.
        # 2. Open wallet.
        self.wallet_handle = await \
            common.create_and_open_wallet_for_steps(self.steps,
                                                    self.wallet_name,
                                                    self.pool_name)

        # 3. Create 'issuer_did'.
        self.steps.add_step("Create 'issuer_did'")
        (issuer_did, _) = await utils.perform(self.steps,
                                              signus.create_and_store_my_did,
                                              self.wallet_handle, "{}")

        # 4. Create and store claim definition and store
        # returned result as 'claim_def'.
        self.steps.add_step("Create and store claim definition and "
                            "store returned result as 'claim_def'")
        claim_def = await \
            utils.perform(self.steps,
                          anoncreds.issuer_create_and_store_claim_def,
                          self.wallet_handle, issuer_did,
                          json.dumps(constant.gvt_schema),
                          constant.signature_type, False)

        claim_def_primary = json.loads(claim_def)['data']['primary']

        # 5. Check len(claim_def['data']['primary']['r']).
        self.steps.add_step("Check len(claim_def['data']['primary']['r'])")
        error_message = "Length of claim_def['data']['primary']['r'] " \
                        "is not equal with 4"
        utils.check(self.steps, error_message,
                    condition=lambda: len(claim_def_primary['r']) == 4)

        # 6. Check claim_def['data']['primary']['n'].
        self.__check_a_field_is_not_empty(claim_def_primary, 'n')

        # 7. Check claim_def['data']['primary']['s'].
        self.__check_a_field_is_not_empty(claim_def_primary, 's')

        # 8. Check claim_def['data']['primary']['rms'].
        self.__check_a_field_is_not_empty(claim_def_primary, 'rms')

        # 9. Check claim_def['data']['primary']['z'].
        self.__check_a_field_is_not_empty(claim_def_primary, "z")

        # 10. Check claim_def['data']['primary']['rctxt'].
        self.__check_a_field_is_not_empty(claim_def_primary, "rctxt")

    def __check_a_field_is_not_empty(self, claim_json_def_primary, key: str):
        self.steps.add_step(
            "Check claim_def['data']['primary']['{}']".format(key))

        error_message = "Claim_def['data']['primary']['{}'] " \
                        "is empty".format(key)

        utils.check(self.steps, error_message,
                    condition=lambda: len(claim_json_def_primary[key]) > 0)


if __name__ == '__main__':
    TestIssuerCreateAndStoreClaimDefWithValidData().execute_scenario()
