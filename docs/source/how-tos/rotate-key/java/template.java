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


		// Step 2 code goes here.

		// Step 3 code goes here.
	}
}
