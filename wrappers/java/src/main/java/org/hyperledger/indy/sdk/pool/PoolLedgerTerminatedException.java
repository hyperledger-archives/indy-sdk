package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when a pool ledger has been terminated.
 */
public class PoolLedgerTerminatedException extends IndyException
{
	private static final long serialVersionUID = 768482152424714514L;
	private final static String message = "The pool ledger was terminated.";

	/**
	 * Initializes a new PoolLedgerTerminatedException.
	 */
	public PoolLedgerTerminatedException()
	{
		super(message, ErrorCode.PoolLedgerTerminated.value());
	}

	/**
	 * Initializes a new PoolLedgerTerminatedException.
	 *
	 * @param sdkMessage The SDK error message.
	 * @param sdkBacktrace The SDK error backtrace.
	 */
	public PoolLedgerTerminatedException(String sdkMessage, String sdkBacktrace)
    {
    	super(sdkMessage, ErrorCode.PoolLedgerTerminated.value(), sdkBacktrace);
    }
}