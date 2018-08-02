package com.evernym.sdk.vcx.token;

import android.util.Log;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class TokenApi extends VcxJava.API {

    private TokenApi(){}
    private static String TAG = "JAVA_WRAPPER::API_CONNECTION";

    private static Callback vcxTokenCB = new Callback() {
        public void callback(int command_handle, int err, String tokenInfo){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;

            future.complete(tokenInfo);
        }
    };

    public static CompletableFuture<String> getTokenInfo(
            int paymentHandle
    ) throws VcxException {
        Log.d(TAG, "getTokenInfo, called with paymentHandle=[" + paymentHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_wallet_get_token_info(commandHandle, paymentHandle, vcxTokenCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxSendTokensCB = new Callback() {
        public void callback(int commandHandle, int error, String receipt) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, error)) {
                return;
            }
            future.complete(receipt);
        }
    };

    public static CompletableFuture<String> sendTokens(
            int paymentHandle,
            long tokens,
            String recipient
    ) throws VcxException {
        Log.d(TAG, "sendTokens, called with paymentHandle=["+paymentHandle+"] tokens=["+tokens+"]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_wallet_send_tokens(commandHandle, paymentHandle, tokens, recipient, vcxSendTokensCB);
        checkResult(result);
        return future;
    }


    private static Callback vcxCreatePaymentAddressCB = new Callback() {
        public void callback(int commandHandle, int error, String address) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, error)) {
                return;
            }
            future.complete(address);
        }
    };

    public static CompletableFuture<String> createPaymentAddress(
            String seed
    ) throws VcxException {
        Log.d(TAG, "createPaymentAddress, called with seed=[__]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_wallet_create_payment_address(commandHandle, seed, vcxCreatePaymentAddressCB);
        checkResult(result);
        return future;
    }
}
