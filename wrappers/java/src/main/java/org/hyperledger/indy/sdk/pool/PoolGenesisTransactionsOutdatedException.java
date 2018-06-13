package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when attempting to connect to pool with outdated genesis transactions
 */
public class PoolGenesisTransactionsOutdatedException extends IndyException
{
	private static final long serialVersionUID = 6945180938262170499L;
	private final static String message = "Pool Genesis transactions has outdated format.";

	/**
	 * Initializes a new PoolConfigNotCreatedException.
	 */
	public PoolGenesisTransactionsOutdatedException()
    {
    	super(message, ErrorCode.PoolGenesisTransactionsOutdated.value());
    }
}