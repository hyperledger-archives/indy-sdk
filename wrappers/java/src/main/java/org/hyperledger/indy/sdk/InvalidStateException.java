package org.hyperledger.indy.sdk;

public class InvalidStateException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -1741244553102207886L;
	private final static String message = "The SDK library experienced an unexpected internal error.";

	public InvalidStateException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}
