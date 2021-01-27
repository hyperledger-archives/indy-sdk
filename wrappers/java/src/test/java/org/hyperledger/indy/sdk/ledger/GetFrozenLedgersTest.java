package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidResults;
import org.json.JSONObject;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.TimeUnit;

public class GetFrozenLedgersTest extends LedgerIntegrationTest {
    @Rule
    public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

    @Test
    public void TestGetFrozenLedgersRequest() throws Exception {
        DidResults.CreateAndStoreMyDidResult trusteeDidResult = Did.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();

        DidResults.CreateAndStoreMyDidResult myDidResult = Did.createAndStoreMyDid(wallet, "{}").get();
        String did = myDidResult.getDid();

        String request = Ledger.GetFrozenLedgersRequest(did).get();
        JSONObject expectedResult = new JSONObject()
            .put("operation", new JSONObject()
                .put("type", "124")
            );

        System.out.println(request);
        assert (new JSONObject(request).toMap().entrySet()
                .containsAll(expectedResult.toMap().entrySet()));
    }

    @Test
    public void TestFreezeLedgersRequest() throws Exception {
        DidResults.CreateAndStoreMyDidResult trusteeDidResult = Did.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();

        DidResults.CreateAndStoreMyDidResult myDidResult = Did.createAndStoreMyDid(wallet, "{}").get();
        String did = myDidResult.getDid();

        String ledgersIds = "[0,1,28,345]";
        String request = Ledger.GetFreezeLedgersRequest(did, ledgersIds).get();

        List<Integer> expe = new ArrayList<Integer>();
        expe.add(0);
        expe.add(1);
        expe.add(28);
        expe.add(345);

        JSONObject expectedResult = new JSONObject()
            .put("operation", new JSONObject()
                .put("type", "123")
                .put("ledgers_ids", expe)
            );

        assert (new JSONObject(request).toMap().entrySet()
                .containsAll(expectedResult.toMap().entrySet()));
    }
}
