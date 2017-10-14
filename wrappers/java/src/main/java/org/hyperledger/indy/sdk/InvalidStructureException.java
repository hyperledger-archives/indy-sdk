package org.hyperledger.indy.sdk;

public class InvalidStructureException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -2157029980107821313L;
	private final static String message = "A value being processed is not valid.";

	public InvalidStructureException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}
