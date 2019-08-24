package com.evernym.sdk.vcx.schema;


import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.concurrent.CompletableFuture;

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
                                                          String version,
                                                          String data,
                                                          int paymentHandle) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        ParamGuard.notNullOrWhiteSpace(schemaName, "schemaName");
        ParamGuard.notNullOrWhiteSpace(version, "version");
        ParamGuard.notNullOrWhiteSpace(data, "data");
        logger.debug("schemaCreate() called with: sourceId = [" + sourceId + "], schemaName = [" + schemaName + "], version = [" + version + "]" + " data = <" + data + ">" + " payment_handle = <" + paymentHandle + ">");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_schema_create(
                commandHandle,
                sourceId,
                schemaName,
                version,
                data,
                paymentHandle,
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
        public void callback(int commandHandle, int err,int schemaHandle, String schemaAttributes) {
            logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], schemaHandle = [" + schemaHandle +  "],  schemaAttributes = [" + schemaAttributes + "]");
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(commandHandle);
            if (!checkCallback(future, err)) return;
            future.complete(schemaAttributes);
        }
    };

    public static CompletableFuture<String> schemaGetAttributes( String sourceId, String schemaId) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        logger.debug("schemaGetAttributes() called with: sourceId = [" + sourceId + "], schemaHandle = [" + schemaId + "]");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_schema_get_attributes(commandHandle, sourceId,schemaId, schemaGetAttributesCB);
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

    public static int schemaRelease(
            int schemaHandle
    ) throws VcxException {
        ParamGuard.notNull(schemaHandle, "schemaHandle");
        logger.debug("schemaRelease() called with: schemaHandle = [" + schemaHandle + "]");

        return LibVcx.api.vcx_schema_release(schemaHandle);
    }

    private static Callback schemaPrepareForEndorserCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int handle, String transaction) {
            logger.debug("callback() called with: command_handle = [" + command_handle + "], err = [" + err + "], handle = [" + handle + "], transaction = [" + transaction + "]");
            CompletableFuture<SchemaPrepareForEndorserResult> future = (CompletableFuture<SchemaPrepareForEndorserResult>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            SchemaPrepareForEndorserResult result = new SchemaPrepareForEndorserResult(handle, transaction);
            future.complete(result);
        }
    };

    public static CompletableFuture<SchemaPrepareForEndorserResult> schemaPrepareForEndorser(String sourceId,
                                                                                             String schemaName,
                                                                                             String version,
                                                                                             String data,
                                                                                             String endorser) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId, "sourceId");
        ParamGuard.notNull(schemaName, "schemaName");
        ParamGuard.notNull(version, "version");
        ParamGuard.notNull(data, "data");
        ParamGuard.notNull(endorser, "endorserendorser");
	    logger.debug("schemaCreate() called with: sourceId = [" + sourceId + "], schemaName = [" + schemaName + "], version = [" + version + "]" + " data = <" + data + ">" + " endorser = <" + endorser + ">");
        CompletableFuture<SchemaPrepareForEndorserResult> future = new CompletableFuture<SchemaPrepareForEndorserResult>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_schema_prepare_for_endorser(
                commandHandle,
		        sourceId,
		        schemaName,
		        version,
		        data,
		        endorser,
		        schemaPrepareForEndorserCB);
        checkResult(result);

        return future;
    }

	private static Callback vcxIntegerCB = new Callback() {
		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int commandHandle, int err, int s) {
			logger.debug("callback() called with: commandHandle = [" + commandHandle + "], err = [" + err + "], s = [" + s + "]");
			CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(commandHandle);
			if (!checkCallback(future, err)) return;
			Integer result = s;
			future.complete(result);
		}
	};

	public static CompletableFuture<Integer> schemaUpdateState(int schemaHandle) throws VcxException {
		logger.debug("vcxSchemaUpdateState() called with: schemaHandle = [" + schemaHandle + "]");
		CompletableFuture<Integer> future = new CompletableFuture<>();
		int commandHandle = addFuture(future);

		int result = LibVcx.api.vcx_schema_update_state(
				commandHandle,
				schemaHandle,
				vcxIntegerCB
		);
		checkResult(result);
		return future;
	}

	public static CompletableFuture<Integer> schemaGetState(int schemaHandle) throws VcxException {
		logger.debug("schemaGetState() called with: schemaHandle = [" + schemaHandle + "]");
		CompletableFuture<Integer> future = new CompletableFuture<>();
		int commandHandle = addFuture(future);

		int result = LibVcx.api.vcx_schema_get_state(
				commandHandle,
				schemaHandle,
				vcxIntegerCB
		);
		checkResult(result);
		return future;
	}
}
