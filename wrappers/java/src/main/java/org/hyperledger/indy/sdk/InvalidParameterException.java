package org.hyperledger.indy.sdk;

public class InvalidParameterException extends IndyException {

	/**
	 * 
	 */
	private static final long serialVersionUID = -1802344846222826490L;
	private int parameterIndex;
	
	private static int getParamIndex(int sdkErrorCode)
    {
        assert(sdkErrorCode >= 100 && sdkErrorCode <= 111);
        return sdkErrorCode - 99;
    }

    private static String buildMessage(int sdkErrorCode)
    {
        return String.format("The value passed to parameter %s is not valid.", getParamIndex(sdkErrorCode));
    }

    public InvalidParameterException(int sdkErrorCode)
    {
    	super(buildMessage(sdkErrorCode), sdkErrorCode);
        parameterIndex = getParamIndex(sdkErrorCode);
    }

    /// <summary>
    /// Gets the index of the parameter that contained the invalid value.
    /// </summary>
    public int getParameterIndex() {
    	return parameterIndex;
    }
}
