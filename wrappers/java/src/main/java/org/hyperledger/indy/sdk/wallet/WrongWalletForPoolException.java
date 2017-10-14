package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyException;

public class WrongWalletForPoolException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -8931044806844925321L;
	private final static String message = "The wallet specified is not compatible with the open pool.";

	public WrongWalletForPoolException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}
