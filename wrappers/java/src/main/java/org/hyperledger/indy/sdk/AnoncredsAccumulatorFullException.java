package org.hyperledger.indy.sdk;

public class AnoncredsAccumulatorFullException extends IndyException
{
	private final static String message = "The anoncreds accumulator is full.";

	AnoncredsAccumulatorFullException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}