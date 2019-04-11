import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidJSONParameters;
import org.hyperledger.indy.sdk.did.DidResults;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import utils.PoolUtils;

import java.util.HashMap;
import java.util.Map;

import static org.hyperledger.indy.sdk.anoncreds.Anoncreds.issuerCreateAndStoreClaimDef;
import static org.hyperledger.indy.sdk.ledger.Ledger.*;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;

public class RotateKey {
	static void demo() throws Exception {
		String walletName = "myWallet";
		String poolName = "pool";
		String stewardSeed = "000000000000000000000000Steward1";
		String seed_trustanchor = "TestTrustAnchor00000000000000000";
		String poolConfig = "{\"genesis_txn\": \"/home/vagrant/code/evernym/indy-sdk/cli/docker_pool_transactions_genesis\"}";


		// 1.
		System.out.println("\n1. Creating a new local pool ledger configuration that can be used later to connect pool nodes.\n");
		Pool.createPoolLedgerConfig(poolName, poolConfig).get();

		// 2
		System.out.println("\n2. Open pool ledger and get the pool handle from libindy.\n");
		Pool pool = Pool.openPoolLedger(poolName, "{}").get();

		// 3
		System.out.println("\n3. Creates a new secure wallet\n");
		Wallet.createWallet(poolName, walletName, "default", null, null).get();

		// 4
		System.out.println("\n4. Open wallet and get the wallet handle from libindy\n");
		Wallet walletHandle = Wallet.openWallet(walletName, null, null).get();

		// 5
		System.out.println("\n5. Generating and storing steward DID and Verkey\n");
		String did_json = "{\"seed\": \"" + stewardSeed + "\"}";
		DidResults.CreateAndStoreMyDidResult stewardResult = Did.createAndStoreMyDid(walletHandle, did_json).get();
		String defaultStewardDid = stewardResult.getDid();
		System.out.println("Steward did: " + defaultStewardDid);

		// 6.
		System.out.println("\n6. Generating and storing Trust Anchor DID and Verkey\n");
		DidResults.CreateAndStoreMyDidResult trustAnchorResult = Did.createAndStoreMyDid(walletHandle, "{}").get();
		String trustAnchorDID = trustAnchorResult.getDid();
		String trustAnchorVerkey = trustAnchorResult.getVerkey();
		System.out.println("Trust anchor DID: " + trustAnchorDID);
		System.out.println("Trust anchor Verkey: " + trustAnchorVerkey);

		// 7
		System.out.println("\n7. Build NYM request to add Trust Anchor to the ledger\n");
		String nymRequest = buildNymRequest(defaultStewardDid, trustAnchorDID, trustAnchorVerkey, null, "TRUST_ANCHOR").get();
		System.out.println("NYM request JSON:\n" + nymRequest);

		// 8
		System.out.println("\n8. Sending NYM request to ledger\n");
		String nymResponseJson = signAndSubmitRequest(pool, walletHandle, defaultStewardDid, nymRequest).get();
		System.out.println("NYM transaction response:\n" + nymResponseJson);

		// 9
		System.out.println("\n9. Generating new Verkey of Trust Anchor in the wallet\n");
		String newTrustAnchorVerkey = Did.replaceKeysStart(walletHandle, trustAnchorDID, "{}").get();
		System.out.println("New Trust Anchor's Verkey: " + newTrustAnchorVerkey);

		// 10
		System.out.println("\n10. Building NYM request to update new verkey to ledger\n");
		String nymUpdateRequest = buildNymRequest(trustAnchorDID, trustAnchorDID, newTrustAnchorVerkey, null, "TRUST_ANCHOR").get();
		System.out.println("NYM request:\n" + nymUpdateRequest);

		// 11
		System.out.println("\n11. Sending NYM request to the ledger\n");
		String nymUpdateResponse = signAndSubmitRequest(pool, walletHandle, trustAnchorDID, nymUpdateRequest).get();
		System.out.println("NYM response:\n" + nymUpdateRequest);

		// 12
		System.out.println("\n12. Applying new Trust Anchor's Verkey in wallet\n");
		Did.replaceKeysApply(walletHandle, trustAnchorDID);

		// 13
		System.out.println("\n13. Reading new Verkey from wallet\n");
		String trustAnchorVerkeyFromWallet = Did.keyForLocalDid(walletHandle, trustAnchorDID).get();

		// 14
		System.out.println("\n14. Building GET_NYM request to get Trust Anchor from Verkey\n");
		String getNymRequest = buildGetNymRequest(trustAnchorDID, trustAnchorDID).get();
		System.out.println("GET_NYM request:\n" + getNymRequest);

		// 15
		System.out.println("\n15. Sending GET_NYM request to ledger\n");
		String getNymResponse = submitRequest(pool, getNymRequest).get();
		System.out.println("GET_NYM response:\n" + getNymResponse);

		// 16
		System.out.println("\n16. Comparing Trust Anchor verkeys\n");
		System.out.println("Written by Steward: " + trustAnchorDID);
		System.out.println("Current from wallet: " + trustAnchorVerkeyFromWallet);
		String responseData = new JSONObject(getNymResponse).getJSONObject("result").getString("data");
		String trustAnchorVerkeyFromLedger = new JSONObject(responseData).getString("verkey");
		System.out.println("Current from ledger: " + trustAnchorVerkeyFromLedger);
		boolean match = !trustAnchorDID.equals(trustAnchorVerkeyFromWallet) && trustAnchorVerkeyFromWallet.equals(trustAnchorVerkeyFromLedger);
		System.out.println("Matching: " + match);

		// 17
		System.out.println("\n17. Close and delete wallet\n");
		walletHandle.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();

		// 18
		System.out.println("\n18. Close pool\n");
		pool.closePoolLedger().get();

		// 19
		System.out.println("\n19. Delete pool ledger config\n");
		Pool.deletePoolLedgerConfig(poolName).get();
	}
}
