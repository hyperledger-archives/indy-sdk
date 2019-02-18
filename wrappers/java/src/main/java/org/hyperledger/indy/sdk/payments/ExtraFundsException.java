package org.hyperledger.indy.sdk.payments;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

public class ExtraFundsException extends IndyException {
    private static final long serialVersionUID = 6397499268992083529L;
    private static final String message = "Extra funds on inputs";

    /**
     * Initializes a new {@link ExtraFundsException} with the specified message.
     */
    public ExtraFundsException() {
        super(message, ErrorCode.ExtraFundsError.value());
    }

    /**
     * Initializes a new {@link ExtraFundsException} with the specified message.
     *
     * @param sdkMessage The SDK error message.
     * @param sdkBacktrace The SDK error backtrace.
     */
    public ExtraFundsException(String sdkMessage, String sdkBacktrace) {
        super(sdkMessage, ErrorCode.ExtraFundsError.value(), sdkBacktrace);
    }
}
