package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.Assert;
import org.junit.Ignore;
import org.junit.Test;

import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class GetValidatorInfoRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {
    @Test
    public void testBuildGetValidatorInfoRequestWorks() throws Exception {
        String expectedResult = String.format("" +
                "\"operation\":{" +
                "\"type\":\"119\"" +
                "}");

        String getValidatorInfoRequest = Ledger.buildGetValidatorInfoRequest(DID).get();
        assertTrue(getValidatorInfoRequest.replace("\\", "").contains(expectedResult));
    }

    @Ignore @Test(timeout = PoolUtils.TEST_TIMEOUT_FOR_REQUEST_ENSURE)
    public void testGetValidatorInfoRequestWorks() throws Exception {
        String did = createStoreAndPublishDidFromTrustee();

        String getValidatorInfoRequest = Ledger.buildGetValidatorInfoRequest(did).get();
        String getValidatorInfoResponse = Ledger.submitRequest(pool, getValidatorInfoRequest).get();

        JSONObject getValidatorInfoObj = new JSONObject(getValidatorInfoResponse);

        Assert.assertFalse(getValidatorInfoObj.getJSONObject("result").isNull("data"));
    }
}
