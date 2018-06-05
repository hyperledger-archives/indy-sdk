package com.evernym.sdk.vcx.credential;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class CredentialApi extends VcxJava.API {

    private CredentialApi(){}

    private static Callback vcxCredentialCreateWithMsgidCB = new Callback() {
        public void callback(int command_handle,int err,int credentailHandle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = credentailHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> credentialCreateWithMsgid(
            String sourceId,
            int connectionHandle,
            String msgId
    ) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId,"sourceId");
        ParamGuard.notNullOrWhiteSpace(msgId,"msgId");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_create_with_msgid(
                commandHandle,
                sourceId,
                connectionHandle,
                msgId,
                vcxCredentialCreateWithMsgidCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxCredentialSendRequestCB = new Callback() {
        public void callback(int command_handle,int err,String credentail){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            String result = credentail;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> credentialSendRequest(
            int credentialHandle,
            int connectionHandle,
            int payment_handle
    ) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_send_request(
                commandHandle,
                credentialHandle,
                connectionHandle,
                payment_handle,
                vcxCredentialSendRequestCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxCredentialSerializeCB = new Callback() {
        public void callback(int command_handle,int err,String serializedCredentail){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            String result = serializedCredentail;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> credentialSerialize(
            int credentailHandle
    ) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_serialize(commandHandle,
                credentailHandle,
                vcxCredentialSerializeCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxCredentialDeserializeCB = new Callback() {
        public void callback(int command_handle,int err,int credentailHandle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = credentailHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> credentialDeserialize(
            String serializedCredential
    ) throws VcxException {
        ParamGuard.notNull(serializedCredential,"serializedCredential");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_deserialize(commandHandle,
                serializedCredential,
                vcxCredentialDeserializeCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxGetCredentialCB = new Callback() {
        public void callback(int command_handle, int err,  String credential){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            String result = credential;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> getCredential(
            int credentialHandle
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_get_credential(commandHandle, credentialHandle, vcxGetCredentialCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxCredentialUpdateStateCB = new Callback() {
        public void callback(int command_handle, int err, int state){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = state;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> credentialUpdateState(
            int credentialHandle
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_update_state(commandHandle, credentialHandle, vcxCredentialUpdateStateCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxCredentialGetStateCB = new Callback() {
        public void callback(int command_handle, int err, int state){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = state;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> credentialGetState(
            int credentialHandle
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_get_state(commandHandle, credentialHandle, vcxCredentialGetStateCB);
        checkResult(result);

        return future;
    }

    public static CompletableFuture<Integer> credentialRelease(
            int credentialHandle
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();

        int result = LibVcx.api.vcx_credential_release(credentialHandle);
        checkResult(result);

        return future;
    }

    private static Callback vcxCredentialGetOffersCB = new Callback() {
        public void callback(int command_handle, int err, String credential_offers){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            String result = credential_offers;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> credentialGetOffers(
            int connectionHandle
    ) throws VcxException {
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_get_offers(commandHandle, connectionHandle, vcxCredentialGetOffersCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxCredentialCreateWithOfferCB = new Callback() {
        public void callback(int command_handle, int err, int credential_handle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = credential_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> credentialCreateWithOffer(
            String sourceId,
            String credentialOffer
    ) throws VcxException {
        ParamGuard.notNull(sourceId, "sourceId");
        ParamGuard.notNull(credentialOffer, "credentialOffer");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_create_with_offer(commandHandle, sourceId, credentialOffer, vcxCredentialCreateWithOfferCB);
        checkResult(result);

        return future;
    }


}
