package com.evernym.sdk.vcx.utils;

import android.util.Log;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;


/**
 * Created by abdussami on 17/05/18.
 */

public class UtilsApi extends VcxJava.API {
    static String TAG = "JAVA_WRAPPER:API_UTILS ";
    public static Callback provAsyncCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, String config) {
            Log.d(TAG, "provAsyncCB() called with: xcommand_handle = [" + xcommand_handle + "], err = [" + err + "], config = [" + config + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
            if (!checkCallback(future, err)) return;

            String result = config;
            future.complete(result);
        }
    };


    public static String vcxProvisionAgent(String config) {
        Log.d(TAG, "vcxProvisionAgent() called with: config = [" + config + "]");
        ParamGuard.notNullOrWhiteSpace(config, "config");
        Log.d(TAG, "vcxProvisionAgent config received: " + config);
        String result = LibVcx.api.vcx_provision_agent(config);
        Log.d(TAG, "vcxProvisionAgent result received: " + result);

        return result;

    }

    public static CompletableFuture<String> vcxAgentProvisionAsync(String conf) throws VcxException {
        Log.d(TAG, "vcxAgentProvisionAsync() called with: conf = [" + conf + "]");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_agent_provision_async(
                commandHandle, conf,
                provAsyncCB);
        checkResult(result);
        return future;
    }

    private static Callback vcxUpdateAgentInfoCB = new Callback() {
        public void callback(int command_handle, int err){
            Log.d(TAG, "vcxUpdateAgentInfoCB() called with: command_handle = [" + command_handle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = command_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxUpdateAgentInfo(String config) throws VcxException {
        Log.d(TAG, "vcxUpdateAgentInfo() called with: config = [" + config + "]");
        ParamGuard.notNullOrWhiteSpace(config, "config");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_agent_update_info(
                commandHandle,
                config,
                vcxUpdateAgentInfoCB
        );
        checkResult(result);
        return future;
    }

}
