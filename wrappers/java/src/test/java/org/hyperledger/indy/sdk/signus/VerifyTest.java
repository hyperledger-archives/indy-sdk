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
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

import static junit.framework.TestCase.assertFalse;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

public class VerifyTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String trusteeDid;
	private String trusteeVerkey;
	private String identityJson;
	private String newDid;

	@Before
	public void createWalletWithDid() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 = new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		Wallet.createWallet(poolName, "signusWallet", "default", null, null).get();
		wallet = Wallet.openWallet("signusWallet", null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee1", null, false);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		assertNotNull(result);

		trusteeDid = result.getDid();
		trusteeVerkey = result.getVerkey();
	}

	@After
	public void deleteWallet() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet("signusWallet", null).get();
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

		String msg = "{\n" +
				"                \"reqId\":1496822211362017764,\n" +
				"                \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
				"                \"operation\":{\n" +
				"                    \"type\":\"1\",\n" +
				"                    \"dest\":\"VsKV7grR1BUE29mG2Fm2kX\",\n" +
				"                    \"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"\n" +
				"                },\n" +
				"                \"signature\":\"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW\"\n" +
				"            }";

		Boolean valid = Signus.verifySignature(wallet, pool, trusteeDid, msg).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForGetVerkeyFromLedger() throws Exception {
		createNewNymWithDidInLedger();
		identityJson = String.format("{\"did\":\"%s\"}", newDid);
		Signus.storeTheirDid(wallet, identityJson).get();

		String msg = "{\"reqId\":1496822211362017764,\n" +
				"\"signature\":\"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai\"}";

		Boolean valid = Signus.verifySignature(wallet, pool, newDid, msg).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForGetNymFromLedger() throws Exception {
		createNewNymWithDidInLedger();
		String msg = "{\"reqId\":1496822211362017764,\n" +
				"\"signature\":\"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai\"}";

		Boolean valid = Signus.verifySignature(wallet, pool, newDid, msg).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForInvalidMessageFormat() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String msg = "\"signature\":\"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW\"";

		Signus.verifySignature(wallet, pool, trusteeDid, msg).get();
	}

	@Test
	public void testVerifyWorksForMessageWithoutSignature() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String msg = "{\n" +
				"                \"reqId\":1496822211362017764,\n" +
				"                \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
				"                \"operation\":{\n" +
				"                    \"type\":\"1\",\n" +
				"                    \"dest\":\"VsKV7grR1BUE29mG2Fm2kX\",\n" +
				"                    \"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"\n" +
				"                },\n" +
				"            }";

		Signus.verifySignature(wallet, pool, trusteeDid, msg).get();
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

		String msg = "{\n" +
				"                \"reqId\":1496822211362017764,\n" +
				"                \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
				"                \"operation\":{\n" +
				"                    \"type\":\"1\",\n" +
				"                    \"dest\":\"VsKV7grR1BUE29mG2Fm2kX\",\n" +
				"                    \"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"\n" +
				"                }\n" +
				"            }";

		String signedMessage = Signus.sign(wallet, trusteeDid, msg).get();

		Boolean valid = Signus.verifySignature(wallet, pool, stewardDid, signedMessage).get();

		assertFalse(valid);
	}
}
