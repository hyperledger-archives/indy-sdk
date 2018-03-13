package blob_storage;

import com.sun.jna.Callback;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;

import java.util.concurrent.CompletableFuture;

/**
 * blob_storage.rs API
 */

/**
 * High level wrapper for wallet SDK functions.
 */
public class BlobStorage extends IndyJava.API {

	private final int tailsReaderHandle;

	private BlobStorage(int tailsReaderHandle) {

		this.tailsReaderHandle = tailsReaderHandle;
	}

	/**
	 * Gets the handle for the blob storage.
	 *
	 * @return The handle for the blob storage.
	 */
	public int getTailsReaderHandle() {

		return this.tailsReaderHandle;
	}

	/*
	 * STATIC CALLBACKS
	 */

	/**
	 * Callback used when openReader completes.
	 */
	private static Callback openReaderCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, int handle) {

			CompletableFuture<BlobStorage> future = (CompletableFuture<BlobStorage>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			BlobStorage tailsReader = new BlobStorage(handle);

			BlobStorage result = tailsReader;
			future.complete(result);
		}
	};

	/*
	 * STATIC METHODS
	 */

	public static CompletableFuture<BlobStorage> openReader(
			String type,
			String config,
			String location,
			String hash) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(type, "type");
		ParamGuard.notNullOrWhiteSpace(config, "config");
		ParamGuard.notNullOrWhiteSpace(location, "location");
		ParamGuard.notNullOrWhiteSpace(hash, "hash");

		CompletableFuture<BlobStorage> future = new CompletableFuture<BlobStorage>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_blob_storage_open_reader(
				commandHandle,
				type,
				config,
				location,
				hash,
				openReaderCb);

		checkResult(result);

		return future;
	}
}