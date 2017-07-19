package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.wallet.Wallet;

import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;
import org.json.*;

import java.util.concurrent.ExecutionException;

public class IssuerCreateAndStoreClaimDefinitionTest extends AnoncredsIntegrationTest {

	private Wallet wallet;
	private String walletName = "createAndStoreClaimDefWallet";

	@Before
	public void createWallet() throws Exception {
		Wallet.createWallet("default", walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();
	}

	@After
	public void deleteWallet() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	private String issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
	private String gvtSchemaJson = "{\n" +
			"                    \"seqNo\":1,\n" +
			"                    \"data\": {\n" +
			"                        \"name\":\"gvt\",\n" +
			"                        \"version\":\"1.0\",\n" +
			"                        \"keys\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
			"                    }\n" +
			"                 }";

	@Test
	public void testIssuerCreateAndStoreClaimDefWorks() throws Exception {

		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, null, false).get();
		assertNotNull(claimDef);

		JSONObject claimDefObject = new JSONObject(claimDef);

		assertEquals(claimDefObject.getJSONObject("data").getJSONObject("primary").getJSONObject("r").length(), 4);
		assertTrue(claimDefObject.getJSONObject("data").getJSONObject("primary").getString("n").length() > 0);
		assertTrue(claimDefObject.getJSONObject("data").getJSONObject("primary").getString("s").length() > 0);
		assertTrue(claimDefObject.getJSONObject("data").getJSONObject("primary").getString("z").length() > 0);
		assertTrue(claimDefObject.getJSONObject("data").getJSONObject("primary").getString("rms").length() > 0);
		assertTrue(claimDefObject.getJSONObject("data").getJSONObject("primary").getString("rctxt").length() > 0);
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForInvalidSchemaJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String schema = "{\"seqNo\":1, \"name\":\"name\",\"version\":\"1.0\", \"keys\":[\"name\"]}";

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, schema, null, false).get();
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForEmptyKeys() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String schema = "{\n" +
				"                    \"seqNo\":1,\n" +
				"                    \"data\": {\n" +
				"                        \"name\":\"gvt\",\n" +
				"                        \"version\":\"1.0\",\n" +
				"                        \"keys\":[]\n" +
				"                    }\n" +
				"                 }";

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, schema, null, false).get();
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForCorrectCryptoType() throws Exception {

		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, "CL", false).get();
		assertNotNull(claimDef);

		JSONObject claimDefObject = new JSONObject(claimDef);

		assertEquals(claimDefObject.getJSONObject("data").getJSONObject("primary").getJSONObject("r").length(), 4);
		assertTrue(claimDefObject.getJSONObject("data").getJSONObject("primary").getString("n").length() > 0);
		assertTrue(claimDefObject.getJSONObject("data").getJSONObject("primary").getString("s").length() > 0);
		assertTrue(claimDefObject.getJSONObject("data").getJSONObject("primary").getString("z").length() > 0);
		assertTrue(claimDefObject.getJSONObject("data").getJSONObject("primary").getString("rms").length() > 0);
		assertTrue(claimDefObject.getJSONObject("data").getJSONObject("primary").getString("rctxt").length() > 0);
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForInvalidCryptoType() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, "type", false).get();
	}
}
