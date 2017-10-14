package org.hyperledger.indy.sdk;

public class IOException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -1581785238453075780L;
	private final static String message = "An IO error occurred.";

	public IOException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}