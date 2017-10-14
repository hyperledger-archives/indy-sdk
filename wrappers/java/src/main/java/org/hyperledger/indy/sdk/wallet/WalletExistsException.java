package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyException;

public class WalletExistsException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 1829076830401150667L;
	private final static String message = "A wallet with the specified name already exists.";

	public WalletExistsException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}