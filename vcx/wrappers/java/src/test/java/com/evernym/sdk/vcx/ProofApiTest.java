package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.connection.ConnectionApi;
import com.evernym.sdk.vcx.connection.InvalidConnectionHandleException;
import com.evernym.sdk.vcx.proof.InvalidProofHandleException;
import com.evernym.sdk.vcx.proof.ProofApi;
import com.evernym.sdk.vcx.vcx.VcxApi;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

public class ProofApiTest {
    private String sourceId = "123";
    private String name = "proof name";
    private String phoneNumber = "8019119191";

    private String attr = "[{'name':'attr1'},{'name':'attr2'}]";

    @BeforeEach
    void setup() throws Exception {
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "DEBUG");
        if (!TestHelper.vcxInitialized) {
            TestHelper.getResultFromFuture(VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE));
            TestHelper.vcxInitialized = true;
        }
    }

    @Test
    @DisplayName("create a proof")
    void createProof() throws VcxException, ExecutionException, InterruptedException {
        int result = TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), "", "{}", name));
        assert (result != 0);
    }

    @Test
    @DisplayName("throw illegal argument exception if invalid arguments are provided")
    void throwIllegalArgumentxException() {
        Assertions.assertThrows(IllegalArgumentException.class, () -> {
            TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), null, "{}", name));
        });
    }

    @Test
    @DisplayName("throw illegal argument exception if no or null arguments are provided")
    void throwIllegalArgumentxException1() {
        Assertions.assertThrows(IllegalArgumentException.class, () -> {
            TestHelper.getResultFromFuture(ProofApi.proofCreate(null, null, null, null, null));
        });
    }

    @Test
    @DisplayName("serialise a proof")
    void serialiseProof() throws VcxException, ExecutionException, InterruptedException {
        int proofHandle = TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), "", "{}", name));
        assert (proofHandle != 0);
        String serialisedProof = TestHelper.getResultFromFuture(ProofApi.proofSerialize(proofHandle));
        assert (serialisedProof.contains(sourceId));

    }

    @Test
    @DisplayName("serialise a bad proof")
    void serialiseBadProof() {
        Assertions.assertThrows(InvalidProofHandleException.class, () -> {
            TestHelper.getResultFromFuture(ProofApi.proofSerialize(0));
        });

    }

    @Test
    @DisplayName("deserialise a proof")
    void deserialiseProof() throws VcxException, ExecutionException, InterruptedException {
        int proofHandle = TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), "", "{}", name));
        assert (proofHandle != 0);
        String serialisedProof = TestHelper.getResultFromFuture(ProofApi.proofSerialize(proofHandle));
        assert (serialisedProof.contains(sourceId));
        int handle = TestHelper.getResultFromFuture(ProofApi.proofDeserialize(serialisedProof));
        assert (handle != 0);
    }

    @Test
    @DisplayName("deserialise a bad proof")
    void deserialiseBadProof() {
        Assertions.assertThrows(ExecutionException.class, () -> {
            TestHelper.getResultFromFuture(ProofApi.proofDeserialize("bad proof"));
        });
    }

    @Test
    @DisplayName("release a proof")
    void releaseProof() throws VcxException, ExecutionException, InterruptedException {
        int proofHandle = TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), "", "{}", name));
        assert (proofHandle != 0);
        ProofApi.proofRelease(proofHandle);
        Assertions.assertThrows(InvalidProofHandleException.class, () -> {
            TestHelper.getResultFromFuture(ProofApi.proofSerialize(proofHandle));
        });
    }

    @Test
    @DisplayName("update state of proof")
    void updateState() throws VcxException, ExecutionException, InterruptedException {
        int proofHandle = TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), "", "{}", name));
        assert (proofHandle != 0);
        int result = TestHelper.getResultFromFuture(ProofApi.proofUpdateState(proofHandle));
        assert(result==1);

    }

    @Test
    @DisplayName("update state of invalid proof")
    void updateStateOfInvalidProof() {
        Assertions.assertThrows(InvalidProofHandleException.class, () -> {
           TestHelper.getResultFromFuture(ProofApi.proofUpdateState(0));
        });

    }

    @Test
    @DisplayName("get state of proof")
    void getState() throws VcxException, ExecutionException, InterruptedException {
        int proofHandle = TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), "", "{}", name));
        assert (proofHandle != 0);
        int result = TestHelper.getResultFromFuture(ProofApi.proofGetState(proofHandle));
        assert(result==1);

    }

    @Test
    @DisplayName("request proof")
    void requestProof() throws VcxException, ExecutionException, InterruptedException {
        int connectionHandle = TestHelper.getResultFromFuture(ConnectionApi.vcxConnectionCreate(sourceId));
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        TestHelper.getResultFromFuture(ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload)));
        int proofHandle = TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), "", "{}", name));
        assert (proofHandle != 0);
        TestHelper.getResultFromFuture(ProofApi.proofSendRequest(proofHandle,connectionHandle));
        int result = TestHelper.getResultFromFuture(ProofApi.proofGetState(proofHandle));
        assert(result==2);

    }

    @Test
    @DisplayName("get request msg")
    void getRequestMsg() throws VcxException, ExecutionException, InterruptedException {
        int proofHandle = TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), "", "{}", name));
        assert (proofHandle != 0);
        String result = TestHelper.getResultFromFuture(ProofApi.proofGetRequestMsg(proofHandle));
        assert(result.length() > 0);
    }

    @Test
    @DisplayName("request proof to invalid connection handle")
    void requestProofToInvalidConnection() {
        Assertions.assertThrows(InvalidConnectionHandleException.class, () -> {
            int proofHandle = TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), "", "{}", name));
            assert (proofHandle != 0);
            TestHelper.getResultFromFuture(ProofApi.proofSendRequest(proofHandle,0));
        });


    }

//    @Test
//    @DisplayName("get proof")
//    void getProof() throws VcxException, ExecutionException, InterruptedException {
//        int connectionHandle = TestHelper.getResultFromFuture(ConnectionApi.vcxConnectionCreate(sourceId));
//        String payload= "{ 'connection_type': 'SMS', 'phone':'8019119191' }";
//        String inviteDetials = TestHelper.getResultFromFuture(ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload)));
//
//        int proofHandle = TestHelper.getResultFromFuture(ProofApi.proofCreate(sourceId, TestHelper.convertToValidJson(attr), "", "{}", name));
//        assert (proofHandle != 0);
//        String serialisedProof = TestHelper.getResultFromFuture(ProofApi.proofSerialize(proofHandle));
//        DocumentContext json = JsonPath.parse(serialisedProof);
//        json.set("data.proof",JsonPath.parse(TestHelper.convertToValidJson("{'version': '1.0', 'to_did': null, 'from_did': null, 'proof_request_id': null, 'libindy_proof': {\"proof_data\":'123'}}")).jsonString());
//        json.set("data.state",4);
//        json.set("data.proof_state",2);
//        System.out.println("::>>" + JsonPath.parse(TestHelper.convertToValidJson("{'version': '1.0', 'to_did': '', 'from_did': '', 'proof_request_id': '', 'libindy_proof': {\"proof_data\":'123'}}")).jsonString());
//        int proof2 = TestHelper.getResultFromFuture(ProofApi.proofDeserialize(json.jsonString()));
//        int result = TestHelper.getResultFromFuture(ProofApi.proofUpdateState(proof2));
//        assert(result==2);
//        GetProofResult proof = TestHelper.getResultFromFuture(ProofApi.getProof(proofHandle,connectionHandle));
//
//
//    }

}
