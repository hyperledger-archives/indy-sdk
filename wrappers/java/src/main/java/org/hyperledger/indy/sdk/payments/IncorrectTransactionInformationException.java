package org.hyperledger.indy.sdk.payments;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception is thrown when information about transaction is incorrect e.g. two equal UTXO/payment addresses in inputs/outputs
 */
public class IncorrectTransactionInformationException extends IndyException {
    private static final String message = "Incorrect information has been passed to transaction";
    private static final long serialVersionUID = -4373419742343305739L;

    /**
     * Initializes a new {@link IncorrectTransactionInformationException} with the specified message.
     */
    public IncorrectTransactionInformationException() {
        super(message, ErrorCode.IncorrectTransactionInformationError.value());
    }
}
