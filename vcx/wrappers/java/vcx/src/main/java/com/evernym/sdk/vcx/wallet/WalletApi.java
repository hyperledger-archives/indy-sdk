package com.evernym.sdk.vcx.wallet;

import android.util.Log;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class WalletApi extends VcxJava.API {
    private static String TAG = "JAVA_WRAPPER::API_VCX::WALLET";

    private WalletApi(){}

    private static Callback vcxExportWalletCB = new Callback() {
        public void callback(int command_handle, int err, int export_handle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = export_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> exportWallet(
            String exportPath,
            String encryptionKey
    ) throws VcxException {
        ParamGuard.notNull(exportPath, "exportPath");
        ParamGuard.notNull(encryptionKey, "encryptionKey");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_export(commandHandle, exportPath, encryptionKey, vcxExportWalletCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxImportWalletCB = new Callback() {
        public void callback(int command_handle, int err, int import_handle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = import_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> importWallet(
            String config
    ) throws VcxException {
        ParamGuard.notNull(config, "config");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_import(commandHandle, config, vcxImportWalletCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxAddRecordWalletCB = new Callback() {
        public void callback(int command_handle, int err){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = command_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> addRecordWallet(
            String recordType,
            String recordId,
            String recordValue
    ) throws VcxException {
        ParamGuard.notNull(recordType, "recordType");
        ParamGuard.notNull(recordId, "recordId");
        ParamGuard.notNull(recordValue, "recordValue");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);
        String recordTag = "{}";

        int result = LibVcx.api.vcx_wallet_add_record( commandHandle, recordType, recordId, recordValue, recordTag, vcxAddRecordWalletCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxDeleteRecordWalletCB = new Callback() {
        public void callback(int command_handle, int err){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = command_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> deleteRecordWallet(
            String recordType,
            String recordId
    ) throws VcxException {
        ParamGuard.notNull(recordType, "recordType");
        ParamGuard.notNull(recordId, "recordId");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_delete_record( commandHandle, recordType, recordId, vcxDeleteRecordWalletCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxGetRecordWalletCB = new Callback() {
        public void callback(int command_handle, int err, String wallet_value){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            String result = wallet_value;
            // if nonzero errorcode, ignore wallet_value (null)
            // if error fail
            // if error = 0 then send the result
            future.complete(result);
        }
    };

    public static CompletableFuture<String> getRecordWallet(
            String recordType,
            String recordId,
            String recordValue
    ) throws VcxException {
        ParamGuard.notNull(recordType, "recordType");
        ParamGuard.notNull(recordId, "recordId");
        ParamGuard.notNull(recordValue, "recordValue");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);
        String recordTag = "{}";

        int result = LibVcx.api.vcx_wallet_get_record( commandHandle, recordType, recordId, recordTag, vcxGetRecordWalletCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxUpdateRecordWalletCB = new Callback() {
        public void callback(int command_handle, int err){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            Integer result = command_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> updateRecordWallet(
            String recordType,
            String recordId,
            String recordValue
    ) throws VcxException {
        ParamGuard.notNull(recordType, "recordType");
        ParamGuard.notNull(recordId, "recordId");
        ParamGuard.notNull(recordValue, "recordValue");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_wallet_update_record_value( commandHandle, recordType, recordId, recordValue, vcxUpdateRecordWalletCB);
        checkResult(result);

        return future;
    }

}