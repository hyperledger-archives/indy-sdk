package org.hyperledger.indy.sdk;

import java.util.concurrent.Future;

import org.hyperledger.indy.sdk.utils.InitHelper;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.OpenPoolLedgerJSONParameter;
import org.junit.Assert;

import junit.framework.TestCase;

public class PoolTest extends TestCase {

	@Override
	protected void setUp() throws Exception {
		InitHelper.init();
	}

	@Override
	protected void tearDown() throws Exception {

	}

	public void testPool() throws Exception {

/*		CreatePoolLedgerConfigOptions config1 = new CreatePoolLedgerConfigOptions(null);
		Future<CreatePoolLedgerConfigResult> future1 = Pool.createPoolLedgerConfig("myconfig", config1.toJson());
		CreatePoolLedgerConfigResult result1 = future1.get();
		Assert.assertNotNull(result1);*/

		OpenPoolLedgerJSONParameter config2 = new OpenPoolLedgerJSONParameter(null, null, null);
		Future<Pool> future2 = Pool.openPoolLedger("myconfig", config2.toJson());
		Pool result2 = future2.get();
		Assert.assertNotNull(result2);
	}
}
