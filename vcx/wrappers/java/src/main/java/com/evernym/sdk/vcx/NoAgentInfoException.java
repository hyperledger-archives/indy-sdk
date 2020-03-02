package com.evernym.sdk.vcx;

/**
 * Exception thrown when the SDK reports than agent pairwise information not found.
 */
public class NoAgentInfoException extends VcxException {

	private static final long serialVersionUID = -1802344846222826490L;
	private final static String message = "Agent pairwise information not found";


	public NoAgentInfoException()
	{
		super(message, ErrorCode.NO_AGENT_INFO.value());
	}
}