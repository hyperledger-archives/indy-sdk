package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Attempt to open encrypted wallet with invalid credentials
 */
public class WalletSecurityException extends IndyException
{
	private static final long serialVersionUID = 3294831240096535507L;
	private final static String message = "The wallet security error.";

	/**
	 * Initializes a new WalletSecurityException.
	 */
	public WalletSecurityException()
    {
    	super(message, ErrorCode.WalletSecurityError.value());
    }
}