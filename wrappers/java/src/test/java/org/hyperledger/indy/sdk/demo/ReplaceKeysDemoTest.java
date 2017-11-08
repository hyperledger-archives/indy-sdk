package org.hyperledger.indy.sdk.demo;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.ledger.InvalidLedgerTransactionException;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;


public class ReplaceKeysDemoTest extends IndyIntegrationTestWithPoolAndSingleWallet {

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
		String newVerkey = Signus.replaceKeysStart(wallet, myDid, "{}").get();

		// 5. Build and send Nym Request with new key
		nymRequest = Ledger.buildNymRequest(myDid, myDid, newVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, nymRequest).get();

		// 6. Apply replacing of keys
		Signus.replaceKeysApply(wallet, myDid).get();

		// 7. Send schema request
		String schemaRequest = Ledger.buildSchemaRequest(myDid, SCHEMA_DATA).get();
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
		thrown.expectCause(isA(InvalidLedgerTransactionException.class));

		// 6. Send schema request
		String schemaRequest = Ledger.buildSchemaRequest(myDid, SCHEMA_DATA).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, schemaRequest).get();
	}
}
