package org.hyperledger.indy.sdk;

import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;


public class IndyIntegrationTestWithPoolAndSingleWallet extends IndyIntegrationTest {

	public Pool pool;
	public Wallet wallet;
	public String poolName;

	@Before
	public void createPoolAndWallet() throws Exception {
		poolName = PoolUtils.createPoolLedgerConfig();
		pool = Pool.openPoolLedger(poolName, null).get();

		Wallet.createWallet(poolName, WALLET, TYPE, null, null).get();
		this.wallet = Wallet.openWallet(WALLET, null, null).get();
	}

	@After
	public void deletePoolAndWallet() throws Exception {
		pool.closePoolLedger().get();
		wallet.closeWallet().get();
		Wallet.deleteWallet(WALLET, null).get();
	}
}
