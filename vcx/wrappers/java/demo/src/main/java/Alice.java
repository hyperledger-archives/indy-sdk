import com.evernym.sdk.vcx.connection.ConnectionApi;
import com.evernym.sdk.vcx.credential.CredentialApi;
import com.evernym.sdk.vcx.proof.DisclosedProofApi;
import com.evernym.sdk.vcx.utils.UtilsApi;
import com.evernym.sdk.vcx.vcx.VcxApi;

import com.jayway.jsonpath.DocumentContext;
import com.jayway.jsonpath.JsonPath;

import org.apache.commons.cli.CommandLine;

import java.util.LinkedHashMap;
import java.util.List;
import java.util.Scanner;
import java.util.logging.Logger;

import utils.Common;
import static utils.Common.prettyJson;
import static utils.State.StateType;

public class Alice {
    // get logger for demo - INFO configured
    static final Logger logger = Common.getDemoLogger();

    public static void main(String[] args) throws Exception {
        // Library logger setup - ERROR|WARN|INFO|DEBUG|TRACE
        Common.setLibraryLogger("ERROR");

        CommandLine options = Common.getCommandLine(args);
        if (options == null) System.exit(0);

        logger.info("#0 Initialize");
        Common.loadNullPayPlugin();

        // static configuration
        long utime = System.currentTimeMillis() / 1000;
        DocumentContext provisionConfig = JsonPath.parse("{" +
                "  agency_url: 'http://localhost:8080'," +
                "  agency_did: 'VsKV7grR1BUE29mG2Fm2kX'," +
                "  agency_verkey: 'Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR'," +
                "  wallet_name: 'node_vcx_demo_alice_wallet_" + utime + "'," +
                "  wallet_key: '123'," +
                "  payment_method: 'null'," +
                "  enterprise_seed: '000000000000000000000000Trustee1'" +
                "}");

        // Communication method. aries.
        provisionConfig.put("$", "protocol_type", "3.0");
        logger.info("Running with Aries VCX Enabled! Make sure VCX agency is configured to use protocol_type 3.0");

        if (options.hasOption("postgres")) {
            Common.loadPostgresPlugin();
            provisionConfig.put("$", "wallet_type", "postgres_storage")
                    .put("$", "storage_config", "{\"url\":\"localhost:5432\"}")
                    .put("$", "storage_credentials", "{\"account\":\"postgres\",\"password\":\"mysecretpassword\"," +
                            "\"admin_account\":\"postgres\",\"admin_password\":\"mysecretpassword\"}");
            logger.info("Running with PostreSQL wallet enabled! Config = " + provisionConfig.read("$.storage_config"));
        } else {
            logger.info("Running with builtin wallet.");
        }

        logger.info("#8 Provision an agent and wallet, get back configuration details: \n" + prettyJson(provisionConfig.jsonString()));
        DocumentContext vcxConfig = JsonPath.parse(UtilsApi.vcxProvisionAgent(provisionConfig.jsonString()));

        vcxConfig.put("$", "institution_name", "alice")
                .put("$", "institution_logo_url", "http://robohash.org/345")
                .put("$", "protocol_version", "2")
                .put("$", "genesis_path", System.getProperty("user.dir") + "/genesis.txn");
        logger.info("#9 Initialize libvcx with new configuration\n" + prettyJson(vcxConfig.jsonString()));
        VcxApi.vcxInitWithConfig(vcxConfig.jsonString()).get();

        logger.info("Input faber invitation details\nEnter your invite details:");
        Scanner sc = new Scanner(System.in);
        DocumentContext details = JsonPath.parse(sc.nextLine());

        logger.info("#10 Convert to valid json and string and create a connection to faber");
        int connectionHandle = ConnectionApi.vcxCreateConnectionWithInvite("faber", details.jsonString()).get();
        ConnectionApi.vcxConnectionConnect(connectionHandle, "{\"use_public_did\": true}").get();
        ConnectionApi.vcxConnectionUpdateState(connectionHandle).get();
        int connectionState = ConnectionApi.connectionGetState(connectionHandle).get();
        while (connectionState != StateType.Accepted) {
            Thread.sleep(2000);
            ConnectionApi.vcxConnectionUpdateState(connectionHandle).get();
            connectionState = ConnectionApi.connectionGetState(connectionHandle).get();
        }

        logger.info("#11 Wait for faber to issue a credential offer");
        Thread.sleep(5000);
        DocumentContext offers = JsonPath.parse(CredentialApi.credentialGetOffers(connectionHandle).get());
        logger.info("Alice found " + offers.read("$.length()") + " credential offers.");
        DocumentContext credentialOffer = JsonPath.parse((List)offers.read("$.[0]"));
        logger.info("credential offer:\n" + prettyJson(credentialOffer.jsonString()));

        // Create a credential object from the credential offer
        int credentialHandle = CredentialApi.credentialCreateWithOffer("credential", credentialOffer.jsonString()).get();

        logger.info("#15 After receiving credential offer, send credential request");
        CredentialApi.credentialSendRequest(credentialHandle, connectionHandle, 0).get();

        logger.info("#16 Poll agency and accept credential from faber");
        CredentialApi.credentialUpdateState(credentialHandle).get();
        int credentialState = CredentialApi.credentialGetState(credentialHandle).get();
        while (credentialState != StateType.Accepted) {
            Thread.sleep(2000);
            CredentialApi.credentialUpdateState(credentialHandle).get();
            credentialState = CredentialApi.credentialGetState(credentialHandle).get();
        }

        logger.info("#22 Poll agency for a proof request");
        DocumentContext requests = JsonPath.parse(DisclosedProofApi.proofGetRequests(connectionHandle).get());
        while (requests.read("$.length()").equals("0")) {
            Thread.sleep(2000);
            requests = JsonPath.parse(DisclosedProofApi.proofGetRequests(connectionHandle).get());
        }
        logger.info("Alice found " + requests.read("$.length()") + " proof requests.");
        DocumentContext proofRequest = JsonPath.parse((LinkedHashMap)requests.read("$.[0]"));
        logger.info("proof request:\n" + prettyJson(proofRequest.jsonString()));

        logger.info("#23 Create a Disclosed proof object from proof request");
        int proofHandle = DisclosedProofApi.proofCreateWithRequest("proof", proofRequest.jsonString()).get();

        logger.info("#24 Query for credentials in the wallet that satisfy the proof request");
        DocumentContext credentials = JsonPath.parse(DisclosedProofApi.proofRetrieveCredentials(proofHandle).get());

        LinkedHashMap<String, Object> attrs = credentials.read("$.attrs");
        for(String key : attrs.keySet()){
            DocumentContext attr = JsonPath.parse((LinkedHashMap)credentials.read("$.attrs." + key + ".[0]"));
            credentials.set("$.attrs." + key, JsonPath.parse("{\"credential\":"+ attr.jsonString() + "}").json());
        }

        logger.info("#25 Generate the proof");
        DisclosedProofApi.proofGenerate(proofHandle, credentials.jsonString(), "{}").get();

        logger.info("#26 Send the proof to faber");
        DisclosedProofApi.proofSend(proofHandle, connectionHandle).get();

        logger.info("#27 Wait for Faber to receive the proof");
        DisclosedProofApi.proofUpdateState(proofHandle).get();
        int proofState = DisclosedProofApi.proofGetState(proofHandle).get();
        while (proofState != StateType.Accepted) {
            Thread.sleep(2000);
            DisclosedProofApi.proofUpdateState(proofHandle).get();
            proofState = DisclosedProofApi.proofGetState(proofHandle).get();
        }
        logger.info("Faber received the proof");

        System.exit(0);
    }
}
