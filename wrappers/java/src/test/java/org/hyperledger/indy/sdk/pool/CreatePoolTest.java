package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.junit.Test;

import java.io.File;

import static org.junit.Assert.assertTrue;

public class CreatePoolTest extends IndyIntegrationTest {

	@Test
	public void testCreatePoolWorksWithoutConfig() throws Exception {
		StorageUtils.cleanupStorage();

		File file = new File("testCreatePoolWorks.txn");
		file.deleteOnExit();
		assertTrue(file.createNewFile());

		Pool.createPoolLedgerConfig("testCreatePoolWorks", null).get();

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testCreatePoolWorksForConfigJSON() throws Exception {
		StorageUtils.cleanupStorage();

		File genesisTxnFile = PoolUtils.createGenesisTxnFile("genesis.txn");

		PoolJSONParameters.CreatePoolLedgerConfigJSONParameter createPoolLedgerConfigJSONParameter
				= new PoolJSONParameters.CreatePoolLedgerConfigJSONParameter(genesisTxnFile.getAbsolutePath());
		Pool.createPoolLedgerConfig("testCreatePoolWorks", createPoolLedgerConfigJSONParameter.toJson()).get();

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testCreatePoolWorksForEmptyName() throws Exception {
		thrown.expect(new ErrorCodeMatcher(ErrorCode.CommonInvalidParam2));

		StorageUtils.cleanupStorage();

		File genesisTxnFile = PoolUtils.createGenesisTxnFile("genesis.txn");

		PoolJSONParameters.CreatePoolLedgerConfigJSONParameter createPoolLedgerConfigJSONParameter
				= new PoolJSONParameters.CreatePoolLedgerConfigJSONParameter(genesisTxnFile.getAbsolutePath());
		Pool.createPoolLedgerConfig("", createPoolLedgerConfigJSONParameter.toJson()).get();

		StorageUtils.cleanupStorage();
	}
}
