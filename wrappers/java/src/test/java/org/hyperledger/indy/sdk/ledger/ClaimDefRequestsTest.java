package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import org.junit.*;
import org.junit.rules.Timeout;

import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

import static org.junit.Assert.assertTrue;

public class ClaimDefRequestsTest extends IndyIntegrationTest {

	private Pool pool;
	private Wallet wallet;
	private String walletName = "ledgerWallet";

	@Rule
	public Timeout globalTimeout = new Timeout(1, TimeUnit.MINUTES);

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
	public void testBuildClaimDefRequestWorks() throws Exception {

		String identifier = "Th7MpTaRZVRYnPiabds81Y";
		String signature_type = "CL";
		int schema_seq_no = 1;
		String data = "{\"primary\":{\"n\":\"1\",\"s\":\"2\",\"rms\":\"3\",\"r\":{\"name\":\"1\"},\"rctxt\":\"1\",\"z\":\"1\"}}";

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"ref\":%d," +
				"\"data\":\"%s\"," +
				"\"type\":\"102\"," +
				"\"signature_type\":\"%s\"" +
				"}", identifier, schema_seq_no, data, signature_type);

		String claimDefRequest = Ledger.buildClaimDefTxn(identifier, schema_seq_no, signature_type, data).get();

		assertTrue(claimDefRequest.replace("\\", "").contains(expectedResult));
	}

	@Test
	public void testBuildGetClaimDefRequestWorks() throws Exception {

		String identifier = "Th7MpTaRZVRYnPiabds81Y";
		String origin = "Th7MpTaRZVRYnPiabds81Y";
		String signature_type = "CL";
		int ref = 1;

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"108\"," +
				"\"ref\":%d," +
				"\"signature_type\":\"%s\"," +
				"\"origin\":\"%s\"" +
				"}", identifier, ref, signature_type, origin);

		String getClaimDefRequest = Ledger.buildGetClaimDefTxn(identifier, ref, signature_type, origin).get();

		assertTrue(getClaimDefRequest.replace("\\", "").contains(expectedResult));
	}

	@Test
	public void testBuildClaimDefRequestWorksForInvalidJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String identifier = "Th7MpTaRZVRYnPiabds81Y";
		String signature_type = "CL";
		int schema_seq_no = 1;
		String data = "{\"primary\":{\"n\":\"1\",\"s\":\"2\",\"rms\":\"3\",\"r\":{\"name\":\"1\"}}}";

		Ledger.buildClaimDefTxn(identifier, schema_seq_no, signature_type, data).get();
	}

	@Test
	public void testClaimDefRequestsWorks() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee1", null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter myDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, null, null, null);

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, myDidJson.toJson()).get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		String schemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\",\"keys\": [\"name\", \"male\"]}";

		String schemaRequest = Ledger.buildSchemaRequest(myDid, schemaData).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, schemaRequest).get();

		String getSchemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
		String getSchemaRequest = Ledger.buildGetSchemaRequest(myDid, myDid, getSchemaData).get();
		String getSchemaResponse = Ledger.submitRequest(pool, getSchemaRequest).get();

		JSONObject schemaObj = new JSONObject(getSchemaResponse);

		int schemaSeqNo = schemaObj.getJSONObject("result").getInt("seqNo");
		String schemaJson = String.format("{\"seqNo\":%d,\"data\":%s}", schemaSeqNo, schemaData);

		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, myDid, schemaJson, null, false).get();

		JSONObject claimDefObj = new JSONObject(claimDef);

		String claimDefJson = String.format("%s", claimDefObj.getJSONObject("data"));
		String signatureType = claimDefObj.getString("signature_type");

		String claimDefRequest = Ledger.buildClaimDefTxn(myDid, schemaSeqNo, signatureType, claimDefJson).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, claimDefRequest).get();

		String getClaimDefRequest = Ledger.buildGetClaimDefTxn(myDid, schemaSeqNo, signatureType, claimDefObj.getString("origin")).get();
		String getClaimDefResponse = Ledger.submitRequest(pool, getClaimDefRequest).get();

		JSONObject getClaimDefResponseObj = new JSONObject(getClaimDefResponse);

		JSONObject expectedClaimDef = claimDefObj.getJSONObject("data").getJSONObject("primary");
		JSONObject actualClaimDef = getClaimDefResponseObj.getJSONObject("result").getJSONObject("data").getJSONObject("primary");

		Assert.assertEquals(expectedClaimDef.getString("n"), actualClaimDef.getString("n"));
		Assert.assertEquals(expectedClaimDef.getString("rms"), actualClaimDef.getString("rms"));
		Assert.assertEquals(expectedClaimDef.getString("rctxt"), actualClaimDef.getString("rctxt"));
		Assert.assertEquals(expectedClaimDef.getString("z"), actualClaimDef.getString("z"));
		Assert.assertEquals(expectedClaimDef.getString("n"), actualClaimDef.getString("n"));
		Assert.assertEquals(expectedClaimDef.getJSONObject("r").toString(), actualClaimDef.getJSONObject("r").toString());

	}

	@Test
	public void testClaimDefRequestWorksWithoutSignature() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.LedgerInvalidTransaction));

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter trusteeDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee1", null, null);

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, trusteeDidJson.toJson()).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter myDidJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, null, null, null);

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, myDidJson.toJson()).get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		String schemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\",\"keys\": [\"name\", \"male\"]}";

		String schemaRequest = Ledger.buildSchemaRequest(myDid, schemaData).get();
		String schemaResponse = Ledger.signAndSubmitRequest(pool, wallet, myDid, schemaRequest).get();

		JSONObject schemaObj = new JSONObject(schemaResponse);

		int schemaSeqNo = schemaObj.getJSONObject("result").getInt("seqNo");
		String schemaJson = String.format("{\"seqNo\":%d,\"data\":%s}", schemaSeqNo, schemaData);

		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, myDid, schemaJson, null, false).get();

		JSONObject claimDefObj = new JSONObject(claimDef);

		String claimDefJson = String.format("%s", claimDefObj.getJSONObject("data"));

		String claimDefRequest = Ledger.buildClaimDefTxn(myDid, schemaSeqNo, claimDefObj.getString("signature_type"), claimDefJson).get();
		Ledger.submitRequest(pool, claimDefRequest).get();
	}
}
