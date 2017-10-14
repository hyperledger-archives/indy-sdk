package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyException;

public class InvalidLedgerTransactionException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -8270805534959603741L;
	private final static String message = "The ledger message is unknown or malformed.";

	public InvalidLedgerTransactionException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}