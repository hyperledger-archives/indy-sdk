package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyException;

public class WalletClosedException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -606730416804502147L;
	private final static String message = "The wallet is closed and cannot be used.";

	public WalletClosedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}
