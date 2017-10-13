package org.hyperledger.indy.sdk;

public class WalletClosedException extends IndyException
{
	private final static String message = "The wallet is closed and cannot be used.";

	WalletClosedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}
