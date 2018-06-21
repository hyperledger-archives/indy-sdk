package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.wallet.WalletType.StringByReference;

/**
 * All custom wallets must inherit from this base class.
 */
public interface CustomWallet {

	/**
	 * Sets a value on a wallet instance.
	 * 
	 * @param key The key to set the value for.
	 * @param value The value to set.
	 * @return An ErrorCode indicating the outcome.
	 */
	ErrorCode set(String key, String value);
	
	/**
	 * Gets a value from a wallet instance.
	 * 
	 * @param key The key of value to get.
	 * @param resultString A result object to contain the value set by implementers.
	 * @return An ErrorCode indicating the outcome.
	 */
	ErrorCode get(String key, StringByReference resultString);
	
	/**
	 * Gets an unexpired value from a wallet instance.
	 * 
	 * @param key The key of value to get.
	 * @param resultString A result object to contain the value set by implementers.
	 * @return An ErrorCode indicating the outcome.
	 */
	ErrorCode getNotExpired(String key, StringByReference resultString);
	
	/**
	 * Gets a list of values optionally filtered by key.
	 * 
	 * @param keyPrefix The prefix of the keys to filter on.  If null no filter will be applied.
	 * @param resultString A result object to contain the value set by implementers.
	 * @return An ErrorCode indicating the outcome.
	 */
	ErrorCode list(String keyPrefix, StringByReference resultString);   
}
