package org.hyperledger.indy.sdk;

import java.io.File;
import java.util.concurrent.Future;

import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.OpenPoolLedgerJSONParameter;
import org.hyperledger.indy.sdk.pool.PoolResults.OpenPoolLedgerResult;
import org.junit.Assert;

import junit.framework.TestCase;

public class PoolTest extends TestCase {

	@Override
	protected void setUp() throws Exception {

		if (! LibIndy.isInitialized()) LibIndy.init(new File("./lib/libindy.so"));
	}

	@Override
	protected void tearDown() throws Exception {

	}

	public void testPool() throws Exception {

/*		CreatePoolLedgerConfigOptions config1 = new CreatePoolLedgerConfigOptions(null);
		Future<CreatePoolLedgerConfigResult> future1 = Pool.createPoolLedgerConfig("myconfig", config1);
		CreatePoolLedgerConfigResult result1 = future1.get();
		Assert.assertNotNull(result1);*/

		OpenPoolLedgerJSONParameter config2 = new OpenPoolLedgerJSONParameter(null, null, null);
		Future<OpenPoolLedgerResult> future2 = Pool.openPoolLedger("myconfig", config2);
		OpenPoolLedgerResult result2 = future2.get();
		Assert.assertNotNull(result2);
	}
}
