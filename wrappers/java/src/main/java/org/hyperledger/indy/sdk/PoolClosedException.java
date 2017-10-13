package org.hyperledger.indy.sdk;

public class PoolClosedException extends IndyException
{
	private final static String message = "The pool is closed and cannot be used.";

	PoolClosedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}