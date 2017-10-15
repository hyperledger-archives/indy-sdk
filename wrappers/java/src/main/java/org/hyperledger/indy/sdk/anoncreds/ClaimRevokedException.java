package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when attempting to use a claim that has been revoked.
 */
public class ClaimRevokedException extends IndyException
{
	private static final long serialVersionUID = 8269746965241515882L;
	private final static String message = "The claim has been revoked.";

	/**
	 * Initializes a new ClaimRevokedException.
	 */
	public ClaimRevokedException() 
    {
    	super(message, ErrorCode.AnoncredsClaimRevoked.value());
    }
}