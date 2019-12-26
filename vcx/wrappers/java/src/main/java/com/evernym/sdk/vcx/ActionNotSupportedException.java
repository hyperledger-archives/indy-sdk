package com.evernym.sdk.vcx;

/**
 * Exception thrown when the SDK reports than an action is not supported.
 */
public class ActionNotSupportedException extends VcxException {

	private static final long serialVersionUID = -1802344846222826490L;
	private final static String message = "Action is not supported";


	public ActionNotSupportedException()
	{
		super(message, ErrorCode.ACTION_NOT_SUPPORTED.value());
	}
}