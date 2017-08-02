package org.hyperledger.indy.sdk.wallet;

import java.util.ArrayList;
import java.util.List;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.wallet.WalletType.StringByReference;

import com.sun.jna.Native;
import com.sun.jna.Pointer;

/**
 * All custom wallets must inherit from this base class.
 */
public abstract class CustomWalletBase implements AutoCloseable {

	/// <summary>
    /// Pointers to values that have been allocated to unmanaged memory by the wallet.
    /// </summary>
    private List<Pointer> valuePointers = new ArrayList<Pointer>();
    
    public List<Pointer> getValuePointers(){
    	return valuePointers;
    }

	/**
	 * Sets a value on a wallet instance.
	 * 
	 * @param handle The handle of the wallet to set the value on.
	 * @param key The key to set the value for.
	 * @param value The value to set.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode set(String key, String value);
	
	/**
	 * Gets a value from a wallet instance.
	 * 
	 * @param handle The handle of the wallet to set the value on.
	 * @param key The key of value to get.
	 * @param resultString A result object to containthe value set by implementers.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode get(String key, StringByReference resultString);
	
	/**
	 * Gets an unexpired value from a wallet instance.
	 * 
	 * @param handle The handle of the wallet to set the value on.
	 * @param key The key of value to get.
	 * @param resultString A result object to containthe value set by implementers.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode getNotExpired(String key, StringByReference resultString);
	
	/**
	 * Gets a list of values optionally filtered by key.
	 * 
	 * @param handle The handle of the wallet to set the value on.
	 * @param keyPrefx The prefix of the keys to filter on.  If null no filter will be applied.
	 * @param resultString A result object to containthe value set by implementers.
	 * @return An ErrorCode indicating the outcome.
	 */
	public abstract ErrorCode list(String keyPrefix, StringByReference resultString);
    
	
	/**
	 * Closes the wallet.
	 */
	@Override
	public void close() throws Exception {
		//Free any outstanding handles.
        for (int i = valuePointers.size() - 1; i >= 0; i--)
        {
        	Pointer valuePointer = valuePointers.get(i);
        	Native.free(Pointer.nativeValue(valuePointer));
            valuePointers.remove(i);
        }
	}

}
