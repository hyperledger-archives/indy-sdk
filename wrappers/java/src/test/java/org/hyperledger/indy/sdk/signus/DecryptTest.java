package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hyperledger.indy.sdk.utils.PoolUtils.DEFAULT_POOL_NAME;
import static org.junit.Assert.assertEquals;

public class DecryptTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String trusteeDid;
	private String trusteeVerkey;
	private String myDid;
	private String myVerkey;
	private String walletName = "signusWallet";
	private String msg = "{\"reqId\":1496822211362017764}";
	private String encrypted_message = "4SWFzd3sx7xNemZEtktt3s558Fa28fGbauAZv9NRQjQhHq9bwT8uBnACQJAKzZ";
	private String nonce = "Dd3vSQgDdADJGoxb6BPcWU6wkEMqSeFwv";
	private String identityJsonTemplate = "{\"did\":\"%s\",\"verkey\":\"%s\"}";

	@Before
	public void createWalletWithDid() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config2 =
				new PoolJSONParameters.OpenPoolLedgerJSONParameter(null, null, null);
		pool = Pool.openPoolLedger(poolName, config2.toJson()).get();

		Wallet.createWallet(DEFAULT_POOL_NAME, walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, false);

		CreateAndStoreMyDidResult trusteeNym = Signus.createAndStoreMyDid(wallet, didJson.toJson()).get();
		trusteeDid = trusteeNym.getDid();
		trusteeVerkey = trusteeNym.getVerkey();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson2 =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, null);

		CreateAndStoreMyDidResult myNym = Signus.createAndStoreMyDid(wallet, didJson2.toJson()).get();
		myDid = myNym.getDid();
		myVerkey = myNym.getVerkey();
	}

	@After
	public void deleteWallet() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
		pool.closePoolLedger().get();
	}

	@Test
	public void testDecryptWorks() throws Exception {
		String identityJson = String.format(identityJsonTemplate, trusteeDid, trusteeVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		String decryptedMessage = Signus.decrypt(wallet, myDid, trusteeDid, encrypted_message, nonce).get();
		assertEquals(msg, decryptedMessage);
	}

	@Test
	public void testDecryptWorksForOtherCoder() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String identityJson = String.format(identityJsonTemplate, trusteeDid, trusteeVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		identityJson = String.format(identityJsonTemplate, myDid, myVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		SignusResults.EncryptResult encryptResult = Signus.encrypt(wallet, pool, myDid, myDid, msg).get();

		Signus.decrypt(wallet, myDid, trusteeDid, encryptResult.getEncryptedMessage(), encryptResult.getNonce()).get();
	}

	@Test
	public void testDecryptWorksForNonceNotCorrespondMessage() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String identityJson = String.format(identityJsonTemplate, trusteeDid, trusteeVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		String nonce = "acS2SQgDdfE3Goxa1AhcWCa4kEMqSelv7";

		Signus.decrypt(wallet, myDid, trusteeDid, encrypted_message, nonce).get();
	}

	@Test
	public void testDecryptWorksForUnknownMyDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletNotFoundError));

		String identityJson = String.format(identityJsonTemplate, trusteeDid, trusteeVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		Signus.decrypt(wallet, "unknowDid", trusteeDid, encrypted_message, nonce).get();
	}
}
