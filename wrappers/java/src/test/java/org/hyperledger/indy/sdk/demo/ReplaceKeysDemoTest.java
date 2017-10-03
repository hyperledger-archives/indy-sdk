package org.hyperledger.indy.sdk.demo;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class ReplaceKeysDemoTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String walletName = "signusWallet";
	private String schemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\",\"attr_names\": [\"name\", \"male\"]}";


	@Before
	public void createWalletWithDid() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();
		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		Wallet.createWallet(poolName, walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();
	}

	@After
	public void deleteWallet() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
		pool.closePoolLedger().get();
	}

	@Test
	public void testReplaceKeysDemoWorks() throws Exception {
		// 1. Create My Did
		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = result.getDid();
		String myVerkey = result.getVerkey();

		// 2. Create Their Did from Trustee1 seed
		SignusJSONParameters.CreateAndStoreMyDidJSONParameter theirDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		CreateAndStoreMyDidResult createTheirDidResult = Signus.createAndStoreMyDid(wallet, theirDidJson.toJson()).get();
		String trusteeDid = createTheirDidResult.getDid();

		// 3. Build and send Nym Request
		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		// 4. Start replacing of keys
		SignusResults.ReplaceKeysStartResult newKeys = Signus.replaceKeysStart(wallet, myDid, "{}").get();
		String newVerkey = newKeys.getVerkey();

		// 5. Build and send Nym Request with new key
		nymRequest = Ledger.buildNymRequest(myDid, myDid, newVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, nymRequest).get();

		// 6. Apply replacing of keys
		Signus.replaceKeysApply(wallet, myDid).get();

		// 7. Send schema request
		String schemaRequest = Ledger.buildSchemaRequest(myDid, schemaData).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, schemaRequest).get();
	}

	@Test
	public void testReplaceKeysWithoutNymTransaction() throws Exception {
		// 1. Create My Did
		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = result.getDid();
		String myVerkey = result.getVerkey();

		// 2. Create Their Did from Trustee1 seed
		SignusJSONParameters.CreateAndStoreMyDidJSONParameter theirDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		CreateAndStoreMyDidResult createTheirDidResult = Signus.createAndStoreMyDid(wallet, theirDidJson.toJson()).get();
		String trusteeDid = createTheirDidResult.getDid();

		// 3. Build and send Nym Request
		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		// 4. Start replacing of keys
		Signus.replaceKeysStart(wallet, myDid, "{}").get();

		// 5. Apply replacing of keys
		Signus.replaceKeysApply(wallet, myDid).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		// 6. Send schema request
		String schemaRequest = Ledger.buildSchemaRequest(myDid, schemaData).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, schemaRequest).get();
	}
}
