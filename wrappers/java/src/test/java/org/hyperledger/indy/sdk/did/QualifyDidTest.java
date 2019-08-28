package org.hyperledger.indy.sdk.did;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.wallet.WalletItemNotFoundException;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class QualifyDidTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void qualifyDid() throws Exception {
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, "{}").get();
		String did = result.getDid();
		String prefix = "did:peer";

		String fullQualifiedDid = Did.qualifyDid(wallet, did, prefix).get();
		String expectedDid = prefix + ":" + did;
		assertEquals(expectedDid, fullQualifiedDid);
	}
}