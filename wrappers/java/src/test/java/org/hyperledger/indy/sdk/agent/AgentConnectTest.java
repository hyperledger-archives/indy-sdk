package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.agent.Agent.Listener;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.Test;

import static org.hyperledger.indy.sdk.IndyIntegrationTest.TRUSTEE_SEED;


public class AgentConnectTest extends AgentIntegrationTest {

	@Test
	public void testAgentConnectWorksForRemoteData() throws Exception {
		String endpoint = "127.0.0.1:9705";
		String listenerWalletName = "listenerWallet";
		String trusteeWalletName = "trusteeWallet";

		Wallet.createWallet(poolName, listenerWalletName, "default", null, null).get();
		Wallet listenerWallet = Wallet.openWallet(listenerWalletName, null, null).get();

		Wallet.createWallet(poolName, trusteeWalletName, "default", null, null).get();
		Wallet trusteeWallet = Wallet.openWallet(trusteeWalletName, null, null).get();
		Wallet senderWallet = trusteeWallet;

		SignusResults.CreateAndStoreMyDidResult createMyDidResult = Signus.createAndStoreMyDid(listenerWallet, "{}").get();
		String listenerDid = createMyDidResult.getDid();
		String listenerVerkey = createMyDidResult.getVerkey();
		String listenerPk = createMyDidResult.getPk();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(trusteeWallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();
		String senderDid = trusteeDid;

		String nymRequest = Ledger.buildNymRequest(trusteeDid, listenerDid, listenerVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, trusteeWallet, trusteeDid, nymRequest).get();

		String attribRequest = Ledger.buildAttribRequest(listenerDid, listenerDid, null,
				String.format("{\"endpoint\":{\"ha\":\"%s\",\"verkey\":\"%s\"}}", endpoint, listenerPk), null).get();
		Ledger.signAndSubmitRequest(pool, listenerWallet, listenerDid, attribRequest).get();

		Listener activeListener = Agent.agentListen(endpoint, incomingConnectionObserver).get();

		activeListener.agentAddIdentity(pool, listenerWallet, listenerDid).get();

		Agent.agentConnect(pool, senderWallet, senderDid, listenerDid, messageObserver).get();

		listenerWallet.closeWallet().get();
		Wallet.deleteWallet(listenerWalletName, null).get();

		trusteeWallet.closeWallet().get();
		Wallet.deleteWallet(trusteeWalletName, null).get();
	}

	@Test
	public void testAgentConnectWorksForAllDataInWalletPresent() throws Exception {
		String endpoint = "127.0.0.1:9706";

		SignusResults.CreateAndStoreMyDidResult myDid = Signus.createAndStoreMyDid(wallet, "{}").get();

		String identityJson = String.format("{\"did\":\"%s\", \"pk\":\"%s\", \"verkey\":\"%s\", \"endpoint\":\"%s\"}",
				myDid.getDid(), myDid.getPk(), myDid.getVerkey(), endpoint);
		Signus.storeTheirDid(wallet, identityJson).get();

		Listener activeListener = Agent.agentListen(endpoint, incomingConnectionObserver).get();

		activeListener.agentAddIdentity(pool, wallet, myDid.getDid()).get();

		Agent.agentConnect(pool, wallet, myDid.getDid(), myDid.getDid(), messageObserver).get();
	}
}