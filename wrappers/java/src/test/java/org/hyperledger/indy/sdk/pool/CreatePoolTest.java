package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.junit.Test;

import java.io.File;

public class CreatePoolTest extends IndyIntegrationTest {

	@Test
	public void testCreatePoolWorks() throws Exception {
		StorageUtils.cleanupStorage();

		File genesisTxnFile = PoolUtils.createGenesisTxnFile("genesis.txn");

		PoolJSONParameters.CreatePoolLedgerConfigJSONParameter createPoolLedgerConfigJSONParameter
				= new PoolJSONParameters.CreatePoolLedgerConfigJSONParameter(genesisTxnFile.getAbsolutePath());
		Pool.createPoolLedgerConfig("testCreatePoolWorks", createPoolLedgerConfigJSONParameter).get();

		StorageUtils.cleanupStorage();
	}
}
