package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidJSONParameters;
import org.hyperledger.indy.sdk.did.DidResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertTrue;

public class SignRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	private DidJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
			new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

	@Test
	public void testSignWorks() throws Exception {

		String msg = "{\n" +
				"                \"reqId\":1496822211362017764,\n" +
				"                \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
				"                \"operation\":{\n" +
				"                    \"type\":\"1\",\n" +
				"                    \"dest\":\"VsKV7grR1BUE29mG2Fm2kX\",\n" +
				"                    \"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"\n" +
				"                }\n" +
				"            }";

		String expectedSignature = "\"signature\":\"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW\"";

		CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
		String did = result.getDid();

		String signedMessage = Ledger.signRequest(this.wallet, did, msg).get();

		assertTrue(signedMessage.contains(expectedSignature));
	}

	@Test
	public void testSignWorksForUnknowDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		String msg = "{\"reqId\":1496822211362017764}";
		Ledger.signRequest(this.wallet, DID, msg).get();
	}

	@Test
	public void testSignWorksForInvalidMessageFormat() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
		String did = result.getDid();

		String msg = "\"reqId\":1496822211362017764";
		Ledger.signRequest(this.wallet, did, msg).get();
	}

}
