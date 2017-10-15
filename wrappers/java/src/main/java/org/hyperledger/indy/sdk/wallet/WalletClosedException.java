package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when attempting to use a wallet that has been closed.
 */
public class WalletClosedException extends IndyException
{
	private static final long serialVersionUID = -606730416804502147L;
	private final static String message = "The wallet is closed and cannot be used.";

	/**
	 * Initializes a new WalletClosedException.
	 */
	public WalletClosedException() 
    {
    	super(message, ErrorCode.WalletInvalidHandle.value());
    }
}
