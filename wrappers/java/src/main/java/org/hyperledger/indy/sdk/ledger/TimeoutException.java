package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

import java.io.Serializable;

/**
 * Exception thrown when timeout happens for ledger operation.
 */
public class TimeoutException extends IndyException implements Serializable {
    private static final long serialVersionUID = -2318833884012610163L;
    private final static String message = "Timeout happens for ledger operation.";

    /**
     * Initializes a new TimeoutException.
     */
    public TimeoutException() {
        super(message, ErrorCode.PoolLedgerTimeout.value());
    }

    /**
     * Initializes a new TimeoutException.
     *
     * @param sdkMessage The SDK error message.
     * @param sdkBacktrace The SDK error backtrace.
     */
    public TimeoutException(String sdkMessage, String sdkBacktrace) {
        super(sdkMessage, ErrorCode.PoolLedgerTimeout.value(), sdkBacktrace);
    }
}
