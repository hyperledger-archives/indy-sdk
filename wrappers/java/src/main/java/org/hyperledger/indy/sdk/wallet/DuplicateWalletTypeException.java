package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyException;

public class DuplicateWalletTypeException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -5414881660233778407L;
	private final static String message = "A wallet type with the specified name has already been registered.";

	public DuplicateWalletTypeException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}