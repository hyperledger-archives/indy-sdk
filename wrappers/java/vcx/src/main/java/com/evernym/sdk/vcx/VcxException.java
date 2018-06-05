package com.evernym.sdk.vcx;



/**
 * Thrown when an Indy specific error has occurred.
 */
public class VcxException extends Exception {

	private static final long serialVersionUID = 2650355290834266234L;
	private int sdkErrorCode;

	/**
	 * Initializes a new VcxException with the specified message.
	 * 
	 * @param message The message for the exception.
	 */
	protected VcxException(String message, int sdkErrorCode) {
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
	 * Initializes a new VcxException using the specified SDK error code.
	 * 
	 * @param sdkErrorCode The SDK error code to construct the exception from.
	 */
	public static VcxException fromSdkError(int sdkErrorCode) {
		
		ErrorCode errorCode = ErrorCode.valueOf(sdkErrorCode);
		
		switch(errorCode){

			default:
				String message = String.format("An unmapped error with the code '%s' was returned by the SDK.", sdkErrorCode);
				return new VcxException(message, sdkErrorCode);
		}
	}
}


