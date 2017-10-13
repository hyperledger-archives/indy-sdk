package org.hyperledger.indy.sdk;

public class InvalidLedgerTransactionException extends IndyException
{
	private final static String message = "The ledger message is unknown or malformed.";

	InvalidLedgerTransactionException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}