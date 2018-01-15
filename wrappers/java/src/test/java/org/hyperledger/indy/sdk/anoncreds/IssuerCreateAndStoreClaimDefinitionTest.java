package org.hyperledger.indy.sdk.anoncreds;

import static org.hamcrest.CoreMatchers.*;

import org.hyperledger.indy.sdk.InvalidStructureException;
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

	@Test
	public void testIssuerCreateAndStoreClaimDefWorks() throws Exception {

		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, null, false).get();
		assertNotNull(claimDef);

		JSONObject claimDefObject = new JSONObject(claimDef);
		JSONObject primary = claimDefObject.getJSONObject("data").getJSONObject("primary");

		assertEquals(4, primary.getJSONObject("r").length());
		assertTrue(primary.getString("n").length() > 0);
		assertTrue(primary.getString("s").length() > 0);
		assertTrue(primary.getString("z").length() > 0);
		assertTrue(primary.getString("rms").length() > 0);
		assertTrue(primary.getString("rctxt").length() > 0);
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForInvalidSchemaJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schema = "{" +
				"                   \"seqNo\":1, " +
				"                   \"name\":\"name\"," +
				"                   \"version\":\"1.0\"," +
				"                   \"attr_names\":[\"name\"]}";

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, schema, null, false).get();
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForEmptyKeys() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schema = "{\n" +
				"                    \"seqNo\":1,\n" +
				"                    \"identifier\":\"NcYxiDXkpYi6ov5FcYDi1e\",\n" +
				"                    \"data\": {\n" +
				"                        \"name\":\"gvt\",\n" +
				"                        \"version\":\"1.0\",\n" +
				"                        \"attr_names\":[]\n" +
				"                    }\n" +
				"                 }";

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, schema, null, false).get();
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForCorrectCryptoType() throws Exception {

		String claimDef = Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, "CL", false).get();
		assertNotNull(claimDef);

		JSONObject claimDefObject = new JSONObject(claimDef);
		JSONObject primary = claimDefObject.getJSONObject("data").getJSONObject("primary");

		assertEquals(4, primary.getJSONObject("r").length());
		assertTrue(primary.getString("n").length() > 0);
		assertTrue(primary.getString("s").length() > 0);
		assertTrue(primary.getString("z").length() > 0);
		assertTrue(primary.getString("rms").length() > 0);
		assertTrue(primary.getString("rctxt").length() > 0);
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForInvalidCryptoType() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, "type", false).get();
	}

	@Test
	public void testIssuerCreateAndStoreClaimDefWorksForDuplicate() throws Exception {
		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, null, false).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(ClaimDefAlreadyExistsException.class));

		Anoncreds.issuerCreateAndStoreClaimDef(wallet, issuerDid, gvtSchemaJson, null, false).get();
	}
}
