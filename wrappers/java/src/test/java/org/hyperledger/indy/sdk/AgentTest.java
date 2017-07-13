package org.hyperledger.indy.sdk;

import java.io.File;

import org.hyperledger.indy.sdk.agent.Agent;
import org.hyperledger.indy.sdk.agent.Agent.Connection;
import org.hyperledger.indy.sdk.agent.Agent.Listener;
import org.hyperledger.indy.sdk.agent.AgentObservers.ConnectionObserver;
import org.hyperledger.indy.sdk.agent.AgentObservers.ListenerObserver;
import org.hyperledger.indy.sdk.agent.AgentObservers.MessageObserver;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.OpenPoolLedgerJSONParameter;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.Assert;

import junit.framework.TestCase;

public class AgentTest extends TestCase {

	private Pool pool;

	@Override
	protected void setUp() throws Exception {

		if (! LibIndy.isInitialized()) LibIndy.init(new File("./lib/libindy.so"));

		OpenPoolLedgerJSONParameter openPoolLedgerOptions = new OpenPoolLedgerJSONParameter(null, null, null);
		this.pool = Pool.openPoolLedger("myconfig", openPoolLedgerOptions.toJson()).get();
	}

	@Override
	protected void tearDown() throws Exception {

		this.pool.closePoolLedger();
	}

	public void testAgent() throws Exception {

		Pool pool = Pool.openPoolLedger("myconfig", null).get();
		Wallet wallet = Wallet.openWallet("mywallet", null, null).get();
		Assert.assertNotNull(pool);
		Assert.assertNotNull(wallet);

		final MessageObserver messageObserver = new MessageObserver() {

			public void onMessage(Connection connection, String message) {

				System.out.println("Received message '" + message + "' on connection " + connection);
			}
		};

		final ConnectionObserver connectionObserver = new ConnectionObserver() {

			public MessageObserver onConnection(Listener listener, Connection connection, String senderDid, String receiverDid) {

				System.out.println("New connection " + connection);

				return messageObserver;
			}
		};

		final ListenerObserver listenerObserver = new ListenerObserver() {

			public ConnectionObserver onListener(Listener listener) {

				System.out.println("New listener " + listener);

				return connectionObserver;
			}
		};

		Agent.agentConnect(pool, wallet, "did1", "did2", connectionObserver);

		Agent.agentListen("endpoint", listenerObserver);
	}
}
