package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.agent.Agent.Connection;
import org.hyperledger.indy.sdk.agent.Agent.Listener;
import org.hyperledger.indy.sdk.agent.AgentObservers.ConnectionObserver;
import org.hyperledger.indy.sdk.agent.AgentObservers.MessageObserver;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.utils.InitHelper;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.BeforeClass;
import org.junit.Rule;
import org.junit.rules.ExpectedException;
import org.junit.rules.Timeout;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.TimeUnit;


public class AgentIntegrationTest {

	@Rule
	public ExpectedException thrown = ExpectedException.none();

	@Rule
	public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

	static Wallet wallet;
	static Pool pool;
	String poolName;
	private String walletName = "agentWallet";

	@BeforeClass
	public static void init() throws Exception {
		InitHelper.init();
	}

	@Before
	public void setUp() throws Exception {
		StorageUtils.cleanupStorage();

		poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		Wallet.createWallet(poolName, walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();
	}

	@After
	public void tearDown() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();

		pool.closePoolLedger().get();
		StorageUtils.cleanupStorage();
	}

	static final AgentObservers.MessageObserver messageObserver = new AgentObservers.MessageObserver() {

		public void onMessage(Connection connection, String message) {

			System.out.println("Received message '" + message + "' on connection " + connection);
		}
	};

	private static final AgentObservers.MessageObserver messageObserverForIncoming = new AgentObservers.MessageObserver() {

		public void onMessage(Connection connection, String message) {

			System.out.println("Received message '" + message + "' on incoming connection " + connection);
		}
	};

	static final AgentObservers.ConnectionObserver incomingConnectionObserver = new AgentObservers.ConnectionObserver() {

		public AgentObservers.MessageObserver onConnection(Listener listener, Connection connection, String senderDid, String receiverDid) {

			System.out.println("New connection " + connection);

			return messageObserverForIncoming;
		}
	};
}