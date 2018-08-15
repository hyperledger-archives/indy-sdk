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

    private VcxApi(){}

    private static Callback vcxIniWithConfigCB = new Callback() {
        public void callback(int command_handle,int err){
            Log.d(TAG, "vcxIniWithConfigCB() called with: command_handle = [" + command_handle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = command_handle;
            future.complete(result);
        }
    };

    private static Callback vcxInitCB = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err) {
            Log.d(TAG, "callback() called with: xcommand_handle = [" + xcommand_handle + "], err = [" + err + "]");
            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
            if (!checkCallback(future, err)) return;
            Void result = null;
            future.complete(result);

        }
    };

    public static CompletableFuture<Integer> vcxInitWithConfig(String config_json) throws VcxException {
        Log.d(TAG, "vcxInitWithConfig() called with: config_json = [" + config_json + "]");
        ParamGuard.notNullOrWhiteSpace(config_json,"config");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_init_with_config(
                commandHandle,
                config_json,
                vcxIniWithConfigCB);
        checkResult(result);

        return future;

    }
    public static CompletableFuture<Integer> vcxInit(String configPath) throws VcxException {
        Log.d(TAG, "vcxInit() called with: configPath = [" + configPath + "]");
        ParamGuard.notNullOrWhiteSpace(configPath,"configPath");
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
