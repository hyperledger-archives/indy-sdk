package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyException;

public class UnknownWalletTypeException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -6275711661964891560L;
	private final static String message = "The wallet type specified has not been registered.";

	public UnknownWalletTypeException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}