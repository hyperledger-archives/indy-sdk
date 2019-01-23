package org.hyperledger.indy.sdk.payments;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception is thrown when information is incompatible e.g. 2 different payment methods in inputs and outputs
 */
public class IncompatiblePaymentException extends IndyException {

    private static final long serialVersionUID = 5531031012103688872L;
    private static final String message = "Information passed to libindy is incompatible";

    /**
     * Initializes a new {@link IncompatiblePaymentException} with the specified message.
     */
    public IncompatiblePaymentException() {
        super(message, ErrorCode.IncompatiblePaymentError.value());
    }

    /**
     * Initializes a new {@link IncompatiblePaymentException} with the specified message.
     *
     * @param sdkMessage The SDK error message.
     * @param sdkBacktrace The SDK error backtrace.
     */
    public IncompatiblePaymentException(String sdkMessage, String sdkBacktrace) {
        super(sdkMessage, ErrorCode.IncompatiblePaymentError.value(), sdkBacktrace);
    }
}
