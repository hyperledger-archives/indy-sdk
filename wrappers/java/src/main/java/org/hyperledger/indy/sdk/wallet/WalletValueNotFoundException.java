package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyException;

public class WalletValueNotFoundException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 667964860056778208L;
	private final static String message = "No value with the specified key exists in the wallet from which it was requested.";

	public WalletValueNotFoundException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}
