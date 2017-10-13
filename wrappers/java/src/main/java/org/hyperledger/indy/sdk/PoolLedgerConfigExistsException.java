package org.hyperledger.indy.sdk;

public class PoolLedgerConfigExistsException extends IndyException
{
	private final static String message = "A pool ledger configuration already exists with the specified name.";

	PoolLedgerConfigExistsException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}