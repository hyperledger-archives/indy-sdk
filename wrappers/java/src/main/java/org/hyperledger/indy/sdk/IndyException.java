package org.hyperledger.indy.sdk;

public class IndyException extends Exception {

	private static final long serialVersionUID = 2650355290834266477L;

	private ErrorCode errorCode;

	public IndyException(String message) {

		super(message);
	}

	public IndyException(ErrorCode errorCode) {
		this(String.format("%s: %d", errorCode.name(), errorCode.value()));
		this.errorCode = errorCode;
	}

	public ErrorCode getErrorCode() {
		return errorCode;
	}
}
