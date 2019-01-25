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
    public void testPackMessageSuccessfully() throws Exception {
        String message = "hello world";

        JSONArray receieversArray = new JSONArray();
        receieversArray.put(IndyIntegrationTest.VERKEY_MY1);
        receieversArray.put(IndyIntegrationTest.VERKEY_MY2);
        receieversArray.put(IndyIntegrationTest.VERKEY_TRUSTEE);

        String myVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();

        byte[] packedMessage = Crypto.packMessage(wallet, receieversArray.toString(), null, message.getBytes()).get();

        assertNotNull(packedMessage);
    }

    @Test
    public void testPackMessageSuccessfullyWithOneReceiver() throws Exception {
        String message = "hello world";

        JSONArray receieversArray = new JSONArray();
        receieversArray.put(IndyIntegrationTest.VERKEY_MY1);

        String myVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();

        byte[] packedMessage = Crypto.packMessage(wallet, receieversArray.toString(), null, message.getBytes()).get();

        assertNotNull(packedMessage);
    }

    @Test(expected = java.util.concurrent.ExecutionException.class)
    public void testPackMessageSuccessfullyWithNoReceivers() throws Exception {
        String message = "hello world";

        JSONArray receieversArray = new JSONArray();

        String myVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();

        byte[] packedMessage = Crypto.packMessage(wallet, receieversArray.toString(), null, message.getBytes()).get();

        // this assert should never trigger since unpackMessage should throw exception
        assertTrue(false);
    }

    @Test(expected = java.util.concurrent.ExecutionException.class)
    public void testPackMessageSuccessfullyInvalidReceivers() throws Exception {
        String message = "hello world";

        JSONArray receieversArray = new JSONArray();
        receieversArray.put("IndyIntegrationTest.VERKEY_MY1");

        String myVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();

        byte[] packedMessage = Crypto.packMessage(wallet, receieversArray.toString(), null, message.getBytes()).get();

        // this assert should never trigger since unpackMessage should throw exception
        assertTrue(false);
    }

    @Test(expected = java.util.concurrent.ExecutionException.class)
    public void testUnpackMessageWithInvalidStructure() throws Exception {

        String packedMessage = "jibberish";
        byte[] unpackedMessage = Crypto.unpackMessage(wallet, packedMessage.getBytes()).get();

        // this assert should never trigger since unpackMessage should throw exception
        assertTrue(false);
    }

    @Test
    public void testUnpackMessageSuccessfully() throws Exception {
        String message = "hello world";

        JSONArray receieversArray = new JSONArray();
        receieversArray.put(IndyIntegrationTest.VERKEY_MY1);
        receieversArray.put(IndyIntegrationTest.VERKEY_MY2);
        receieversArray.put(IndyIntegrationTest.VERKEY_TRUSTEE);

        String myVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();

        byte[] packedMessage = Crypto.packMessage(wallet, receieversArray.toString(), null, message.getBytes()).get();
        byte[] unpackedMessage = Crypto.unpackMessage(wallet, packedMessage).get();

        assertNotNull(unpackedMessage);
    }

}
