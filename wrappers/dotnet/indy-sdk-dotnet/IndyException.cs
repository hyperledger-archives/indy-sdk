using System;

namespace Indy.Sdk.Dotnet
{
    /// <summary>
    /// Exception indicating a problem originating from Sovrin.
    /// </summary>
    public class IndyException : Exception
    {
        /// <summary>
        /// Intializes a new SovrinException with a message and error code.
        /// </summary>
        /// <param name="message">The message for the exception.</param>
        /// <param name="errorCode">The error code for the exception.</param>
        public IndyException(String message, int errorCode) : base(message)
        {
            ErrorCode = errorCode;
        }

        /// <summary>
        /// Generates a SovrinException from the provided error code.
        /// </summary>
        /// <param name="errorCode">The error code.</param>
        /// <returns>A SovrinException instance.</returns>
        public static IndyException fromErrorCode(int errorCode)
        {
            if (Enum.IsDefined(typeof(ErrorCode), errorCode))
            {
                var message = string.Format("{0}:{1}", Enum.GetName(typeof(ErrorCode), errorCode), errorCode);
                return new IndyException(message, errorCode);
            }
            else
            {
                var message = string.Format("An unknown error occurred ({0}).", errorCode);
                return new IndyException(message, errorCode);
            }
        }

        /// <summary>
        /// Gets the error code for the exception.
        /// </summary>
        public int ErrorCode { get; }
    }
}
