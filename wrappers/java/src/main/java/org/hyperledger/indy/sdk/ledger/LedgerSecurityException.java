package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyException;

public class LedgerSecurityException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 1695822815015877550L;
	private final static String message = "The transaction cannot be sent as the privileges for the current pool connection don't allow it.";

	public LedgerSecurityException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}