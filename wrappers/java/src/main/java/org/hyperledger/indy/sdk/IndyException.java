package org.hyperledger.indy.sdk;

public class IndyException extends Exception {

	private static final long serialVersionUID = 2650355290834266477L;

	public IndyException(String message) {

		super(message);
	}

	public static IndyException fromErrorCode(ErrorCode errorCode, int err) {

		return new IndyException("" + (errorCode == null ? null : errorCode.name()) + ": " + (errorCode == null ? null : errorCode.value()) + " (" + Integer.toString(err) + ")");
	}
}
