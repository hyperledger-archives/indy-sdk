import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidJSONParameters;
import org.hyperledger.indy.sdk.did.DidResults.CreateAndStoreMyDidResult;
import org.json.JSONObject;
import utils.PoolUtils;

import static org.hyperledger.indy.sdk.ledger.Ledger.buildNymRequest;
import static org.hyperledger.indy.sdk.ledger.Ledger.buildSchemaRequest;
import static org.hyperledger.indy.sdk.ledger.Ledger.signAndSubmitRequest;
import static org.hyperledger.indy.sdk.ledger.Ledger.submitRequest;
import static org.hyperledger.indy.sdk.ledger.Ledger.multiSignRequest;
import static org.hyperledger.indy.sdk.ledger.Ledger.appendRequestEndorser;
import static org.hyperledger.indy.sdk.anoncreds.Anoncreds.issuerCreateSchema;
import static org.junit.Assert.assertEquals;

class Endorser {
    static void demo() throws Exception {

        System.out.println("Crypto sample -> started");
        String trusteeSeed = "000000000000000000000000Trustee1";

        // Set protocol version 2 to work with Indy Node 1.4
        Pool.setProtocolVersion(PoolUtils.PROTOCOL_VERSION).get();

        // 1. Create and Open Pool
        String poolName = PoolUtils.createPoolLedgerConfig();
        Pool pool = Pool.openPoolLedger(poolName, "{}").get();

        // 2. Create and Open Author Wallet
        String authorWalletConfig = "{\"id\":\"authorWallet\"}";
        String authorWalletCredentials = "{\"key\":\"author_wallet_key\"}";
        Wallet.createWallet(authorWalletConfig, authorWalletCredentials).get();
        Wallet authorWallet = Wallet.openWallet(authorWalletConfig, authorWalletCredentials).get();

        // 3. Create and Open Endorser Wallet
        String endorserWalletConfig = "{\"id\":\"endorserWallet\"}";
        String endorserWalletCredentials = "{\"key\":\"endorser_wallet_key\"}";
        Wallet.createWallet(endorserWalletConfig, endorserWalletCredentials).get();
        Wallet endorserWallet = Wallet.openWallet(endorserWalletConfig, endorserWalletCredentials).get();

        // 3. Create and Open Trustee Wallet
        String trusteeWalletConfig = "{\"id\":\"trusteeWallet\"}";
        String trusteeWalletCredentials = "{\"key\":\"trustee_wallet_key\"}";
        Wallet.createWallet(trusteeWalletConfig, trusteeWalletCredentials).get();
        Wallet trusteeWallet = Wallet.openWallet(trusteeWalletConfig, trusteeWalletCredentials).get();

        // 4. Create Trustee DID
        DidJSONParameters.CreateAndStoreMyDidJSONParameter theirDidJson =
                new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, trusteeSeed, null, null);
        CreateAndStoreMyDidResult createTheirDidResult = Did.createAndStoreMyDid(trusteeWallet, theirDidJson.toJson()).get();
        String trusteeDid = createTheirDidResult.getDid();

        // 5. Create Author DID
        CreateAndStoreMyDidResult createMyDidResult = Did.createAndStoreMyDid(authorWallet, "{}").get();
        String authorDid = createMyDidResult.getDid();
        String authorVerkey = createMyDidResult.getVerkey();

        // 6. Create Endorser DID
        createMyDidResult = Did.createAndStoreMyDid(endorserWallet, "{}").get();
        String endorserDid = createMyDidResult.getDid();
        String endorserVerkey = createMyDidResult.getVerkey();

        // 7. Build Author Nym Request
        String nymRequest = buildNymRequest(trusteeDid, authorDid, authorVerkey, null, null).get();

        // 8. Trustee Sign Author Nym Request
        String nymResponseJson = signAndSubmitRequest(pool, trusteeWallet, trusteeDid, nymRequest).get();

        // 9. Build Endorser Nym Request
        nymRequest = buildNymRequest(trusteeDid, endorserDid, endorserVerkey, null, "ENDORSER").get();

        // 10. Trustee Sign Endorser Nym Request
        nymResponseJson = signAndSubmitRequest(pool, trusteeWallet, trusteeDid, nymRequest).get();

        // 11. Create schema with endorser

        String schemaName = "gvt";
        String schemaVersion = "1.0";
        String schemaAttributes = "[\"name\", \"age\", \"sex\", \"height\"]";
        AnoncredsResults.IssuerCreateSchemaResult createSchemaResult =
                issuerCreateSchema(authorDid, schemaName, schemaVersion, schemaAttributes).get();
        String schemaId = createSchemaResult.getSchemaId();
        String schemaJson = createSchemaResult.getSchemaJson();

        String schemaRequest = buildSchemaRequest(authorDid, schemaJson).get();
        String schemaRequestWithEndorser = appendRequestEndorser(schemaRequest, endorserDid).get();
        String schemaRequestWithEndorserAuthorSigned =
                multiSignRequest(authorWallet, authorDid, schemaRequestWithEndorser).get();
        String schemaRequestWithEndorserSigned =
                multiSignRequest(endorserWallet, endorserDid, schemaRequestWithEndorserAuthorSigned).get();
        String response = submitRequest(pool, schemaRequestWithEndorserSigned).get();
        JSONObject responseJson = new JSONObject(response);

        assertEquals("REPLY", responseJson.getJSONObject("op"));
    }
}