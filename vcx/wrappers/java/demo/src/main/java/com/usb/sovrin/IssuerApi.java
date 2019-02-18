package com.usb.sovrin;

//
// Source code recreated from a .class file by IntelliJ IDEA
// (powered by Fernflower decompiler)
//


import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava.API;
import com.sun.jna.Callback;
import java9.util.concurrent.CompletableFuture;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class IssuerApi extends API {
    private static final Logger logger = LoggerFactory.getLogger("IssuerApi");
    private static Callback issuerCreateCredentialCB = new Callback() {
        public void callback(int commandHandle, int err, int credentialHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], credntialHandle = [" + credentialHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>)removeFuture(commandHandle);
            if (checkCallback(future, err)) {
                Integer result = credentialHandle;
                future.complete(result);
            }
        }
    };
    private static Callback issuerSendcredentialOfferCB = new Callback() {
        public void callback(int commandHandle, int err) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>)removeFuture(commandHandle);
            if (checkCallback(future, err)) {
                future.complete(err);
            }
        }
    };
    private static Callback issuerCredentialUpdateStateCB = new Callback() {
        public void callback(int commandHandle, int err, int state) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future =(CompletableFuture<Integer>) removeFuture(commandHandle);
            if (checkCallback(future, err)) {
                future.complete(state);
            }
        }
    };
    private static Callback issuerCredntialGetStateCB = new Callback() {
        public void callback(int commandHandle, int err, int state) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>)removeFuture(commandHandle);
            if (checkCallback(future, err)) {
                future.complete(state);
            }
        }
    };
    private static Callback issuerSendCredentialCB = new Callback() {
        public void callback(int commandHandle, int err, int credentialDefId) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], credentialDefId = [" + credentialDefId + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>)removeFuture(commandHandle);
            if (checkCallback(future, err)) {
                future.complete(credentialDefId);
            }
        }
    };
    private static Callback issuerCredentialSerializeCB = new Callback() {
        public void callback(int commandHandle, int err, String serializedData) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], serializedData = [" + serializedData + "]");
            CompletableFuture<String> future = (CompletableFuture<String>)removeFuture(commandHandle);
            if (checkCallback(future, err)) {
                future.complete(serializedData);
            }
        }
    };
    private static Callback issuerCredentialDeserializeCB = new Callback() {
        public void callback(int commandHandle, int err, int handle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], handle = [" + handle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>)removeFuture(commandHandle);
            if (checkCallback(future, err)) {
                Integer result = handle;
                future.complete(result);
            }
        }
    };

    public IssuerApi() {
    }

    public static CompletableFuture<Integer> issuerCreateCredential(String sourceId, int credentialDefHandle, String issuerId, String credentialData, String credentialName, long price) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        ParamGuard.notNullOrWhiteSpace(sourceId, "credentialDefId");
        ParamGuard.notNullOrWhiteSpace(sourceId, "SchemaId");
        logger.debug("issuerCreateCredential() called with: sourceId = [" + sourceId + "], credentialDefHandle = [" + credentialDefHandle + "], issuerId = [" + issuerId + "], credentialData = [" + credentialData + "], credentialName = [" + credentialName + "], price = [" + price + "]");
        CompletableFuture<Integer> future = new CompletableFuture();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_issuer_create_credential(issue, sourceId, credentialDefHandle, issuerId, credentialData, credentialName, String.valueOf(price), issuerCreateCredentialCB);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> issuerSendcredentialOffer(int credentialOffer, int connectionHandle) throws VcxException {
        ParamGuard.notNull(credentialOffer, "credentialOffer");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        logger.debug("issuerSendcredentialOffer() called with: credentialOffer = [" + credentialOffer + "], connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_issuer_send_credential_offer(issue, credentialOffer, connectionHandle, issuerSendcredentialOfferCB);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> issuerCredentialUpdateState(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerCredntialUpdateState() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_issuer_credential_update_state(issue, credentialHandle, issuerCredentialUpdateStateCB);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> issuerCredntialGetState(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerCredntialGetState() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_issuer_credential_get_state(issue, credentialHandle, issuerCredntialGetStateCB);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<String> issuerSendCredential(int credentialHandle, int connectionHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        logger.debug("issuerSendCredential() called with: credentialHandle = [" + credentialHandle + "], connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<String> future = new CompletableFuture();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_issuer_send_credential(issue, credentialHandle, connectionHandle, issuerSendCredentialCB);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<String> issuerCredentialSerialize(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerCredentialSerialize() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<String> future = new CompletableFuture();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_issuer_credential_serialize(issue, credentialHandle, issuerCredentialSerializeCB);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> issuerCredentialDeserialize(String serializedData) throws VcxException {
        ParamGuard.notNull(serializedData, "serializedData");
        logger.debug("issuerCredentialDeserialize() called with: serializedData = [" + serializedData + "]");
        CompletableFuture<Integer> future = new CompletableFuture();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_issuer_credential_deserialize(issue, serializedData, issuerCredentialDeserializeCB);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> issuerTerminateCredential(int credentialHandle, int state, String msg) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(state, "state");
        ParamGuard.notNullOrWhiteSpace(msg, "msg");
        logger.debug("issuerTerminateCredential() called with: credentialHandle = [" + credentialHandle + "], state = [" + state + "], msg = [" + msg + "]");
        CompletableFuture<Integer> future = new CompletableFuture();
        int issue = addFuture(future);
        int result = LibVcx.api.vcx_issuer_terminate_credential(issue, credentialHandle, state, msg);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> issuerCredntialRelease(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerCredntialRelease() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture();
        int result = LibVcx.api.vcx_issuer_credential_release(credentialHandle);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> issuercredentialRequest(int credentialHandle, String credentialRequest) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        ParamGuard.notNull(credentialRequest, "credentialRequest");
        logger.debug("issuercredentialRequest() called with: credentialHandle = [" + credentialHandle + "], credentialRequest = [" + credentialRequest + "]");
        CompletableFuture<Integer> future = new CompletableFuture();
        int result = LibVcx.api.vcx_issuer_get_credential_request(credentialHandle, credentialRequest);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> issuerAcceptRequest(int credentialHandle) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        logger.debug("issuerAcceptRequest() called with: credentialHandle = [" + credentialHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture();
        int result = LibVcx.api.vcx_issuer_accept_credential(credentialHandle);
        checkResult(result);
        return future;
    }
}
