package com.evernym.sdk.vcx.vcx;

import android.util.Log;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class VcxApi extends VcxJava.API {
    private static String TAG = "JAVA_WRAPPER::API_VCX";

    private VcxApi() {
    }

    private static Callback vcxIniWithConfigCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            Log.d(TAG, "vcxIniWithConfigCB() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = commandHandle;
            future.complete(result);
        }
    };

    private static Callback vcxInitCB = new Callback() {


        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommandHandle, int err) {
            Log.d(TAG, "callback() called with: xcommandHandle = [" + xcommandHandle + "], err = [" + err + "]");
            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommandHandle);
            if (!checkCallback(future, err)) return;
            Void result = null;
            future.complete(result);

        }
    };

    public static CompletableFuture<Integer> vcxInitWithConfig(String configJson) throws VcxException {
        Log.d(TAG, "vcxInitWithConfig() called with: configJson = [" + configJson + "]");
        ParamGuard.notNullOrWhiteSpace(configJson, "config");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_init_with_config(
                commandHandle,
                configJson,
                vcxIniWithConfigCB);
        checkResult(result);

        return future;

    }

    public static CompletableFuture<Integer> vcxInit(String configPath) throws VcxException {
        Log.d(TAG, "vcxInit() called with: configPath = [" + configPath + "]");
        ParamGuard.notNullOrWhiteSpace(configPath, "configPath");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_init(
                commandHandle, configPath,
                vcxInitCB);
        checkResult(result);
        return future;
    }

    public static int vcxShutdown(Boolean deleteWallet) throws VcxException {


        int result = LibVcx.api.vcx_shutdown(deleteWallet);
        checkResult(result);
        return result;
    }

    public static String vcxErrorCMessage(int errorCode) {
        Log.d(TAG, "vcxErrorCMessage() called with: errorCode = [" + errorCode + "]");
        return LibVcx.api.vcx_error_c_message(errorCode);


    }

}
