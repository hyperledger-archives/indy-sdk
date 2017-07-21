package org.hyperledger.indy.sdk.wallet;

import static org.junit.Assert.assertNotNull;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters.CreateAndStoreMyDidJSONParameter;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.junit.Test;


public class RegisterWalletTypeTest extends IndyIntegrationTest {

	@Test
	public void testRegisterWalletTypeWorks() throws Exception {

		Wallet.registerWalletType("inmem", WalletTypeInmem.getInstance());
		
		Wallet.createWallet("default", "registerWalletTypeWorks", "inmem", null, null).get();

		Wallet wallet = Wallet.openWallet("registerWalletTypeWorks", null, null).get();
		assertNotNull(wallet);

		CreateAndStoreMyDidJSONParameter createAndStoreMyDidJSONParameter = new CreateAndStoreMyDidJSONParameter(null, null, null, null);
		CreateAndStoreMyDidResult createAndStoreMyDidResult = Signus.createAndStoreMyDid(wallet, createAndStoreMyDidJSONParameter.toJson()).get();
		String did = createAndStoreMyDidResult.getDid();

/*		String nymRequest = Ledger.buildGetNymRequest(did, did).get();
		String signature = Signus.sign(wallet, did, nymRequest).get();
		Signus.verifySignature(wallet, null, did, signature);*/

		wallet.closeWallet().get();
	}
}
