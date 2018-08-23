package com.evernym.sdk.vcx.connection;

/**
 * Created by abdussami on 05/06/18.
 */


import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

/**
 * Created by abdussami on 03/06/18.
 */

public class ConnectionApi extends VcxJava.API {


    private static String TAG = "JAVA_WRAPPER::API_CONNECTION";

    private static Callback vcxConnectionCreateCB = new Callback() {
        // TODO: This callback and jna definition needs to be fixed for this API
        // it should accept connection handle as well
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int connectionHandle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = connectionHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxConnectionCreate(String sourceId) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_create(
                commandHandle,
                sourceId,
                vcxConnectionCreateCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxUpdateStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, LibVcx.State s) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = s.ordinal();
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxConnectionUpdateState(int connectionHandle) throws VcxException {
        CompletableFuture<Integer> future = new CompletableFuture<>();
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
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int connectionHandle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
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
        ParamGuard.notNullOrWhiteSpace(invitationId, "invitationId");
        ParamGuard.notNullOrWhiteSpace(inviteDetails, "inviteDetails");
        CompletableFuture<Integer> future = new CompletableFuture<>();
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
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String inviteDetails) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
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
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        ParamGuard.notNullOrWhiteSpace(connectionType, "connectionType");
        CompletableFuture<String> future = new CompletableFuture<>();
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
            future.complete(serializedData);
        }
    };

    public static CompletableFuture<String> connectionSerialize(int connectionHandle) throws VcxException {
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        CompletableFuture<String> future = new CompletableFuture<>();
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
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int connectionHandle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            future.complete(connectionHandle);
        }
    };

    public static CompletableFuture<Integer> connectionDeserialize(String connectionData) throws VcxException {
        ParamGuard.notNull(connectionData, "connectionData");
        CompletableFuture<Integer> future = new CompletableFuture<>();
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
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(0);
        }
    };

    public static CompletableFuture<Integer> deleteConnection(int connectionHandle) throws VcxException {
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_delete_connection(commandHandle, connectionHandle, vcxConnectionDeleteCB);
        checkResult(result);
        return future;
    }

    private static Callback vcxConnectionInviteDetailsCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String details) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(details);
        }
    };

    public static CompletableFuture<String> connectionInviteDetails(int connectionHandle, int abbreviated) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_connection_invite_details(commandHandle, connectionHandle, abbreviated, vcxConnectionInviteDetailsCB);
        checkResult(result);
        return future;
    }

    public static int connectionRelease(int connnectionHandle) {
        return LibVcx.api.vcx_connection_release(connnectionHandle);
    }

    private static Callback vcxConnectionGetStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, LibVcx.State state) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(state.ordinal());
        }
    };

    public static CompletableFuture<Integer> connectionGetState(int connnectionHandle) throws VcxException {
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_connection_get_state(commandHandle, connnectionHandle, vcxConnectionGetStateCB);
        checkResult(result);
        return future;
    }
}
