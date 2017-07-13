package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.junit.Test;

import java.io.IOException;
import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertNotNull;

public class ClosePoolTest extends IndyIntegrationTest {

	@Test
	public void testClosePoolWorks() throws IndyException, ExecutionException, InterruptedException, IOException {
		Pool pool = PoolUtils.createAndOpenPoolLedger();
		assertNotNull(pool);
		openedPools.add(pool);

		pool.closePoolLedger().get();
		openedPools.remove(pool);
	}

	@Test
	public void testClosePoolWorksForTwice() throws IndyException, ExecutionException, InterruptedException, IOException {
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.PoolLedgerInvalidPoolHandle));

		Pool pool = PoolUtils.createAndOpenPoolLedger();
		assertNotNull(pool);
		openedPools.add(pool);

		pool.closePoolLedger().get();
		openedPools.remove(pool);
		pool.closePoolLedger().get();
	}

	@Test
	public void testClosePoolWorksForReopenAfterClose() throws IndyException, ExecutionException, InterruptedException, IOException {
		String poolName = PoolUtils.createPoolLedgerConfig();

		Pool pool = Pool.openPoolLedger(poolName, null).get();
		assertNotNull(pool);
		openedPools.add(pool);

		pool.closePoolLedger().get();

		Pool.openPoolLedger(poolName, null).get();
	}
}
