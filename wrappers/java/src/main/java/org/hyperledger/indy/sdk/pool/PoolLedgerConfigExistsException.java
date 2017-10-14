package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.IndyException;

public class PoolLedgerConfigExistsException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 2032790158242533689L;
	private final static String message = "A pool ledger configuration already exists with the specified name.";

	public PoolLedgerConfigExistsException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}