package org.hyperledger.indy.sdk;

import java.io.File;

import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.OpenPoolLedgerJSONParameter;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.Assert;

import junit.framework.TestCase;

public class WalletTest extends TestCase {

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

	public void testWallet() throws Exception {

		Wallet wallet;
		
		Wallet.createWallet("default", "mywallet", null, null, null).get();

		wallet = Wallet.openWallet("mywallet", null, null).get();
		Assert.assertNotNull(wallet);

		wallet.closeWallet().get();

		Wallet.deleteWallet("mywallet", null).get();
	}
}
