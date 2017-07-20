package org.hyperledger.indy.sdk.wallet;

import com.sun.jna.Pointer;

public class WalletTypeMemory extends WalletType {

	private static WalletTypeMemory instance;

	public static WalletTypeMemory getInstance() {

		if (instance == null) instance = new WalletTypeMemory();
		return instance;
	}

	private WalletTypeMemory() {

	}

	@Override
	public void create(String name, String config, String credentials) {
		// TODO Auto-generated method stub
		
	}

	@Override
	public void open(String name, String config, String runtimeConfig, String credentials, Pointer handle) {
		// TODO Auto-generated method stub
		
	}

	@Override
	public void set(int handle, String key, String value) {
		// TODO Auto-generated method stub
		
	}

	@Override
	public void get(int handle, String key, Pointer valuePtr) {
		// TODO Auto-generated method stub
		
	}

	@Override
	public void getNotExpired(int handle, String key, Pointer valuePtr) {
		// TODO Auto-generated method stub
		
	}

	@Override
	public void list(int handle, String keyPrefix, Pointer valuesJsonPtr) {
		// TODO Auto-generated method stub
		
	}

	@Override
	public void close(int handle) {
		// TODO Auto-generated method stub
		
	}

	@Override
	public void delete(String name, String config, String credentials) {
		// TODO Auto-generated method stub
		
	}

	@Override
	public void free(int WalletHandle, String value) {
		// TODO Auto-generated method stub
		
	}

}
