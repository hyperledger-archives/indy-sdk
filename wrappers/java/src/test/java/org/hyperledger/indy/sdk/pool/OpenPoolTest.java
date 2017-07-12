package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.OpenPoolLedgerJSONParameter;
import org.hyperledger.indy.sdk.pool.PoolResults.OpenPoolLedgerResult;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.junit.Test;

import static org.junit.Assert.assertNotNull;

public class OpenPoolTest extends IndyIntegrationTest {

	@Test
	public void testOpenPoolWorks() throws Exception {
		StorageUtils.cleanupStorage();

		String poolName = PoolUtils.createPoolLedgerConfig();

		OpenPoolLedgerJSONParameter config2 = new OpenPoolLedgerJSONParameter(null, null, null);
		OpenPoolLedgerResult openResult = Pool.openPoolLedger(poolName, config2).get();

		assertNotNull(openResult);

		openResult.getPool().closePoolLedger();
		StorageUtils.cleanupStorage();
	}

	@Test
	public void testOpenPoolWorksForTwoNodes() throws Exception {
		StorageUtils.cleanupStorage();

		String poolName = PoolUtils.createPoolLedgerConfig(2);

		OpenPoolLedgerJSONParameter config2 = new OpenPoolLedgerJSONParameter(null, null, null);
		OpenPoolLedgerResult openResult = Pool.openPoolLedger(poolName, config2).get();

		assertNotNull(openResult);

		openResult.getPool().closePoolLedger();
		StorageUtils.cleanupStorage();
	}

	@Test
	public void testOpenPoolWorksForThreeNodes() throws Exception {
		StorageUtils.cleanupStorage();

		String poolName = PoolUtils.createPoolLedgerConfig(3);

		OpenPoolLedgerJSONParameter config2 = new OpenPoolLedgerJSONParameter(null, null, null);
		OpenPoolLedgerResult openResult = Pool.openPoolLedger(poolName, config2).get();

		assertNotNull(openResult);

		openResult.getPool().closePoolLedger();
		StorageUtils.cleanupStorage();
	}

}
