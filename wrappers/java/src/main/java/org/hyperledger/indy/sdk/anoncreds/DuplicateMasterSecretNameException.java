package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.IndyException;

public class DuplicateMasterSecretNameException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 7180454759216991453L;
	private final static String message = "Another master-secret with the specified name already exists.";

	public DuplicateMasterSecretNameException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}