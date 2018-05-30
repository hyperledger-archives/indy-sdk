package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when invalid cache is cleared.
 */
public class InvalidPoolCacheException extends IndyException
{
    private static final long serialVersionUID = 3067464663432608988L;
    private final static String message = "TInvalid cache cleared.";

    /**
     * Initializes a new PoolClosedExecption.
     */
    public InvalidPoolCacheException()
    {
        super(message, ErrorCode.PoolLedgerInvalidCacheError.value());
    }
}