package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.IndyException;

public class PoolLedgerTerminatedException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 768482152424714514L;
	private final static String message = "The pool ledger was terminated.";

	public PoolLedgerTerminatedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}