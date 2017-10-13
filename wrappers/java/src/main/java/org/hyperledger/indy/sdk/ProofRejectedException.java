package org.hyperledger.indy.sdk;

public class ProofRejectedException extends IndyException
{
	private final static String message = "The proof has been rejected.";

	ProofRejectedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}