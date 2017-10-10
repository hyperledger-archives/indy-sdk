package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.junit.Test;

import java.io.File;
import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertTrue;

public class CreatePoolTest extends IndyIntegrationTest {

	@Test
	public void testCreatePoolWorksForNullConfig() throws Exception {
		PoolUtils.createGenesisTxnFile("testCreatePoolWorks.txn");
		Pool.createPoolLedgerConfig("testCreatePoolWorks", null).get();
	}

	@Test
	public void testCreatePoolWorksForConfigJSON() throws Exception {
		File genesisTxnFile = PoolUtils.createGenesisTxnFile("genesis.txn");

		PoolJSONParameters.CreatePoolLedgerConfigJSONParameter createPoolLedgerConfigJSONParameter
				= new PoolJSONParameters.CreatePoolLedgerConfigJSONParameter(genesisTxnFile.getAbsolutePath());
		Pool.createPoolLedgerConfig("testCreatePoolWorks", createPoolLedgerConfigJSONParameter.toJson()).get();
	}

	@Test
	public void testCreatePoolWorksForEmptyName() throws Exception {
		thrown.expect(new ErrorCodeMatcher(ErrorCode.CommonInvalidParam2));

		File genesisTxnFile = PoolUtils.createGenesisTxnFile("genesis.txn");

		PoolJSONParameters.CreatePoolLedgerConfigJSONParameter createPoolLedgerConfigJSONParameter
				= new PoolJSONParameters.CreatePoolLedgerConfigJSONParameter(genesisTxnFile.getAbsolutePath());
		Pool.createPoolLedgerConfig("", createPoolLedgerConfigJSONParameter.toJson()).get();
	}

	@Test
	public void testCreatePoolWorksForTwice() throws Exception {
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.PoolLedgerConfigAlreadyExistsError));

		File genesisTxnFile = PoolUtils.createGenesisTxnFile("genesis.txn");

		PoolJSONParameters.CreatePoolLedgerConfigJSONParameter createPoolLedgerConfigJSONParameter
				= new PoolJSONParameters.CreatePoolLedgerConfigJSONParameter(genesisTxnFile.getAbsolutePath());
		Pool.createPoolLedgerConfig("pool1", createPoolLedgerConfigJSONParameter.toJson()).get();
		Pool.createPoolLedgerConfig("pool1", createPoolLedgerConfigJSONParameter.toJson()).get();
	}
}
