package org.hyperledger.indy.sdk;

public class UnknownCryptoException extends IndyException
{
	private final static String message = "An unknown crypto format has been used for a DID entity key.";

	UnknownCryptoException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}