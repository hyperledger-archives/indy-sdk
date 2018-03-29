import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidResults;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.anoncreds.Anoncreds;

import static org.hyperledger.indy.sdk.anoncreds.Anoncreds.*;
import static org.hyperledger.indy.sdk.ledger.Ledger.*;

public class IssueCredential {
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
		System.out.println("Steward did: " + defautStewardDid);

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
		System.out.println("\n12. Creating Prover wallet and opening it to get the handle\n");
		String proverDID = "VsKV7grR1BUE29mG2Fm2kX";
		String proverWalletName = "prover_wallet";
		Wallet.createWallet(poolName, proverWalletName, null, null, null);
		Wallet proverWalletHandle = Wallet.openWallet(proverWalletName, null, null).get();

		// 13
		System.out.println("\n13. Prover is creating Master Secret\n");
		String masterSecretName = "master_secret";
		Anoncreds.proverCreateMasterSecret(proverWalletHandle, masterSecretName).get();

		// 14
		System.out.println("\n14. Issuer (Trust Anchor) is creating a Claim Offer for Prover\n");
		String claimOfferJSON = issuerCreateClaimOffer(walletHandle, schemaJSON, trustAnchorDID, proverDID).get();
		System.out.println("Claim Offer:\n" + claimOfferJSON);

		// 15
		System.out.println("\n15. Prover creates Claim Request\n");
		String claimRequestJSON = proverCreateAndStoreClaimReq(proverWalletHandle, proverDID, claimOfferJSON,
				claimDef, masterSecretName).get();
		System.out.println("Claim Request:\n" + claimRequestJSON);

		// 1
		System.out.println("\n16. Issuer (Trust Anchor) creates Claim for Claim Request\n");
		String claimAttributesJson = "{\n" +
		"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
		"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
		"               \"height\":[\"175\",\"175\"],\n" +
		"               \"age\":[\"28\",\"28\"]\n" +
		"        }";
		AnoncredsResults.IssuerCreateClaimResult createClaimResult = issuerCreateClaim(walletHandle, claimRequestJSON,
				claimAttributesJson, - 1).get();
		String claimJSON = createClaimResult.getClaimJson();
		System.out.println("Claim:\n" + claimJSON);

		// 17
		System.out.println("\n17. Prover processes and stores Claim\n");
		Anoncreds.proverStoreClaim(proverWalletHandle, claimJSON, null).get();

		// 18
		System.out.println("\n18. Close and delete wallet\n");
		walletHandle.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();

		// 19
		System.out.println("\n19. Close pool\n");
		pool.closePoolLedger().get();

		// 20
		System.out.println("\n20. Delete pool ledger config\n");
		Pool.deletePoolLedgerConfig(poolName).get();
	}
}
