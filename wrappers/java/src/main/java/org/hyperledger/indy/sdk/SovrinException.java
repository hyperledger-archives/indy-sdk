package org.hyperledger.indy.sdk;

public class SovrinException extends Exception {

	private static final long serialVersionUID = 2650355290834266477L;

	public SovrinException(String message) {

		super(message);
	}

	public static SovrinException fromErrorCode(ErrorCode errorCode, int err) {

		return new SovrinException("" + (errorCode == null ? null : errorCode.name()) + ": " + (errorCode == null ? null : errorCode.value()) + " (" + Integer.toString(err) + ")");
	}
}
