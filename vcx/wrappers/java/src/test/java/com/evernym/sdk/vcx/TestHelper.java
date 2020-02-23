package com.evernym.sdk.vcx;


import com.evernym.sdk.vcx.connection.ConnectionApi;
import com.evernym.sdk.vcx.credential.CredentialApi;
import com.evernym.sdk.vcx.credentialDef.CredentialDefApi;
import com.jayway.jsonpath.JsonPath;
import java.util.concurrent.CompletableFuture;
import org.awaitility.Awaitility;

import java.util.concurrent.ExecutionException;

class TestHelper {
    static boolean vcxInitialized = false;
    static String VCX_CONFIG_TEST_MODE = "ENABLE_TEST_MODE";
    private static String getConnectionId(){
        return "testConnectionId";
    }

    static String convertToValidJson(String InvalidJson){
        String validJson = InvalidJson.replace("'","\"");
        return validJson;
    }

    static final String address1InOffer = "101 Tela Lane";
    private static String offer = "[{\n" +
            "        'msg_type': 'CLAIM_OFFER',\n" +
            "                'version': '0.1',\n" +
            "                'to_did': '8XFh8yBzrpJQmNyZzgoTqB',\n" +
            "                'from_did': '8XFh8yBzrpJQmNyZzgoTqB',\n" +
            "                'libindy_offer': '{}',\n" +
            "                'credential_attrs': {\n" +
            "            'address1': [\n" +
            "            '"+ address1InOffer + "'\n" +
            "      ],\n" +
            "            'address2': [\n" +
            "            '101 Wilson Lane'\n" +
            "      ],\n" +
            "            'city': [\n" +
            "            'SLC'\n" +
            "      ],\n" +
            "            'state': [\n" +
            "            'UT'\n" +
            "      ],\n" +
            "            'zip': [\n" +
            "            '87121'\n" +
            "      ]\n" +
            "        },\n" +
            "        'schema_seq_no': 1487,\n" +
            "                'cred_def_id': 'id1',\n" +
            "                'claim_name': 'Credential',\n" +
            "                'claim_id': 'defaultCredentialId',\n" +
            "                'msg_ref_id': '',\n" +
            "    }]";

    static int _createConnection() throws VcxException {
        CompletableFuture<Integer> futureResult = ConnectionApi.vcxConnectionCreate(TestHelper.getConnectionId());
        Awaitility.await().until(futureResult::isDone);

        Integer result = futureResult.getNow(-1);
        if(result == -1){
            throw new VcxException("Unable to create connection handle",0);
        }else{
//            System.out.println("Connection created with connection handle => "  + result);

            return result;
        }
    }

    static int _createConnectionWithInvite(String inviteDetails) throws VcxException {
        CompletableFuture<Integer> futureResult = ConnectionApi.vcxCreateConnectionWithInvite(TestHelper.getConnectionId(), inviteDetails);
        Awaitility.await().until(futureResult::isDone);

        Integer result = futureResult.getNow(-1);
        if(result == -1){
            throw new VcxException("Unable to create connection handle",0);
        }else{
//            System.out.println("Connection created with connection handle => "  + result);
            return result;
        }
    }

    static int _createCredential() throws VcxException, ExecutionException, InterruptedException {
        CompletableFuture<Integer> futureResult = CredentialApi.credentialCreateWithOffer("1",JsonPath.read(offer,"$").toString());
        Awaitility.await().until(futureResult::isDone);
        return futureResult.get();
    }
    static int _createCredentialDef() throws VcxException, ExecutionException, InterruptedException {
        return  getResultFromFuture(CredentialDefApi.credentialDefCreate(
                "testCredentialDefSourceId",
                "testCredentialDefName",
                "testCredentialDefSchemaId",
                null,
                "tag1",
                "{\"support_revocation\":false, \"tails_file\": \"/tmp/tailsfile.txt\", \"max_creds\": 1}",
                0
                ));
    }

    static  <T> T getResultFromFuture(CompletableFuture<T> future) throws ExecutionException, InterruptedException {
        Awaitility.await().until(future::isDone);
        return future.get();
    }
}
