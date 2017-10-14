package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.IndyException;

public class NotIssuedException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -1936666689407460229L;
	private final static String message = "The anoncreds is not issued.";

	public NotIssuedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}