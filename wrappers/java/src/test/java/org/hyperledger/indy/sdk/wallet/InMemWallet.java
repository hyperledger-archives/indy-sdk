package org.hyperledger.indy.sdk.wallet;

import java.util.Date;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.wallet.WalletType.StringByReference;
import org.json.JSONArray;
import org.json.JSONObject;

public class InMemWallet implements CustomWallet {

	private Map<String, WalletRecord> records = new ConcurrentHashMap<String, WalletRecord>();
	private int freshnessDuration;
	private Boolean isOpen = false;
	
	public InMemWallet(int freshnessDuration) {
		this.freshnessDuration = freshnessDuration;
	}
	
	public void setIsOpen(Boolean value) {
		isOpen = value;
	}
	
	public Boolean getIsOpen() {
		return isOpen;
	}
	
	@Override
	public ErrorCode set(String key, String value) {
		WalletRecord record = new WalletRecord(value);

		if(records.containsKey(key))
			records.replace(key, record);
		else
			records.put(key, record);

        return ErrorCode.Success;
	}

	@Override
	public ErrorCode get(String key, StringByReference resultString) {
		
		if (!records.containsKey(key))
            return ErrorCode.WalletNotFoundError;

        WalletRecord record = records.get(key);

        resultString.setValue(record.value);

        return ErrorCode.Success;
	}

	@Override
	public ErrorCode getNotExpired(String key, StringByReference resultString) {
		
		if (!records.containsKey(key))
            return ErrorCode.WalletNotFoundError;

		WalletRecord record = records.get(key);
        long recordAge = new Date().getTime() - record.created.getTime();

        if (recordAge > freshnessDuration)
            return ErrorCode.WalletNotFoundError;

        resultString.setValue(record.value);

        return ErrorCode.Success;
	}

	@Override
	public ErrorCode list(String keyPrefix, StringByReference resultString) {

		JSONArray jsonValues = new JSONArray();
		
		for (Iterator<Map.Entry<String, WalletRecord>> iterator = records.entrySet().iterator(); iterator.hasNext(); ) {

			Map.Entry<String, WalletRecord> entry = iterator.next();
			String key = entry.getKey();
			WalletRecord record = entry.getValue();
			
			if (!key.startsWith(keyPrefix)) 
				continue;
			
			JSONObject valueObject = new JSONObject();
			valueObject.put("key", key);
			valueObject.put("value", record.value);
			
			jsonValues.put(valueObject);
		}
		
		JSONObject resultObject = new JSONObject();
		resultObject.put("values", jsonValues);
		
        resultString.setValue(resultObject.toString());

        return ErrorCode.Success;
	}
	
	private class WalletRecord {
		
		public WalletRecord(String value) {
			this.value = value; 
		}
		
		public String value;
		public Date created = new Date();
		
	}

}
