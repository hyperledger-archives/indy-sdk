package org.hyperledger.indy.sdk;

public class PoolLedgerTerminatedException extends IndyException
{
	private final static String message = "The pool ledger was terminated.";

	PoolLedgerTerminatedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}