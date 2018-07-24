package com.evernym.sdk.vcx.proof;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class ProofApi extends VcxJava.API {
    private ProofApi(){}

    private static Callback vcxProofCreateCB = new Callback() {
        public void callback(int command_handle, int err, int proof_handle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = proof_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofCreate(
            String sourceId,
            String requestedAttrs,
            String requestedPredicates,
            String name
    ) throws VcxException {
        ParamGuard.notNull(sourceId, "sourceId");
        ParamGuard.notNull(requestedAttrs, "requestedAttrs");
        ParamGuard.notNull(requestedPredicates, "requestedPredicates");
        ParamGuard.notNull(name, "name");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_create(commandHandle, sourceId, requestedAttrs, requestedPredicates, name, vcxProofCreateCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofSendRequestCB = new Callback() {
        public void callback(int command_handle, int err){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = command_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofSendRequest(
            int proofHandle,
            int connectionHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_send_request(commandHandle, proofHandle, connectionHandle, vcxProofSendRequestCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxGetProofCB = new Callback() {
        public void callback(int command_handle, int err, int proof_state, String response_data){
            CompletableFuture<GetProofResult> future = (CompletableFuture<GetProofResult>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            GetProofResult result = new GetProofResult(proof_state,response_data);
            future.complete(result);
        }
    };

    public static CompletableFuture<GetProofResult> getProof(
            int proofHandle,
            int connectionHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        CompletableFuture<GetProofResult> future = new CompletableFuture<GetProofResult>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_get_proof(commandHandle, proofHandle, connectionHandle, vcxGetProofCB);
        checkResult(result);

        return future;
    }

    // vcx_proof_accepted
    public static CompletableFuture<Integer> proofAccepted(
            int proofHandle,
            String responseData
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        ParamGuard.notNull(responseData, "responseData");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();

        int result = LibVcx.api.vcx_proof_accepted(proofHandle, responseData);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofUpdateStateCB = new Callback() {
        public void callback(int command_handle, int err, int state){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = state;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofUpdateState(
            int proofHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_update_state(commandHandle, proofHandle, vcxProofUpdateStateCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofGetStateCB = new Callback() {
        public void callback(int command_handle, int err, int state){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = state;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofGetState(
            int proofHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_get_state(commandHandle, proofHandle, vcxProofGetStateCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofSerializeCB = new Callback() {
        public void callback(int command_handle, int err, String proof_state){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            String result = proof_state;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofSerialize(
            int proofHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_serialize(commandHandle, proofHandle, vcxProofSerializeCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofDeserializeCB = new Callback() {
        public void callback(int command_handle, int err, int proof_handle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = proof_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofDeserialize(
            String serializedProof
    ) throws VcxException {
        ParamGuard.notNull(serializedProof, "serializedProof");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_proof_deserialize(commandHandle, serializedProof, vcxProofDeserializeCB);
        checkResult(result);

        return future;
    }

    public static CompletableFuture<Integer> proofRelease(
            int proofHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();

        int result = LibVcx.api.vcx_proof_release(proofHandle);
        checkResult(result);

        return future;
    }

}