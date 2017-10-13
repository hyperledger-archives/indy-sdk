package org.hyperledger.indy.sdk;

public class WalletAlreadyOpenedException extends IndyException
{
	private final static String message = "The wallet is already open.";

	WalletAlreadyOpenedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}