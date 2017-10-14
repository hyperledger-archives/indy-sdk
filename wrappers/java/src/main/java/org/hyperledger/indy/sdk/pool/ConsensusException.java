package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.IndyException;

public class ConsensusException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -6503578332467229584L;
	private final static String message = "No consensus was reached during the ledger operation.";

	public ConsensusException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}