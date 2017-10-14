package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyException;

public class UnknownCryptoException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 4955846571270561834L;
	private final static String message = "An unknown crypto format has been used for a DID entity key.";

	public UnknownCryptoException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}