package org.hyperledger.indy.sdk;

import java.io.File;

import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.OpenPoolLedgerJSONParameter;
import org.junit.Assert;

import junit.framework.TestCase;

public class LedgerTest extends TestCase {

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

	public void testLedger() throws Exception {

		String result1 = Ledger.buildGetDdoRequest("did:sov:21tDAKCERh95uGgKbJNHYp", "did:sov:1yvXbmgPoUm4dl66D7KhyD", "{}").get();
		Assert.assertNotNull(result1);

		String result2 = Ledger.buildGetNymRequest("did:sov:21tDAKCERh95uGgKbJNHYp", "did:sov:1yvXbmgPoUm4dl66D7KhyD").get();
		Assert.assertNotNull(result2);
	}
}
