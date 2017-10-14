package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.IndyException;

public class ClaimRevokedException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 8269746965241515882L;
	private final static String message = "The claim has been revoked.";

	public ClaimRevokedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}