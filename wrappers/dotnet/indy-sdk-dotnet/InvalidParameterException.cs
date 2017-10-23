using System.Diagnostics;

namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating that one of the parameters provided to an SDK call contained a valid that was considered invalid.
    /// </summary>
    public class InvalidParameterException : IndyException
    {
        /// <summary>
        /// Gets the index of the parameter from the SDK error code.
        /// </summary>
        /// <param name="sdkErrorCode">The SDK error code.</param>
        /// <returns>The parameter index the SDK indicated was invalid.</returns>
        private static int GetParamIndex(int sdkErrorCode)
        {
            Debug.Assert((int)sdkErrorCode >= 100 && (int)sdkErrorCode <= 111);

            return (int)sdkErrorCode - 99;
        }

        /// <summary>
        /// Builds the error message from the SDK error code.
        /// </summary>
        /// <param name="sdkErrorCode">Teh SDK error code.</param>
        /// <returns>The error message.</returns>
        private static string BuildMessage(int sdkErrorCode)
        {
            return string.Format("The value passed to parameter {0} is not valid.", GetParamIndex(sdkErrorCode));
        }

        /// <summary>
        /// Initializes a new InvalidParameterException from the specified SDK error code.
        /// </summary>
        /// <param name="sdkErrorCode">The SDK error code that specifies which parameter was invalid.</param>
        internal InvalidParameterException(int sdkErrorCode) : base(BuildMessage(sdkErrorCode), sdkErrorCode)
        {
            ParameterIndex = GetParamIndex(sdkErrorCode);
        }

        /// <summary>
        /// Gets the index of the parameter that contained the invalid value.
        /// </summary>
        public int ParameterIndex { get; private set; }
    }

}
