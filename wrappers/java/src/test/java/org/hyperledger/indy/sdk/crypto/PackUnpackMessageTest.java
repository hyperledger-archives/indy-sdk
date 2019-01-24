package org.hyperledger.indy.sdk.crypto;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidResults;
import org.hyperledger.indy.sdk.wallet.WalletItemNotFoundException;
import org.junit.Test;
import org.json.JSONArray;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.*;

public class PackUnpackMessageTest extends IndyIntegrationTestWithSingleWallet {

    @Test
    public void testPackMessage() throws Exception {
        String message = "hello world";

        JSONArray receieversArray = new JSONArray();
        receieversArray.put(IndyIntegrationTest.VERKEY_MY1);
        receieversArray.put(IndyIntegrationTest.VERKEY_MY2);
        receieversArray.put(IndyIntegrationTest.VERKEY_TRUSTEE);

        String myVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();

        byte[] packedMessage = Crypto.packMessage(wallet, receieversArray.toString(), null, message.getBytes()).get();

        assertNotNull(packedMessage);
    }

    // This test proves the API is hooked up correctly and error result is returned
    @Test(expected = java.util.concurrent.ExecutionException.class)
    public void testUnpackMessageWithInvalidStructure() throws Exception {

        String packedMessage = "jibberish";
        byte[] unpackedMessage = Crypto.unpackMessage(wallet, packedMessage.getBytes()).get();

    }

}
