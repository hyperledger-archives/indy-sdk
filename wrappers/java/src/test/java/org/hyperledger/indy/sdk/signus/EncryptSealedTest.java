package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertNotNull;

public class EncryptSealedTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String did;
	private String verkey;
	private String walletName = "signusWallet";
	private byte[] msg = "{\"reqId\":1496822211362017764}".getBytes();

	@Before
	public void before() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		pool = Pool.openPoolLedger(poolName, config.toJson()).get();

		Wallet.createWallet(poolName, walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, false);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		String trusteeDid = result.getDid();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson2 =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, null);

		CreateAndStoreMyDidResult nym = Signus.createAndStoreMyDid(wallet, didJson2.toJson()).get();
		did = nym.getDid();
		verkey = nym.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, did, verkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();
	}

	@After
	public void after() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
		pool.closePoolLedger().get();
	}

	@Test
	public void testEncryptSealedWorksForPkCachedInWallet() throws Exception {
		String identityJson = String.format("{\"did\":\"%s\",\"verkey\":\"%s\"}", did, verkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		byte[] encryptResult = Signus.encryptSealed(wallet, pool, did, msg).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptSealedWorksForGetPkFromLedger() throws Exception {
		String identityJson = String.format("{\"did\":\"%s\"}", did);
		Signus.storeTheirDid(wallet, identityJson).get();

		byte[] encryptResult = Signus.encryptSealed(wallet, pool, did, msg).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptSealedWorksForGetNymFromLedger() throws Exception {
		byte[] encryptResult = Signus.encryptSealed(wallet, pool, did, msg).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptSealedWorksForNotFoundNym() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidState));
		
		Signus.encryptSealed(wallet, pool, "8wZcEriaNLNKtteJvx7f8i", msg).get();
	}
}
