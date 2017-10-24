package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exceptiont thrown when a message submitted to the ledger is rejected because it is invalid.
 */
public class InvalidLedgerTransactionException extends IndyException
{
	private static final long serialVersionUID = -8270805534959603741L;
	private final static String message = "The ledger message is unknown or malformed.";

	/**
	 * Initializes a new InvalidLedgerTransactionException.
	 */
	public InvalidLedgerTransactionException() 
    {
    	super(message, ErrorCode.LedgerInvalidTransaction.value());
    }
}