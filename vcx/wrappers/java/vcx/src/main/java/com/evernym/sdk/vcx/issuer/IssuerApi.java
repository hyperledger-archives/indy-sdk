package com.evernym.sdk.vcx.issuer;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class IssuerApi extends VcxJava.API {

    static String TAG = "JAVA_WRAPPER:IssuerApi ";

    private static final Callback issuerCreateCredentialCB = new Callback() {
        // TODO: This callback and jna definition needs to be fixed for this API
        // it should accept connection handle as well
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int credntialHandle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = credntialHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> issuerCreateCredential(String sourceId,
                                                                    String credentialDefId,
                                                                    String issuerId,
                                                                    String credentialData,
                                                                    String credentialName,
                                                                    long price) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        ParamGuard.notNullOrWhiteSpace(sourceId, "credentialDefId");
        ParamGuard.notNullOrWhiteSpace(sourceId, "SchemaId");
        //TODO: Check for more mandatory params in vcx to add in PamaGuard
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_create_credential(
                issue,
                sourceId,
                credentialDefId,
                issuerId,
                credentialData,
                credentialName,
                price,
                issuerCreateCredentialCB);
        checkResult(result);
        return future;
    }

    private static Callback issuerSendcredentialOfferCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            future.complete(err);
        }
    };

    public static CompletableFuture<Integer> issuerSendcredentialOffer(int credentialOffer,
                                                                       int connectionHandle) throws VcxException {
        ParamGuard.notNull(credentialOffer, "credentialOffer");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_send_credential_offer(
                issue,
                credentialOffer,
                connectionHandle,
                issuerSendcredentialOfferCB
        );
        checkResult(result);
        return future;
    }

    private static Callback issuerCredntialUpdateStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, LibVcx.State state) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(state.ordinal());
        }
    };

    public static CompletableFuture<Integer> issuerCredntialUpdateState(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_connection_get_state(issue, credentialHandle, issuerCredntialUpdateStateCB);
        checkResult(result);
        return future;
    }

    private static Callback issuerCredntialGetStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, LibVcx.State state) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(state.ordinal());
        }
    };

    public static CompletableFuture<Integer> issuerCredntialGetState(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_connection_get_state(issue, credentialHandle, issuerCredntialGetStateCB);
        checkResult(result);
        return future;
    }
    private static Callback issuerSendCredentialCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String credentialDefId) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(credentialDefId);
        }
    };

    public static CompletableFuture<String> issuerSendCredential(int credentialHandle,
                                                                 int connectionHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        CompletableFuture<String> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_send_credential(
                issue,
                credentialHandle,
                connectionHandle,
                issuerSendCredentialCB);

        checkResult(result);
        return future;
    }

    private static Callback issuerCredentialSerializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String serializedData) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            String result = serializedData;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> issuerCredentialSerialize(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        CompletableFuture<String> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_credential_serialize(
                issue,
                credentialHandle,
                issuerCredentialSerializeCB
        );
        checkResult(result);
        return future;
    }

    private static Callback issuerCredentialDeserializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int handle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            Integer result = handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> issuerCredentialDeserialize(String serializedData) throws VcxException {
        ParamGuard.notNull(serializedData, "serializedData");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_credential_deserialize(
                issue,
                serializedData,
                issuerCredentialDeserializeCB
        );
        checkResult(result);
        return future;
    }



    public static CompletableFuture<Integer> issuerTerminateCredential(
            int credentialHandle,
            int state,
            String msg
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(state, "state");
        ParamGuard.notNullOrWhiteSpace(msg, "msg");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int issue = addFuture(future);

        int result = LibVcx.api.vcx_issuer_terminate_credential(
                issue,
                credentialHandle,
                state,
                msg);
        checkResult(result);

        return future;

    }
    public static CompletableFuture<Integer> issuerCredntialRelease(
            int credentialHandle
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<>();

        int result = LibVcx.api.vcx_issuer_credential_release(credentialHandle);
        checkResult(result);

        return future;
    }

    public static CompletableFuture<Integer> issuercredentialRequest(
            int credentialHandle,
            String credentialRequest) throws VcxException {

        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(credentialRequest, "credentialRequest");
        CompletableFuture<Integer> future = new CompletableFuture<>();

        int result = LibVcx.api.vcx_issuer_get_credential_request(
                credentialHandle,
                credentialRequest);
        checkResult(result);

        return future;
    }

    public static CompletableFuture<Integer> issuerAcceptRequest(
            int credentialHandle) throws VcxException {

        ParamGuard.notNull(credentialHandle, "credentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<>();

        int result = LibVcx.api.vcx_issuer_accept_credential(
                credentialHandle);
        checkResult(result);

        return future;
    }
}
