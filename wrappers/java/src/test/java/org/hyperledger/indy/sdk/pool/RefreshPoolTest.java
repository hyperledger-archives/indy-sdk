package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.junit.Test;

import java.io.IOException;
import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertNotNull;

public class RefreshPoolTest extends IndyIntegrationTest {

	@Test
	public void testRefreshPoolWorks() throws IndyException, ExecutionException, InterruptedException, IOException {
		Pool pool = PoolUtils.createAndOpenPoolLedger();
		assertNotNull(pool);
		openedPools.add(pool);

		pool.refreshPoolLedger().get();
	}
}
