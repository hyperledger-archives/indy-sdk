package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.OpenPoolLedgerJSONParameter;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.junit.Test;

import java.util.HashSet;

import static org.junit.Assert.assertNotNull;

public class OpenPoolTest extends IndyIntegrationTest {

	@Test
	public void testOpenPoolWorksForNullConfig() throws Exception {
		StorageUtils.cleanupStorage();

		String poolName = PoolUtils.createPoolLedgerConfig();

		Pool pool = Pool.openPoolLedger(poolName, null).get();

		assertNotNull(pool);
		openedPools.add(pool);

		afterEach();
	}

	@Test
	public void testOpenPoolWorksForConfig() throws Exception {
		StorageUtils.cleanupStorage();

		String poolName = PoolUtils.createPoolLedgerConfig();

		OpenPoolLedgerJSONParameter config = new OpenPoolLedgerJSONParameter(true, null, null);
		Pool pool = Pool.openPoolLedger(poolName, config.toJson()).get();

		assertNotNull(pool);
		openedPools.add(pool);

		afterEach();
	}

	@Test
	public void testOpenPoolWorksForTwice() throws Exception {
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.PoolLedgerInvalidPoolHandle));

		StorageUtils.cleanupStorage();

		try {
			String poolName = PoolUtils.createPoolLedgerConfig();

			Pool pool1 = Pool.openPoolLedger(poolName, null).get();
			assertNotNull(pool1);
			openedPools.add(pool1);

			Pool.openPoolLedger(poolName, null).get();
		} finally {
			afterEach();
		}
	}

	@Test
	public void testOpenPoolWorksForTwoNodes() throws Exception {
		StorageUtils.cleanupStorage();

		String poolName = PoolUtils.createPoolLedgerConfig(2);

		Pool pool = Pool.openPoolLedger(poolName, null).get();

		assertNotNull(pool);
		openedPools.add(pool);

		afterEach();
	}

	@Test
	public void testOpenPoolWorksForThreeNodes() throws Exception {
		StorageUtils.cleanupStorage();

		String poolName = PoolUtils.createPoolLedgerConfig(3);

		Pool pool = Pool.openPoolLedger(poolName, null).get();

		assertNotNull(pool);
		openedPools.add(pool);

		afterEach();
	}

}
