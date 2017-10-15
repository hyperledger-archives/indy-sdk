package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown an an anonymous credential has not been issued.
 */
public class NotIssuedException extends IndyException
{
	private static final long serialVersionUID = -1936666689407460229L;
	private final static String message = "The anoncreds is not issued.";

	/**
	 * Initializes a new NotIssuedException.
	 */
	public NotIssuedException() 
    {
    	super(message, ErrorCode.AnoncredsNotIssuedError.value());
    }
}