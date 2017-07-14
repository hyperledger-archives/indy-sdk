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

public class DeletePoolTest extends IndyIntegrationTest {

	@Test
	public void testDeletePoolWorks() throws InterruptedException, ExecutionException, IndyException, IOException {
		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool.deletePoolLedgerConfig(poolName).get();
	}

	@Test
	public void testDeletePoolWorksForOpened() throws InterruptedException, ExecutionException, IndyException, IOException {
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidState));

		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool pool = Pool.openPoolLedger(poolName, null).get();
		assertNotNull(pool);
		openedPools.add(pool);
		Pool.deletePoolLedgerConfig(poolName).get();
	}
}
