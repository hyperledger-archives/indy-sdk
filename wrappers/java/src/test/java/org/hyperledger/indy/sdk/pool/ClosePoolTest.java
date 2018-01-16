package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class ClosePoolTest extends IndyIntegrationTest {

	@Test
	public void testClosePoolWorks() throws Exception {
		Pool pool = PoolUtils.createAndOpenPoolLedger();
		assertNotNull(pool);
		openedPools.add(pool);

		pool.closePoolLedger().get();
		openedPools.remove(pool);
	}

	@Test
	public void testClosePoolWorksForTwice() throws Exception {
		thrown.expectCause(isA(InvalidPoolException.class));

		Pool pool = PoolUtils.createAndOpenPoolLedger();
		assertNotNull(pool);
		openedPools.add(pool);

		pool.closePoolLedger().get();
		openedPools.remove(pool);
		pool.closePoolLedger().get();
	}

	@Test
	public void testClosePoolWorksForReopenAfterClose() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();

		Pool pool = Pool.openPoolLedger(poolName, null).get();
		assertNotNull(pool);

		pool.closePoolLedger().get();

		pool = Pool.openPoolLedger(poolName, null).get();
		openedPools.add(pool);
	}

	@Test
	public void testAutoCloseWorks() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();
		try (Pool pool = Pool.openPoolLedger(poolName, null).get()) {
			assertNotNull(pool);
		}
		Pool pool = Pool.openPoolLedger(poolName, null).get();
		openedPools.add(pool);
	}
}
