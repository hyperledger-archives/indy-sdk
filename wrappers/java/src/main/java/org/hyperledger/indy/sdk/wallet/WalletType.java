package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;

import com.sun.jna.Callback;
import com.sun.jna.Pointer;
import com.sun.jna.ptr.PointerByReference;

/**
 * Base type for implementing custom wallet types.
 */
public abstract class WalletType {

	/**
	 * Callback called when a wallet is being created.
	 */
	private Callback createCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(String name, String config, String credentials) {

			return WalletType.this.create(name, config, credentials).ordinal();
		}
	};

	/**
	 * Callback called when a wallet is being opened.
	 */
	private Callback openCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(String name, String config, String runtime_config, String credentials, Pointer handle) {

			return WalletType.this.open(name, config, runtime_config, credentials, handle).ordinal();
		}
	};

	/**
	 * Callback called when a value is being set on a wallet.
	 */
	private Callback setCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key, String value) {

			return WalletType.this.set(handle, key, value).ordinal();
		}
	};

	/**
	 * Callback called value is requested from a wallet.
	 */
	private Callback getCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key, PointerByReference value_ptr) {

			return WalletType.this.get(handle, key, value_ptr).ordinal();
		}
	};

	/**
	 * Callback called when an unexpired value is being requested from a wallet.
	 */
	private Callback getNotExpiredCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key, PointerByReference value_ptr) {

			return WalletType.this.getNotExpired(handle, key, value_ptr).ordinal();
		}
	};

	/**
	 * Callback called when a list of values is being requested from a wallet.
	 */
	private Callback listCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key_prefix, PointerByReference values_json_ptr) {

			return WalletType.this.list(handle, key_prefix, values_json_ptr).ordinal();
		}
	};

	/**
	 * Callback called when a wallet is being closed.
	 */
	private Callback closeCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle) {

			return WalletType.this.close(handle).ordinal();
		}
	};

	/**
	 * Callback called when a wallet is being deleted.
	 */
	private Callback deleteCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(String name, String config, String credentials) {

			return WalletType.this.delete(name, config, credentials).ordinal();
		}
	};

	/**
	 * Callback called when value requested from a wallet is no longer being used and should be freed.
	 */
	private Callback freeCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int wallet_handle, Pointer value) {

			return WalletType.this.free(wallet_handle, value).ordinal();
		}
	};

	/**
	 * Creates a new wallet.
	 * 
	 * @param name The name of the wallet to create.
	 * @param config The configuration of the wallet.
	 * @param credentials The credentials of the wallet.
	 * @return An Errorcode indicating the outcome.
	 */
	public abstract ErrorCode create(String name, String config, String credentials);
	
	/**
	 * Opens a wallet.
	 * 
	 * @param name The name of the wallet to open.
	 * @param config The configuration for the wallet specified on creation.
	 * @param runtimeConfig The runtime configuration for the wallet.
	 * @param credentials The credentials for the wallet.
	 * @param handle A handle for the opened wallet instance to be set by the implementer.
	 * @return An Errorcode indicating the outcome.
	 */
	public abstract ErrorCode open(String name, String config, String runtimeConfig, String credentials, Pointer handle);
	
	/**
	 * Sets a value on a wallet instance.
	 * 
	 * @param handle The handle of the wallet to set the value on.
	 * @param key The key to set the value for.
	 * @param value The value to set.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode set(int handle, String key, String value);
	
	/**
	 * Gets a value from a wallet instance.
	 * 
	 * @param handle The handle of the wallet to set the value on.
	 * @param key The key of value to get.
	 * @param valuePtr A pointer to the value to be set by implementers.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode get(int handle, String key, PointerByReference valuePtr);
	
	/**
	 * Gets an unexpired value from a wallet instance.
	 * 
	 * @param handle The handle of the wallet to set the value on.
	 * @param key The key of value to get.
	 * @param valuePtr A pointer to the return value to be set by implementers.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode getNotExpired(int handle, String key, PointerByReference valuePtr);
	
	/**
	 * Gets a list of values optionally filtered by key.
	 * 
	 * @param handle The handle of the wallet to set the value on.
	 * @param keyPrefx The prefix of the keys to filter on.  If null no filter will be applied.
	 * @param valuesJsonPtr A pointer to the return value to be set by implementers.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode list(int handle, String keyPrefx, PointerByReference valuesJsonPtr);
	
	/**
	 * Closes a wallet.
	 * 
	 * @param handle The handle of the wallet to close.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode close(int handle);
	
	/**
	 * Deletes a wallet.
	 * 
	 * @param name The name of the wallet.
	 * @param config The configuration of the wallet.
	 * @param credentials The credentials of the wallet.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode delete(String name, String config, String credentials);
	
	/**
	 * Frees a value returned by a wallet.
	 * 
	 * @param walletHandle The handle of the wallet the value belongs to.
	 * @param value A pointer to the value to be freed.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode free(int walletHandle, Pointer value);
	
	/**
	 * Gets the create callback.
	 * 
	 * @return The create callback.
	 */
	public Callback getCreateCb() {
		return createCb;
	}

	/**
	 * Gets the open callback.
	 * 
	 * @return The open callback.
	 */
	public Callback getOpenCb() {
		return openCb;
	}

	/**
	 * Gets the set callback.
	 * 
	 * @return The set callback.
	 */
	public Callback getSetCb() {
		return setCb;
	}

	/**
	 * Gets the get callback.
	 * 
	 * @return The get callback.
	 */
	public Callback getGetCb() {
		return getCb;
	}

	/**
	 * Gets the getNotExpired callback.
	 * 
	 * @return The getNotExpired callback.
	 */
	public Callback getGetNotExpiredCb() {
		return getNotExpiredCb;
	}

	/**
	 * Gets the list callback.
	 * 
	 * @return The list callback.
	 */
	public Callback getListCb() {
		return listCb;
	}

	/**
	 * Gets the close callback.
	 * 
	 * @return The close callback.
	 */
	public Callback getCloseCb() {
		return closeCb;
	}

	/**
	 * Gets the delete callback.
	 * 
	 * @return The delete callback.
	 */
	public Callback getDeleteCb() {
		return deleteCb;
	}

	/**
	 * Gets the free callback.
	 * 
	 * @return The free callback.
	 */
	public Callback getFreeCb() {
		return freeCb;
	}
}
