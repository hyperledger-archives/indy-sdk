package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertTrue;

public class NodeRequestsTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String walletName = "ledgerWallet";
	private String identifier = "Th7MpTaRZVRYnPiabds81Y";
	private String dest = "Th7MpTaRZVRYnPiabds81Y";

	@Before
	public void openPool() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();
		pool = Pool.openPoolLedger(poolName, null).get();

		Wallet.createWallet(poolName, walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();
	}

	@After
	public void closePool() throws Exception {
		pool.closePoolLedger().get();
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testBuildNodeRequestWorks() throws Exception {

		String data = "{\"node_ip\":\"10.0.0.100\"," +
				"\"node_port\":910," +
				"\"client_ip\":\"10.0.0.100\"," +
				"\"client_port\":911," +
				"\"alias\":\"some\"," +
				"\"services\":[\"VALIDATOR\"]," +
				"\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"0\"," +
				"\"dest\":\"%s\"," +
				"\"data\":%s" +
				"}", identifier, dest, data);

		String nodeRequest = Ledger.buildNodeRequest(identifier, dest, data).get();

		assertTrue(nodeRequest.replace("\\", "").contains(expectedResult));
	}

	@Test
	public void testSendNodeRequestWorksWithoutSignature() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Steward1", null, null);

		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String did = didResult.getDid();

		String data = "{\"node_ip\":\"10.0.0.100\"," +
				"\"node_port\":910," +
				"\"client_ip\":\"10.0.0.100\"," +
				"\"client_port\":910," +
				"\"alias\":\"some\"," +
				"\"services\":[\"VALIDATOR\"]," +
				"\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

		String nodeRequest = Ledger.buildNodeRequest(did, did, data).get();
		Ledger.submitRequest(pool, nodeRequest).get();
	}

	@Test
	public void testBuildNodeRequestWorksForWrongServiceType() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String data = "{\"node_ip\":\"10.0.0.100\"," +
				"\"node_port\":910," +
				"\"client_ip\":\"10.0.0.100\"," +
				"\"client_port\":911," +
				"\"alias\":\"some\"," +
				"\"services\":[\"SERVICE\"]" +
				"\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

		Ledger.buildNodeRequest(identifier, dest, data).get();
	}

	@Test
	public void testBuildNodeRequestWorksForMissedField() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String data = "{\"node_ip\":\"10.0.0.100\"," +
				"\"node_port\":910," +
				"\"client_ip\":\"10.0.0.100\"," +
				"\"client_port\":910," +
				"\"services\":[\"VALIDATOR\"]}";

		Ledger.buildNodeRequest(identifier, dest, data).get();
	}

	@Test
	public void testSendNodeRequestWorksForWrongRole() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String did = didResult.getDid();

		String data = "{\"node_ip\":\"10.0.0.100\"," +
				"\"node_port\":910," +
				"\"client_ip\":\"10.0.0.100\"," +
				"\"client_port\":911," +
				"\"alias\":\"some\"," +
				"\"services\":[\"VALIDATOR\"]," +
				"\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

		String nodeRequest = Ledger.buildNodeRequest(did, did, data).get();
		Ledger.signAndSubmitRequest(pool, wallet, did, nodeRequest).get();
	}

	@Test
	@Ignore
	public void testSendNodeRequestWorksForNewSteward() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = didResult.getDid();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter myDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, null, null, null);

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, myDidJson.toJson()).get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String role = "STEWARD";

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, role).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		String data = "{\"node_ip\":\"10.0.0.100\"," +
				"\"node_port\":910," +
				"\"client_ip\":\"10.0.0.100\"," +
				"\"client_port\":911," +
				"\"alias\":\"some\"," +
				"\"services\":[\"VALIDATOR\"]}";

		String dest = "A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y";

		String nodeRequest = Ledger.buildNodeRequest(myDid, dest, data).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, nodeRequest).get();
	}
}
