package com.evernym.sdk.vcx.connection;

/**
 * Created by abdussami on 05/06/18.
 */


import android.util.Log;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

/**
 * Created by abdussami on 03/06/18.
 */

public class ConnectionApi extends VcxJava.API{
    // TODO: We should assign explicit numbers to each state
    public enum State
    {
        None,
        initialized,
        offer_sent,
        request_received,
        accepted,
        unfulfilled,
        expired,
        revoked,
    }
    private static String TAG = "JAVA_WRAPPER::API_CONNECTION";

    private static Callback vcxConnectionCreateCB = new Callback() {
        // TODO: This callback and jna definition needs to be fixed for this API
        // it should accept connection handle as well
        public void callback(int command_handle, int err){
            Log.d(TAG, "callback() called with: command_handle = [" + command_handle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = command_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxConnectionCreate(String SourceId) throws VcxException {
        Log.d(TAG, "vcxConnectionCreate() called with: SourceId = [" + SourceId + "]");
        ParamGuard.notNullOrWhiteSpace(SourceId, "SourceId");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_create(
                commandHandle,
                SourceId,
                vcxConnectionCreateCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxUpdateStateCB = new Callback() {
        public void callback(int command_handle, int err, State s){
            Log.d(TAG, "callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], s = [" + s + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = s.ordinal();
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxConnectionUpdateState(int connectionHandle) throws VcxException {
        Log.d(TAG, "vcxConnectionUpdateState() called with: connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_update_state(
                commandHandle,
                connectionHandle,
                vcxUpdateStateCB
        );
        checkResult(result);
        return future;
    }


    private static Callback vcxCreateConnectionWithInviteCB = new Callback() {
        public void callback(int command_handle, int err, int connectionHandle){
            Log.d(TAG, "vcxCreateConnectionWithInviteCB() called with: command_handle = [" + command_handle + "], err = [" + err + "], connectionHandle = [" + connectionHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            Integer result = connectionHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxCreateConnectionWithInvite(String invitationId, String inviteDetails) throws VcxException {
        Log.d(TAG, "vcxCreateConnectionWithInvite() called with: invitationId = [" + invitationId + "], inviteDetails = [" + inviteDetails + "]");
        ParamGuard.notNullOrWhiteSpace(invitationId, "invitationId");
        ParamGuard.notNullOrWhiteSpace(inviteDetails, "inviteDetails");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_create_with_invite(
                commandHandle,
                invitationId,
                inviteDetails,
                vcxCreateConnectionWithInviteCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxAcceptInvitationCB = new Callback() {
        public void callback(int command_handle, int err, String inviteDetails){
            Log.d(TAG, "vcxAcceptInvitationCB() called with: command_handle = [" + command_handle + "], err = [" + err + "], inviteDetails = [" + inviteDetails + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            String result = inviteDetails;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> vcxAcceptInvitation(int connectionHandle, String connectionType) throws VcxException {
        Log.d(TAG, "vcxAcceptInvitation() called with: connectionHandle = [" + connectionHandle + "], connectionType = [" + connectionType + "]");
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        ParamGuard.notNullOrWhiteSpace(connectionType, "connectionType");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_connect(
                commandHandle,
                connectionHandle,
                connectionType,
                vcxAcceptInvitationCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxConnectionSerializeCB = new Callback() {
        public void callback(int command_handle, int err, String serialized_data){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            String result = serialized_data;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> connectionSerialize(int connectionHandle) throws VcxException {
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_serialize(
                commandHandle,
                connectionHandle,
                vcxConnectionSerializeCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxConnectionDeserializeCB = new Callback() {
        public void callback(int command_handle, int err, int connection_handle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            Integer result = connection_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> connectionDeserialize(String connectionData) throws VcxException {
        ParamGuard.notNull(connectionData, "connectionData");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_deserialize(
                commandHandle,
                connectionData,
                vcxConnectionDeserializeCB
        );
        checkResult(result);
        return future;
    }


    private static Callback vcxConnectionDeleteCB = new Callback() {
        public void callback(int command_handle, int err){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            future.complete(0);
        }
    };

    public static CompletableFuture<Integer> deleteConnection(int connectionHandle) throws VcxException {
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_delete_connection(commandHandle, connectionHandle, vcxConnectionDeleteCB);
        checkResult(result);
        return future;
    }
}
