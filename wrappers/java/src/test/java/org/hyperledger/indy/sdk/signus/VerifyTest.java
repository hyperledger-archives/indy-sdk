package org.hyperledger.indy.sdk.signus;

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

import static junit.framework.TestCase.assertFalse;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class VerifyTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String trusteeDid;
	private String trusteeVerkey;
	private String identityJson;
	private String newDid;
	private String walletName = "signusWallet";

	@Before
	public void createWalletWithDid() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		Wallet.createWallet(poolName, walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, false);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		assertNotNull(result);

		trusteeDid = result.getDid();
		trusteeVerkey = result.getVerkey();
	}

	@After
	public void deleteWallet() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
		pool.closePoolLedger().get();
	}

	private void createNewNymWithDidInLedger() throws Exception {
		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "00000000000000000000000000000My1", null, null);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		newDid = result.getDid();
		String newVerkey = result.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, newDid, newVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();
	}

	@Test
	public void testVerifyWorksForVerkeyCachedInWallet() throws Exception {
		identityJson = String.format("{\"did\":\"%s\",\"verkey\":\"%s\"}", trusteeDid, trusteeVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		String msg = "{\"reqId\":1496822211362017764}";

		String signature = "R4Rj68n4HZosQqEc3oMUbQh7MtG8tH7WmXE2Mok8trHJ67CrzyqahZn5ziJy4nebRtq6Qi6fVH9JkvVCM85XjFa";

		Boolean valid = Signus.verifySignature(wallet, pool, trusteeDid, msg, signature).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForGetVerkeyFromLedger() throws Exception {
		createNewNymWithDidInLedger();
		Signus.storeTheirDid(wallet, String.format("{\"did\":\"%s\"}", newDid)).get();

		String msg = "{\"reqId\":1496822211362017764}";

		String signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

		Boolean valid = Signus.verifySignature(wallet, pool, newDid, msg, signature).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForGetNymFromLedger() throws Exception {
		createNewNymWithDidInLedger();
		String msg = "{\"reqId\":1496822211362017764}";

		String signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

		Boolean valid = Signus.verifySignature(wallet, pool, newDid, msg, signature).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForOtherSigner() throws Exception {
		identityJson = String.format("{\"did\":\"%s\", \"verkey\":\"%s\"}", trusteeDid, trusteeVerkey);

		Signus.storeTheirDid(wallet, identityJson).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Steward1", null, null);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		String stewardDid = result.getDid();
		String stewardVerkey = result.getVerkey();

		identityJson = String.format("{\"did\":\"%s\", \"verkey\":\"%s\"}", stewardDid, stewardVerkey);

		Signus.storeTheirDid(wallet, identityJson).get();

		String msg = "{\"reqId\":1496822211362017764}";

		String signature = Signus.sign(wallet, trusteeDid, msg).get();

		Boolean valid = Signus.verifySignature(wallet, pool, stewardDid, msg, signature).get();

		assertFalse(valid);
	}
}
