package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when an anoncreds accumulator is full.
 */
public class AccumulatorFullException extends IndyException
{
	private static final long serialVersionUID = -6792822612990030627L;
	private final static String message = "The anoncreds accumulator is full.";

	/**
	 * Initializes a new AccumulatorFullException.
	 */
	public AccumulatorFullException() 
    {
    	super(message, ErrorCode.AnoncredsAccumulatorIsFull.value());
    }
}