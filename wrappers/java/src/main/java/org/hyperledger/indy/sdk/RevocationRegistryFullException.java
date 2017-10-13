package org.hyperledger.indy.sdk;

public class RevocationRegistryFullException extends IndyException
{
	private final static String message = "The specified revocation registry is full.  Another revocation registry must be created.";

	RevocationRegistryFullException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}