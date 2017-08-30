import org.hyperledger.indy.sdk.agent.Agent.Connection;
import org.hyperledger.indy.sdk.agent.Agent.Listener;
import org.hyperledger.indy.sdk.agent.AgentObservers.ConnectionObserver;
import org.hyperledger.indy.sdk.agent.AgentObservers.MessageObserver;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.Assert;
import utils.PoolUtils;

import java.util.concurrent.CompletableFuture;

import static org.hyperledger.indy.sdk.agent.Agent.agentConnect;
import static org.hyperledger.indy.sdk.agent.Agent.agentListen;


class Agent {


	static void demo() throws Exception {
		System.out.println("Agent sample -> started");

		String listenerWalletName = "listenerWallet";
		String trusteeWalletName = "trusteeWallet";
		String endpoint = "127.0.0.1:9801";
		String message = "test";
		String trusteeSeed = "000000000000000000000000Trustee1";


		//1. Create and Open Pool
		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool pool = Pool.openPoolLedger(poolName, "{}").get();

		//2. Create and Open Listener Wallet
		Wallet.createWallet(poolName, listenerWalletName, "default", null, null).get();
		Wallet listenerWallet = Wallet.openWallet(listenerWalletName, null, null).get();

		//3. Create and Open Trustee Wallet
		Wallet.createWallet(poolName, trusteeWalletName, "default", null, null).get();
		Wallet trusteeWallet = Wallet.openWallet(trusteeWalletName, null, null).get();
		Wallet senderWallet = trusteeWallet;

		//4. Create My Did
		SignusResults.CreateAndStoreMyDidResult createMyDidResult = Signus.createAndStoreMyDid(listenerWallet, "{}").get();
		String listenerDid = createMyDidResult.getDid();
		String listenerVerkey = createMyDidResult.getVerkey();
		String listenerPk = createMyDidResult.getPk();

		//5. Create Their Did from Trustee seed
		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, trusteeSeed, null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(trusteeWallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();
		String senderDid = trusteeDid;

		// 6. Prepare and Send NYM request with signing
		String nymRequest = Ledger.buildNymRequest(trusteeDid, listenerDid, listenerVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, trusteeWallet, trusteeDid, nymRequest).get();

		// 7. Prepare and Send Attrib for listener (will be requested from ledger and used by sender at start connection)
		String attribRequest = Ledger.buildAttribRequest(listenerDid, listenerDid, null,
				String.format("{\"endpoint\":{\"ha\":\"%s\",\"verkey\":\"%s\"}}", endpoint, listenerPk), null).get();
		Ledger.signAndSubmitRequest(pool, listenerWallet, listenerDid, attribRequest).get();

		CompletableFuture<String> clientToServerMsgFuture = new CompletableFuture<String>();

		final MessageObserver messageObserver = new MessageObserver() {

			public void onMessage(Connection connection, String message) {

				System.out.println("Received message '" + message + "' on connection " + connection);
			}
		};

		final MessageObserver messageObserverForIncoming = new MessageObserver() {

			public void onMessage(Connection connection, String receivedMessage) {

				System.out.println("Received message '" + receivedMessage + "' on incoming connection " + connection);

				clientToServerMsgFuture.complete(receivedMessage);
			}
		};

		final ConnectionObserver incomingConnectionObserver = new ConnectionObserver() {

			public MessageObserver onConnection(Listener listener, Connection connection, String senderDid, String receiverDid) {

				System.out.println("New connection " + connection);

				return messageObserverForIncoming;
			}
		};

		// 8. start listener on endpoint
		Listener activeListener = agentListen(endpoint, incomingConnectionObserver).get();

		// 9. Allow listener accept incoming connection for specific DID (listener_did)
		activeListener.agentAddIdentity(pool, listenerWallet, listenerDid).get();

		// 10. Initiate connection from sender to listener
		Connection connection = agentConnect(pool, senderWallet, senderDid, listenerDid, messageObserver).get();

		// 11. Send test message from sender to listener
		connection.agentSend(message).get();

		Assert.assertEquals(message, clientToServerMsgFuture.get());

		// 12. Close connection
		connection.agentCloseConnection();

		// 13. Close listener
		activeListener.agentCloseListener();

		// 14. Close and delete Listener Wallet
		listenerWallet.closeWallet().get();
		Wallet.deleteWallet(listenerWalletName, null).get();

		// 15. Close and delete Sender Wallet
		trusteeWallet.closeWallet().get();
		Wallet.deleteWallet(trusteeWalletName, null).get();

		// 16. Close Pool
		pool.closePoolLedger().get();

		// 17. Delete Pool ledger config
		Pool.deletePoolLedgerConfig(poolName).get();

		System.out.println("Agent sample -> completed");
	}
}