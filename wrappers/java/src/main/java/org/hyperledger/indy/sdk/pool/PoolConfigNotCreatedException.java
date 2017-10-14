package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.IndyException;

public class PoolConfigNotCreatedException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 6945180938262170499L;
	private final static String message = "The requested pool cannot be opened because it does not have an existing configuration.";

	public PoolConfigNotCreatedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}