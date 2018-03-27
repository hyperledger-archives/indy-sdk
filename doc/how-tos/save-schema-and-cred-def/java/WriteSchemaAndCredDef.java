import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidResults;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;

import static org.hyperledger.indy.sdk.anoncreds.Anoncreds.issuerCreateAndStoreClaimDef;
import static org.hyperledger.indy.sdk.ledger.Ledger.*;

public class WriteSchemaAndCredDef {
	static void demo() throws Exception {
		String walletName = "myWallet";
		String poolName = "pool";
		String stewardSeed = "000000000000000000000000Steward1";
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
		String defautStewardDid = stewardResult.getDid();
		System.out.println("Steward DID: " + defautStewardDid);
		System.out.println("Steward Verkey: " + stewardResult.getVerkey());

		// 6.
		System.out.println("\n6. Generating and storing Trust Anchor DID and Verkey\n");
		DidResults.CreateAndStoreMyDidResult trustAnchorResult = Did.createAndStoreMyDid(walletHandle, "{}").get();
		String trustAnchorDID = trustAnchorResult.getDid();
		String trustAnchorVerkey = trustAnchorResult.getVerkey();
		System.out.println("Trust anchor DID: " + trustAnchorDID);
		System.out.println("Trust anchor Verkey: " + trustAnchorVerkey);

		// 7
		System.out.println("\n7. Build NYM request to add Trust Anchor to the ledger\n");
		String nymRequest = buildNymRequest(defautStewardDid, trustAnchorDID, trustAnchorVerkey, null, "TRUST_ANCHOR").get();
		System.out.println("NYM request JSON:\n" + nymRequest);

		// 8
		System.out.println("\n8. Sending the nym request to ledger\n");
		String nymResponseJson = signAndSubmitRequest(pool, walletHandle, defautStewardDid, nymRequest).get();
		System.out.println("NYM transaction response:\n" + nymResponseJson);

		// 9
		System.out.println("\n9. Build the SCHEMA request to add new schema to the ledger as a Steward\n");
		String name = "gvt";
		String version = "1.0";
		String attributes = "[\"age\", \"sex\", \"height\", \"name\"]";
		String schemaDataJSON = "{\"name\":\"" + name + "\",\"version\":\"" + version + "\",\"attr_names\":" + attributes + "}";
		System.out.println("Schema: " + schemaDataJSON);
		String schemaRequest = buildSchemaRequest(defautStewardDid, schemaDataJSON).get();
		System.out.println("Schema request:\n" + schemaRequest);

		// 10
		System.out.println("\n10. Sending the SCHEMA request to the ledger\n");
		String schemaResponse = signAndSubmitRequest(pool, walletHandle, defautStewardDid, schemaRequest).get();
		System.out.println("Schema response:\n" + schemaResponse);

		// 11
		System.out.println("\n11. Creating and storing CLAIM DEFINITION using anoncreds as Trust Anchor, for the given Schema\n");
		String schemaJSON = "{\"seqNo\": 1, \"dest\": \"" + defautStewardDid + "\", \"data\": " + schemaDataJSON + "}";
		System.out.println("Schema:\n" + schemaJSON);
		String claimDef = issuerCreateAndStoreClaimDef(walletHandle, trustAnchorDID, schemaJSON, "CL", false).get();
		System.out.println("Claim Definition:\n" + claimDef);

		// 12
		System.out.println("\n12. Close and delete wallet\n");
		walletHandle.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();

		// 13
		System.out.println("\n13. Close pool\n");
		pool.closePoolLedger().get();

		// 14
		System.out.println("\n14. Delete pool ledger config\n");
		Pool.deletePoolLedgerConfig(poolName).get();
	}
}
