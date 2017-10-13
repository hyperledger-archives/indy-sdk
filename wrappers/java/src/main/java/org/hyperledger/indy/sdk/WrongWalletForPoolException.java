package org.hyperledger.indy.sdk;

public class WrongWalletForPoolException extends IndyException
{
	private final static String message = "The wallet specified is not compatible with the open pool.";

	WrongWalletForPoolException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}
