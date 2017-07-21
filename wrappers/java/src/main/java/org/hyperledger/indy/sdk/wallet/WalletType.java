package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;

import com.sun.jna.Callback;
import com.sun.jna.Pointer;
import com.sun.jna.ptr.PointerByReference;

public abstract class WalletType {

	private Callback createCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(String name, String config, String credentials) {

			return WalletType.this.create(name, config, credentials).ordinal();
		}
	};

	private Callback openCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(String name, String config, String runtime_config, String credentials, Pointer handle) {

			return WalletType.this.open(name, config, runtime_config, credentials, handle).ordinal();
		}
	};

	private Callback setCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key, String value) {

			return WalletType.this.set(handle, key, value).ordinal();
		}
	};

	private Callback getCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key, PointerByReference value_ptr) {

			return WalletType.this.get(handle, key, value_ptr).ordinal();
		}
	};

	private Callback getNotExpiredCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key, PointerByReference value_ptr) {

			return WalletType.this.getNotExpired(handle, key, value_ptr).ordinal();
		}
	};

	private Callback listCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle, String key_prefix, PointerByReference values_json_ptr) {

			return WalletType.this.list(handle, key_prefix, values_json_ptr).ordinal();
		}
	};

	private Callback closeCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int handle) {

			return WalletType.this.close(handle).ordinal();
		}
	};

	private Callback deleteCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(String name, String config, String credentials) {

			return WalletType.this.delete(name, config, credentials).ordinal();
		}
	};

	private Callback freeCb = new Callback() {

		@SuppressWarnings("unused")
		public int callback(int wallet_handle, Pointer value) {

			return WalletType.this.free(wallet_handle, value).ordinal();
		}
	};

	public abstract ErrorCode create(String name, String config, String credentials);
	public abstract ErrorCode open(String name, String config, String runtimeConfig, String credentials, Pointer handle);
	public abstract ErrorCode set(int handle, String key, String value);
	public abstract ErrorCode get(int handle, String key, PointerByReference valuePtr);
	public abstract ErrorCode getNotExpired(int handle, String key, PointerByReference valuePtr);
	public abstract ErrorCode list(int handle, String keyPrefx, PointerByReference valuesJsonPtr);
	public abstract ErrorCode close(int handle);
	public abstract ErrorCode delete(String name, String config, String credentials);
	public abstract ErrorCode free(int walletHandle, Pointer value);
	
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
