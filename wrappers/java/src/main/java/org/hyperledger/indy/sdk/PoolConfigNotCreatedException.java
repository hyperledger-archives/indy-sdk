package org.hyperledger.indy.sdk;

public class PoolConfigNotCreatedException extends IndyException
{
	private final static String message = "The requested pool cannot be opened because it does not have an existing configuration.";

	PoolConfigNotCreatedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}