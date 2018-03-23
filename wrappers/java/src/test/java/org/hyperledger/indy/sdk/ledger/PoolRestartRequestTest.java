package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.junit.Test;

import static org.junit.Assert.assertTrue;

public class PoolRestartRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {
    @Test
    public void testBuildPoolRestartRequestWorksForStartAction() throws Exception {
        String expectedResult = String.format("\"identifier\":\"%s\"," +
                "\"operation\":{\"type\":\"116\"," +
                "\"action\":\"start\"," +
                "\"schedule\":{}", DID);

        String request = Ledger.buildPoolRestartRequest(DID, "start","{}").get();

        assertTrue(request.contains(expectedResult));
    }
}
