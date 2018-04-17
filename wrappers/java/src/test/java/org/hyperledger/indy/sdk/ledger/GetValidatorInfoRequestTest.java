package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.Test;

import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class GetValidatorInfoRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {
    @Test
    public void testBuildGetValidatorInfoRequestWorks() throws Exception {
        String expectedResult = String.format("" +
                "\"identifier\":\"%s\"," +
                "\"operation\":{" +
                "\"type\":\"3\"," +
                "}", DID);

        String getValidatorInfoRequest = Ledger.buildGetValidatorInfoRequest(DID).get();
        assertTrue(getValidatorInfoRequest.replace("\\", "").contains(expectedResult));
    }

    @Test(timeout = PoolUtils.TEST_TIMEOUT_FOR_REQUEST_ENSURE)
    public void testGetValidatorInfoRequestWorks() throws Exception {
        String did = createStoreAndPublishDidFromTrustee();

        String getValidatorInfoRequest = Ledger.buildGetValidatorInfoRequest(did).get();
        String getValidatorInfoResponse = PoolUtils.ensurePreviousRequestApplied(pool, getValidatorInfoRequest, response -> {
            JSONObject getValidatorInfoResponseObj = new JSONObject(response);
            /* TODO  Provide JSON Object similar response*/
            //return new JSONObject(SCHEMA_DATA).similar(schemaTransactionObj.getJSONObject("data"));
            return true;
        });
        assertNotNull(getValidatorInfoResponse);
    }
}
