package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.IndyException;

public class ProofRejectedException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = -5100028213117687183L;
	private final static String message = "The proof has been rejected.";

	public ProofRejectedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}