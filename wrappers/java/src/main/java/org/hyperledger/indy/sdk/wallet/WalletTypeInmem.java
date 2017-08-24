package org.hyperledger.indy.sdk.wallet;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;

import org.hyperledger.indy.sdk.ErrorCode;

import com.sun.jna.Native;
import com.sun.jna.Pointer;
import com.sun.jna.ptr.PointerByReference;

public class WalletTypeInmem extends WalletType {

	private static WalletTypeInmem instance;

	public static WalletTypeInmem getInstance() {

		if (instance == null) instance = new WalletTypeInmem();
		return instance;
	}

	private WalletTypeInmem() {

	}

	@Override
	public ErrorCode create(String name, String config, String credentials) {

		if (this.walletsByName.containsKey(name)) return ErrorCode.CommonInvalidState;

		WalletInmem wallet = new WalletInmem();
		this.walletsByName.put(name, wallet);
		this.walletsByHandle.put(wallet.handle, wallet);

		return ErrorCode.Success;
	}

	@Override
	public ErrorCode open(String name, String config, String runtimeConfig, String credentials, Pointer handle) {

		WalletInmem wallet = this.walletsByName.get(name);
		if (wallet == null) return ErrorCode.CommonInvalidState;

		wallet.open = true;

		handle.setInt(0, wallet.handle);
		return ErrorCode.Success;
	}

	@Override
	public ErrorCode set(int handle, String key, String value) {

		WalletInmem wallet = this.walletsByHandle.get(handle);
		if (wallet == null) return ErrorCode.CommonInvalidState;

		wallet.values.put(key, value);

		return ErrorCode.Success;
	}

	@Override
	public ErrorCode get(int handle, String key, PointerByReference valuePtr) {

		WalletInmem wallet = this.walletsByHandle.get(handle);
		if (wallet == null) return ErrorCode.CommonInvalidState;

		if (! wallet.values.containsKey(key)) return ErrorCode.WalletNotFoundError;

		String value = wallet.values.get(key);

		byte[] bytes = Native.toByteArray(value, "UTF-8");
		Pointer pointer = new Pointer(Native.malloc(bytes.length));
		pointer.write(0, bytes, 0, bytes.length);
		valuePtr.setValue(pointer);
		return ErrorCode.Success;
	}

	@Override
	public ErrorCode getNotExpired(int handle, String key, PointerByReference valuePtr) {

		WalletInmem wallet = this.walletsByHandle.get(handle);
		if (wallet == null) return ErrorCode.CommonInvalidState;

		if (! wallet.values.containsKey(key)) return ErrorCode.WalletNotFoundError;

		String value = wallet.values.get(key);

		byte[] bytes = Native.toByteArray(value, "UTF-8");
		Pointer pointer = new Pointer(Native.malloc(bytes.length));
		pointer.write(0, bytes, 0, bytes.length);
		valuePtr.setValue(pointer);
		return ErrorCode.Success;
	}

	@Override
	public ErrorCode list(int handle, String keyPrefix, PointerByReference valuesJsonPtr) {

		WalletInmem wallet = this.walletsByHandle.get(handle);
		if (wallet == null) return ErrorCode.CommonInvalidState;

		StringBuilder builder = new StringBuilder();
		builder.append("{\"values\":[");

		for (Iterator<Map.Entry<String, String>> iterator = wallet.values.entrySet().iterator(); iterator.hasNext(); ) {

			Map.Entry<String, String> entry = iterator.next();
			String key = entry.getKey();
			String value = entry.getValue();
			if (!key.startsWith(keyPrefix)) continue;
			String pair = String.format("{\"key\":\"%s\", \"value\":\"%s\"}", key, escapeJson(value));
			builder.append(pair);
			if (iterator.hasNext()) builder.append(",");
		}

		builder.append("]}");

		byte[] bytes = Native.toByteArray(builder.toString(), "UTF-8");
		Pointer pointer = new Pointer(Native.malloc(bytes.length));
		pointer.write(0, bytes, 0, bytes.length);
		valuesJsonPtr.setValue(pointer);
		return ErrorCode.Success;
	}

	@Override
	public ErrorCode close(int handle) {

		WalletInmem wallet = this.walletsByHandle.get(handle);
		if (wallet == null) return ErrorCode.CommonInvalidState;

		wallet.open = false;

		return ErrorCode.Success;
	}

	@Override
	public ErrorCode delete(String name, String config, String credentials) {

		if (! this.walletsByName.containsKey(name)) return ErrorCode.CommonInvalidState;

		WalletInmem wallet = new WalletInmem();
		this.walletsByName.remove(name);
		this.walletsByHandle.remove(wallet.handle);

		return ErrorCode.Success;
	}

	@Override
	public ErrorCode free(int walletHandle, Pointer value) {

		Native.free(Pointer.nativeValue(value));

		return ErrorCode.Success;
	}

	private static String escapeJson(String string) {

		return string.replace("\\", "\\\\").replace("\"", "\\\"");
	}

	private AtomicInteger atomicInteger = new AtomicInteger();
	private Map<String, WalletInmem> walletsByName = new ConcurrentHashMap<String, WalletInmem>();
	private Map<Integer, WalletInmem> walletsByHandle = new ConcurrentHashMap<Integer, WalletInmem>();

	private int newHandle() {

		return Integer.valueOf(atomicInteger.incrementAndGet());
	}

	public void clear() {
		this.walletsByName.clear();
		this.walletsByHandle.clear();
	}

	private class WalletInmem {

		private int handle;
		private boolean open;
		private Map<String, String> values;

		private WalletInmem() {

			this.handle = WalletTypeInmem.this.newHandle();
			this.open = false;
			this.values = new HashMap<>();
		}
	}
}
