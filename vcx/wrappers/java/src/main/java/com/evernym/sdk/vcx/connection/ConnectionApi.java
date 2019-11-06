package com.evernym.sdk.vcx.connection;

/**
 * Created by abdussami on 05/06/18.
 */


import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.concurrent.CompletableFuture;

/**
 * Created by abdussami on 03/06/18.
 */

public class ConnectionApi extends VcxJava.API {

    private static final Logger logger = LoggerFactory.getLogger("ConnectionApi");

    private static Callback vcxConnectionCreateCB = new Callback() {
        // TODO: This callback and jna definition needs to be fixed for this API
        // it should accept connection handle as well
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int connectionHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], connectionHandle = [" + connectionHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = connectionHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxConnectionCreate(String sourceId) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        logger.debug("vcxConnectionCreate() called with: sourceId = [ {} ]",sourceId);
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
        public void callback(int commandHandle, int err, int s) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], s = [" + s + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = s;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxConnectionUpdateState(int connectionHandle) throws VcxException {
        logger.debug("vcxConnectionUpdateState() called with: connectionHandle = [" + connectionHandle + "]");
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

    public static CompletableFuture<Integer> vcxConnectionUpdateStateWithMessage(int connectionHandle, String message) throws VcxException {
        logger.debug("vcxConnectionUpdateState() called with: connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_update_state_with_message(
                commandHandle,
                connectionHandle,
                message,
                vcxUpdateStateCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxCreateConnectionWithInviteCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int connectionHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], connectionHandle = [" + connectionHandle + "]");
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
        logger.debug("vcxCreateConnectionWithInvite() called with: invitationId = [" + invitationId + "], inviteDetails = [" + inviteDetails + "]");
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

    private static Callback vcxConnectionConnectCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String inviteDetails) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], inviteDetails = [" + inviteDetails + "]");
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

    @Deprecated
    public static CompletableFuture<String> vcxAcceptInvitation(int connectionHandle, String connectionType) throws VcxException {
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        ParamGuard.notNullOrWhiteSpace(connectionType, "connectionType");
        return vcxConnectionConnect(connectionHandle,connectionType);
    }

    public static CompletableFuture<String> vcxConnectionConnect(int connectionHandle, String connectionType) throws VcxException {
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        ParamGuard.notNullOrWhiteSpace(connectionType, "connectionType");
        logger.debug("vcxAcceptInvitation() called with: connectionHandle = [" + connectionHandle + "], connectionType = [" + connectionType + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_connect(
                commandHandle,
                connectionHandle,
                connectionType,
                vcxConnectionConnectCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxConnectionSerializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String serializedData) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], serializedData = [" + serializedData + "]");
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
        logger.debug("connectionSerialize() called with: connectionHandle = [" + connectionHandle + "]");
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
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], connectionHandle = [" + connectionHandle + "]");
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
        logger.debug("connectionDeserialize() called with: connectionData = [" + connectionData + "]");
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
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(0);
        }
    };

    public static CompletableFuture<Integer> deleteConnection(int connectionHandle) throws VcxException {
        logger.debug("deleteConnection() called with: connectionHandle = [" + connectionHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_delete_connection(commandHandle, connectionHandle, vcxConnectionDeleteCB);
        checkResult(result);
        return future;
    }

    private static Callback vcxConnectionInviteDetailsCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String details) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], details = [" + details + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(details);
        }
    };

    public static CompletableFuture<String> connectionInviteDetails(int connectionHandle, int abbreviated) throws VcxException {
        logger.debug("connectionInviteDetails() called with: connectionHandle = [" + connectionHandle + "], abbreviated = [" + abbreviated + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_connection_invite_details(commandHandle, connectionHandle, abbreviated, vcxConnectionInviteDetailsCB);
        checkResult(result);
        return future;
    }


    public static int connectionRelease(int handle) throws VcxException {
        logger.debug("connectionRelease() called with: handle = [" + handle + "]");
        ParamGuard.notNull(handle, "handle");
        int result = LibVcx.api.vcx_connection_release(handle);
        checkResult(result);

        return result;
    }

    private static Callback vcxConnectionGetStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int state) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], state = [" + state + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(state);
        }
    };

    public static CompletableFuture<Integer> connectionGetState(int connnectionHandle) throws VcxException {
        logger.debug("connectionGetState() called with: connnectionHandle = [" + connnectionHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_connection_get_state(commandHandle, connnectionHandle, vcxConnectionGetStateCB);
        checkResult(result);
        return future;
    }
}
