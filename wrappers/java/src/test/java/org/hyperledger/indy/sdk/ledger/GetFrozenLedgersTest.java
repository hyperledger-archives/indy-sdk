package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidResults;
import org.json.JSONObject;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

import java.util.Arrays;
import java.util.List;
import java.util.concurrent.TimeUnit;

public class GetFrozenLedgersTest extends LedgerIntegrationTest {
    @Rule
    public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

    @Test
    public void TestBuildGetFrozenLedgersRequest() throws Exception {
        DidResults.CreateAndStoreMyDidResult myDidResult = Did.createAndStoreMyDid(wallet, "{}").get();
        String did = myDidResult.getDid();

        String request = Ledger.buildGetFrozenLedgersRequest(did).get();
        JSONObject expectedResult = new JSONObject()
            .put("operation", new JSONObject()
                .put("type", "10")
            );

        System.out.println(request);
        assert (new JSONObject(request).toMap().entrySet()
                .containsAll(expectedResult.toMap().entrySet()));
    }

    @Test
    public void TestLedgersFreezeRequest() throws Exception {
        DidResults.CreateAndStoreMyDidResult myDidResult = Did.createAndStoreMyDid(wallet, "{}").get();
        String did = myDidResult.getDid();

        List<Integer> ledgersIds = Arrays.asList(0, 1, 28 ,345);
        String request = Ledger.buildLedgersFreezeRequest(did, ledgersIds).get();

        JSONObject expectedResult = new JSONObject()
            .put("operation", new JSONObject()
                .put("type", "9")
                .put("ledgers_ids", ledgersIds)
            );

        assert (new JSONObject(request).toMap().entrySet()
                .containsAll(expectedResult.toMap().entrySet()));
    }

    @Test
    public void TestLedgersFreezeRequestWithEmptyData() throws Exception {
        DidResults.CreateAndStoreMyDidResult myDidResult = Did.createAndStoreMyDid(wallet, "{}").get();
        String did = myDidResult.getDid();

        List<Integer> ledgersIds = Arrays.asList();
        String request = Ledger.buildLedgersFreezeRequest(did, ledgersIds).get();

        JSONObject expectedResult = new JSONObject()
                .put("operation", new JSONObject()
                        .put("type", "9")
                        .put("ledgers_ids", ledgersIds)
                );

        assert (new JSONObject(request).toMap().entrySet()
                .containsAll(expectedResult.toMap().entrySet()));
    }
}
