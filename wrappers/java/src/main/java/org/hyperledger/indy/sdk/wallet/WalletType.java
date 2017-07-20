package org.hyperledger.indy.sdk.wallet;

import com.sun.jna.Callback;
import com.sun.jna.Pointer;

public abstract class WalletType {

	private Callback createCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(String name, String config, String credentials) {

			WalletType.this.create(name, config, credentials);
		}
	};

	private Callback openCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(String name, String config, String runtime_config, String credentials, Pointer handle) {

			WalletType.this.open(name, config, runtime_config, credentials, handle);
		}
	};

	private Callback setCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int handle, String key, String value) {

			WalletType.this.set(handle, key, value);
		}
	};

	private Callback getCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int handle, String key, Pointer value_ptr) {

			WalletType.this.get(handle, key, value_ptr);
		}
	};

	private Callback getNotExpiredCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int handle, String key, Pointer value_ptr) {

			WalletType.this.getNotExpired(handle, key, value_ptr);
		}
	};

	private Callback listCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int handle, String key_prefix, Pointer values_json_ptr) {

			WalletType.this.list(handle, key_prefix, values_json_ptr);
		}
	};

	private Callback closeCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int handle) {

			WalletType.this.close(handle);
		}
	};

	private Callback deleteCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(String name, String config, String credentials) {

			WalletType.this.delete(name, config, credentials);
		}
	};

	private Callback freeCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int wallet_handle, String value) {

			WalletType.this.free(wallet_handle, value);
		}
	};

	public abstract void create(String name, String config, String credentials);
	public abstract void open(String name, String config, String runtimeConfig, String credentials, Pointer handle);
	public abstract void set(int handle, String key, String value);
	public abstract void get(int handle, String key, Pointer valuePtr);
	public abstract void getNotExpired(int handle, String key, Pointer valuePtr);
	public abstract void list(int handle, String keyPrefx, Pointer valuesJsonPtr);
	public abstract void close(int handle);
	public abstract void delete(String name, String config, String credentials);
	public abstract void free(int walletHandle, String value);
	
	public Callback getCreateCb() {
		return createCb;
	}

	public Callback getOpenCb() {
		return openCb;
	}

	public Callback getSetCb() {
		return setCb;
	}

	public Callback getGetCb() {
		return getCb;
	}

	public Callback getGetNotExpiredCb() {
		return getNotExpiredCb;
	}

	public Callback getListCb() {
		return listCb;
	}

	public Callback getCloseCb() {
		return closeCb;
	}

	public Callback getDeleteCb() {
		return deleteCb;
	}

	public Callback getFreeCb() {
		return freeCb;
	}
}
