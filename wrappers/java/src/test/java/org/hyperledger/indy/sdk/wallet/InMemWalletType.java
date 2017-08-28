package org.hyperledger.indy.sdk.wallet;

import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;

import org.hyperledger.indy.sdk.ErrorCode;
import org.json.JSONObject;

public class InMemWalletType extends WalletType {

	private Map<Integer, InMemWallet> openWallets = new ConcurrentHashMap<Integer, InMemWallet>();
	private Map<String, InMemWallet> configuredWallets = new ConcurrentHashMap<String, InMemWallet>();
	
    private static AtomicInteger nextWalletHandle = new AtomicInteger();
    
    private int getNextWalletHandle() {
    	return Integer.valueOf(nextWalletHandle.incrementAndGet());
    }
    
    @Override
    protected CustomWallet getWalletByHandle(int handle) {
    	return openWallets.get(handle);
    }    
	
	@Override
	public ErrorCode create(String name, String config, String credentials) {
		
		if (configuredWallets.containsKey(name))
            return ErrorCode.WalletAlreadyExistsError;

        int freshnessDuration = 1000;
        
        if(config != null){
        	
	        JSONObject configObj =  new JSONObject(config);
	        
	        if(configObj != null && !configObj.isNull("freshness_time")) {
	            freshnessDuration = configObj.getInt("freshness_time");
	        }
        }

        configuredWallets.put(name, new InMemWallet(freshnessDuration));
        return ErrorCode.Success;
	}
	
	@Override
	public ErrorCode delete(String name, String config, String credentials) {
		
		if (!configuredWallets.containsKey(name))
            return ErrorCode.CommonInvalidState;

        InMemWallet wallet = configuredWallets.get(name);

        if (wallet.getIsOpen())
            return ErrorCode.CommonInvalidState;

        configuredWallets.remove(name);

        return ErrorCode.Success;
	}
	
	@Override
	public ErrorCode open(String name, String config, String runtimeConfig, String credentials,
			HandleByReference walletHandle) {
		
        if (!configuredWallets.containsKey(name))
            return ErrorCode.CommonInvalidState;

        InMemWallet wallet = configuredWallets.get(name);

        if (wallet.getIsOpen())
            return ErrorCode.WalletAlreadyOpenedError;

        wallet.setIsOpen(true);

        int newHandle = getNextWalletHandle();
        
        walletHandle.setValue(newHandle);
        openWallets.put(newHandle, wallet);

        return ErrorCode.Success;
	}
	
	@Override
	public ErrorCode close(int handle) {
		
		InMemWallet wallet;

        try
        {
            wallet = (InMemWallet)getWalletByHandle(handle);
        }
        catch(Exception e)
        {
            return ErrorCode.WalletInvalidHandle;
        }
        
        wallet.setIsOpen(false);
        openWallets.remove(handle);

        return ErrorCode.Success;
	}
}

