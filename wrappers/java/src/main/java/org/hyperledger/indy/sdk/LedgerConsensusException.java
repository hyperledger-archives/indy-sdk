package org.hyperledger.indy.sdk;

public class LedgerConsensusException extends IndyException
{
	private final static String message = "No consensus was reached during the ledger operation.";

	LedgerConsensusException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}