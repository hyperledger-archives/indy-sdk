package org.hyperledger.indy.sdk;

public class AnoncredsNotIssuedException extends IndyException
{
	private final static String message = "The anoncreds is not issued.";

	AnoncredsNotIssuedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}