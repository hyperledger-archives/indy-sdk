package com.evernym.indy.jnitest;

/**
 * Created by abdussami on 24/04/18.
 */

public class IndyException extends Exception {

    private static final long serialVersionUID = 2650355290834266477L;
    private int sdkErrorCode;

    /**
     * Initializes a new IndyException with the specified message.
     *
     * @param message The message for the exception.
     */
    protected IndyException(String message, int sdkErrorCode) {
        super(message);
        this.sdkErrorCode = sdkErrorCode;
    }

    /**
     * Gets the SDK error code for the exception.
     *
     * @return The SDK error code used to construct the exception.
     */
    public int getSdkErrorCode() {
        return sdkErrorCode;
    }

    /**
     * Initializes a new IndyException using the specified SDK error code.
     *
     * @param sdkErrorCode The SDK error code to construct the exception from.
     */
    public static IndyException fromSdkError(int sdkErrorCode) {

        ErrorCode errorCode = ErrorCode.valueOf(sdkErrorCode);

        return new IndyException("error",sdkErrorCode);
    }
}