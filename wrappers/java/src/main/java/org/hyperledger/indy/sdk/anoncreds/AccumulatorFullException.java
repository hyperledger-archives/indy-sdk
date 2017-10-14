package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.IndyException;

public class AccumulatorFullException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -6792822612990030627L;
	private final static String message = "The anoncreds accumulator is full.";

	public AccumulatorFullException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}