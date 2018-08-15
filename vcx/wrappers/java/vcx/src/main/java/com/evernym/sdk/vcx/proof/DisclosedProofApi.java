package com.evernym.sdk.vcx.proof;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class DisclosedProofApi extends VcxJava.API {

    private DisclosedProofApi (){}

    private static Callback vcxProofCreateWithMsgIdCB = new Callback() {
        public void callback(int command_handle, int err, int proof_handle, String proof_request){
            CompletableFuture<CreateProofMsgIdResult> future = (CompletableFuture<CreateProofMsgIdResult>) removeFuture(command_handle);
            if(!checkCallback(future, err)) return;
            CreateProofMsgIdResult result = new CreateProofMsgIdResult(proof_handle, proof_request);
            future.complete(result);
        }
    };

    public static CompletableFuture<CreateProofMsgIdResult> proofCreateWithMsgId(
            String sourceId,
            int connectionHandle,
            String msgId
    ) throws VcxException {
        ParamGuard.notNull(sourceId, "sourceId");
        ParamGuard.notNull(msgId, "msgId");
        CompletableFuture<CreateProofMsgIdResult> future = new CompletableFuture<CreateProofMsgIdResult>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_create_with_msgid(commandHandle, sourceId, connectionHandle, msgId, vcxProofCreateWithMsgIdCB);
        checkResult(result);

        return future;
    }


    private static Callback vcxProofRetrieveCredentialsCB = new Callback() {
        public void callback(int command_handle, int err, String matching_credentials){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if(!checkCallback(future, err)) return;
            String result = matching_credentials;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> proofRetrieveCredentials(
            int proofHandle
    ) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_retrieve_credentials(commandHandle, proofHandle, vcxProofRetrieveCredentialsCB);
        checkResult(result);

        return future;
    }


    private static Callback vcxProofGenerateCB = new Callback() {
        public void callback(int command_handle, int err) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future, err)) return;
            // resolving with no error
            Integer result = 0;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofGenerate(
            int proofHandle,
            String selectedCredentials,
            String selfAttestedAttributes
    ) throws VcxException {
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_generate_proof(commandHandle, proofHandle, selectedCredentials, selfAttestedAttributes, vcxProofGenerateCB);
        checkResult(result);

        return future;
    }


    private static Callback vcxProofSendCB = new Callback() {
        public void callback(int command_handle, int err) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future, err)) return;
            // resolving with no error
            Integer result = 0;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofSend(
            int proofHandle,
            int connectionHandle
    ) throws VcxException {
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_send_proof(commandHandle, proofHandle, connectionHandle, vcxProofSendCB);
        checkResult(result);

        return future;
    }


    private static Callback vcxProofCreateWithRequestCB = new Callback() {
        public void callback(int command_handle, int err, int proofHandle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future, err)) return;
            // resolving with no error
            Integer result = proofHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofCreateWithRequest(
            String sourceId,
            String proofRequest
    ) throws VcxException {
        ParamGuard.notNull(sourceId, "sourceId");
        ParamGuard.notNull(proofRequest, "proofRequest");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_create_with_request(commandHandle, sourceId, proofRequest, vcxProofCreateWithRequestCB);
        checkResult(result);

        return future;
    }


    private static Callback vcxProofSerializeCB = new Callback() {
        public void callback(int command_handle, int err, String serializedProof) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if(!checkCallback(future, err)) return;

            future.complete(serializedProof);
        }
    };

    public static CompletableFuture<String> proofSerialize(
            int proofHandle
    ) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_serialize(commandHandle, proofHandle, vcxProofSerializeCB);
        checkResult(result);

        return future;
    }


    private static Callback vcxProofDeserializeCB = new Callback() {
        public void callback(int command_handle, int err, int proofHandle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future, err)) return;

            future.complete(proofHandle);
        }
    };

    public static CompletableFuture<Integer> proofDeserialize(
            String serializedProof
    ) throws VcxException {
        ParamGuard.notNull(serializedProof, "serializedProof");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_deserialize(commandHandle, serializedProof, vcxProofDeserializeCB);
        checkResult(result);

        return future;
    }

}
