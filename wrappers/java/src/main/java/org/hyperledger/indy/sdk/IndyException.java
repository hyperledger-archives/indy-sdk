package org.hyperledger.indy.sdk;

/**
 * Indy specific exception.
 */
public class IndyException extends Exception {

	private static final long serialVersionUID = 2650355290834266477L;

	private ErrorCode errorCode;

	/**
	 * Initializes a new IndyException with the specified message.
	 * 
	 * @param message The message for the exception.
	 */
	public IndyException(String message) {

		super(message);
	}

	/**
	 * Initializes a new IndyException using the specified ErrorCode.
	 * 
	 * @param errorCode The error code for the exception.
	 */
	public IndyException(ErrorCode errorCode) {
		this(String.format("%s: %d", errorCode.name(), errorCode.value()));
		this.errorCode = errorCode;
	}

	/**
	 * Gets the ErrorCode for the exception.
	 * 
	 * @return The ErrorCode used to construct the exception.
	 */
	public ErrorCode getErrorCode() {
		return errorCode;
	}
}
