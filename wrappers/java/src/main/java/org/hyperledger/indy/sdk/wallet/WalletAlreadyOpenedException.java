package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyException;

public class WalletAlreadyOpenedException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 3294831240096535507L;
	private final static String message = "The wallet is already open.";

	public WalletAlreadyOpenedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}