package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.IndyException;

public class PoolClosedException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 7124250084655044699L;
	private final static String message = "The pool is closed and cannot be used.";

	public PoolClosedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}