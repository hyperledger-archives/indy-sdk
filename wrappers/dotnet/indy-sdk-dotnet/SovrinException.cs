using System;

namespace Indy.Sdk.Dotnet
{
    /// <summary>
    /// Exception indicating a problem originating from Sovrin.
    /// </summary>
    public class SovrinException : Exception
    {
        /// <summary>
        /// Intializes a new SovrinException with a message and error code.
        /// </summary>
        /// <param name="message">The message for the exception.</param>
        /// <param name="errorCode">The error code for the exception.</param>
        public SovrinException(String message, int errorCode) : base(message)
        {
            ErrorCode = errorCode;
        }

        /// <summary>
        /// Generates a SovrinException from the provided error code.
        /// </summary>
        /// <param name="errorCode">The error code.</param>
        /// <returns>A SovrinException instance.</returns>
        public static SovrinException fromErrorCode(int errorCode)
        {
            if (Enum.IsDefined(typeof(ErrorCode), errorCode))
            {
                var message = string.Format("{0}:{1}", Enum.GetName(typeof(ErrorCode), errorCode), errorCode);
                return new SovrinException(message, errorCode);
            }
            else
            {
                var message = string.Format("An unknown error occurred ({0}).", errorCode);
                return new SovrinException(message, errorCode);
            }
        }

        /// <summary>
        /// Gets the error code for the exception.
        /// </summary>
        public int ErrorCode { get; }
    }
}
