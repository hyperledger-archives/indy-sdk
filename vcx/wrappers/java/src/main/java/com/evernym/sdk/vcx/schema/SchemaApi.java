package com.evernym.sdk.vcx.schema;


import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java9.util.concurrent.CompletableFuture;

public class SchemaApi extends VcxJava.API {
    private static final Logger logger = LoggerFactory.getLogger("SchemaApi");
    private static Callback schemaCreateCB = new Callback() {
        // TODO: This callback and jna definition needs to be fixed for this API
        // it should accept connection handle as well
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int schemaHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], schemaHandle = [" + schemaHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            Integer result = schemaHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> schemaCreate(String sourceId,
                                                          String schemaName,
                                                          String schemaDate) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        ParamGuard.notNullOrWhiteSpace(sourceId, "schemaName");
        ParamGuard.notNullOrWhiteSpace(sourceId, "schemaDate");
        logger.debug("schemaCreate() called with: sourceId = [" + sourceId + "], schemaName = [" + schemaName + "], schemaDate = [" + schemaDate + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_schema_create(
                commandHandle,
                sourceId,
                schemaName,
                schemaDate,
                schemaCreateCB
        );
        checkResult(result);
        return future;
    }

    private static Callback schemaSerializeHandle = new Callback() {
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
            String result = serializedData;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> schemaSerialize(int schemaHandle) throws VcxException {
        ParamGuard.notNull(schemaHandle, "schemaHandle");
        logger.debug("schemaSerialize() called with: schemaHandle = [" + schemaHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_schema_serialize(
                commandHandle,
                schemaHandle,
                schemaSerializeHandle
        );
        checkResult(result);
        return future;
    }

    private static Callback schemaDeserializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, int schemaHandle) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], schemaHandle = [" + schemaHandle + "]");
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            Integer result = schemaHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> schemaDeserialize(String schemaData) throws VcxException {
        ParamGuard.notNull(schemaData, "schemaData");
        logger.debug("schemaDeserialize() called with: schemaData = [" + schemaData + "]");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_schema_deserialize(
                commandHandle,
                schemaData,
                schemaDeserializeCB
        );
        checkResult(result);
        return future;
    }

    private static Callback schemaGetAttributesCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String schemaAttributes) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], schemaAttributes = [" + schemaAttributes + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(schemaAttributes);
        }
    };

    public static CompletableFuture<String> schemaGetAttributes( String sourceId, int sequenceNo) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        logger.debug("schemaGetAttributes() called with: sourceId = [" + sourceId + "], sequenceNo = [" + sequenceNo + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_schema_get_attributes(commandHandle, sourceId,sequenceNo, schemaGetSchemaID);
        checkResult(result);
        return future;
    }

    private static Callback schemaGetSchemaID = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int commandHandle, int err, String schemaId) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], schemaId = [" + schemaId + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(schemaId);
        }
    };

    public static CompletableFuture<String> schemaGetSchemaId( int schemaHandle) throws VcxException {
        ParamGuard.notNull(schemaHandle, "SchemaHandle");
        logger.debug("schemaGetSchemaId() called with: schemaHandle = [" + schemaHandle + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_schema_get_schema_id(commandHandle,schemaHandle, schemaGetSchemaID);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> schemaRelease(
            int schemaHandle
    ) throws VcxException {
        ParamGuard.notNull(schemaHandle, "schemaHandle");
        logger.debug("schemaRelease() called with: schemaHandle = [" + schemaHandle + "]");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();

        int result = LibVcx.api.vcx_schema_release(schemaHandle);
        checkResult(result);

        return future;
    }
}
