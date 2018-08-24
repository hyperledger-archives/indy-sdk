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
    private static Callback provAsyncCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String config) {
            Log.d(TAG, "provAsyncCB() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], config = [" + config + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
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
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            Log.d(TAG, "vcxUpdateAgentInfoCB() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = commandHandle;
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

    private static Callback vcxGetMessagesCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String messages) {
            Log.d(TAG, "vcxGetMessagesCB() called with commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            String result = messages;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> vcxGetMessages(String messageStatus, String uids, String pwdids) throws VcxException {
        Log.d(TAG, "vcxGetMessage() called with: message_status = [" + messageStatus + "], uids =[" + uids + "], pw_dids = [" + pwdids + "]");
        ParamGuard.notNullOrWhiteSpace(messageStatus, "messageStatus");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_messages_download(
                commandHandle,
                messageStatus,
                uids,
                pwdids,
                vcxGetMessagesCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxUpdateMessagesCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            Log.d(TAG, "vcxUpdateMessageCB() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = commandHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxUpdateMessages(String messageStatus, String msgJson) throws VcxException {
        Log.d(TAG, "vcxUpdateMessages() called with: messageStatus = [" + messageStatus + "], msgJson = [" + msgJson + "]");
        ParamGuard.notNullOrWhiteSpace(messageStatus, "messageStatus");
        ParamGuard.notNull(msgJson, "msgJson");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_messages_update_status(
                commandHandle,
                messageStatus,
                msgJson,
                vcxUpdateMessagesCB
        );
        checkResult(result);
        return future;
    }

    private static Callback getLedgerFeesCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String fees) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            String result = fees;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> getLedgerFees() throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_ledger_get_fees(
                commandHandle,
                getLedgerFeesCB
        );
        checkResult(result);
        return future;
    }

}
