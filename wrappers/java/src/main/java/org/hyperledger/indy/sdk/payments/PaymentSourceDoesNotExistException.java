package org.hyperledger.indy.sdk.payments;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

public class PaymentSourceDoesNotExistException extends IndyException {
    private static final long serialVersionUID = -5009466707967765943L;
    private static final String message = "No such source found";

    /**
     * Initializes a new {@link PaymentSourceDoesNotExistException} with the specified message.
     */
    public PaymentSourceDoesNotExistException() {
        super(message, ErrorCode.PaymentSourceDoesNotExistError.value());
    }

    /**
     * Initializes a new {@link PaymentSourceDoesNotExistException} with the specified message.
     *
     * @param sdkMessage The SDK error message.
     * @param sdkBacktrace The SDK error backtrace.
     */
    public PaymentSourceDoesNotExistException(String sdkMessage, String sdkBacktrace) {
        super(sdkMessage, ErrorCode.PaymentSourceDoesNotExistError.value(), sdkBacktrace);
    }
}
