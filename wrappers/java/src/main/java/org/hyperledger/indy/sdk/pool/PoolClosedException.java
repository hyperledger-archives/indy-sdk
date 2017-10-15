package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when attempting to use a poll that has already been closed.
 */
public class PoolClosedException extends IndyException
{
	private static final long serialVersionUID = 7124250084655044699L;
	private final static String message = "The pool is closed and cannot be used.";

	/**
	 * Initializes a new PoolClosedExecption.
	 */
	public PoolClosedException() 
    {
    	super(message, ErrorCode.PoolLedgerInvalidPoolHandle.value());
    }
}