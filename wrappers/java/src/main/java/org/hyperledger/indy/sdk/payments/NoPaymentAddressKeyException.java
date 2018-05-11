package org.hyperledger.indy.sdk.payments;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

public class NoPaymentAddressKeyException extends IndyException {
    private static final long serialVersionUID = -4687621498850055893L;
    private static final String message = "No payment address key found in wallet";

    /**
     * Initializes a new {@link NoPaymentAddressKeyException} with the specified message.
     */
    public NoPaymentAddressKeyException() {
        super(message, ErrorCode.NoPaymentAddressKeyError.value());
    }
}
