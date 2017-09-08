using System;

namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating a problem originating from the Indy SDK.
    /// </summary>
    public sealed class IndyException : Exception
    {
        /// <summary>
        /// Initializes a new IndyException with a message and error code.
        /// </summary>
        /// <param name="message">The message for the exception.</param>
        /// <param name="errorCode">The error code for the exception.</param>
        public IndyException(String message, ErrorCode errorCode) : base(message)
        {
            ErrorCode = errorCode;
        }

        /// <summary>
        /// Generates an IndyException from the provided error code.
        /// </summary>
        /// <param name="errorCode">The error code.</param>
        /// <returns>An IndyException instance.</returns>
        public static IndyException FromErrorCode(int errorCode)
        {
            if (!Enum.IsDefined(typeof(ErrorCode), errorCode))
                throw new InvalidCastException(string.Format("The error #{0} does not have a corresponding ErrorCode value.", errorCode));

            var message = string.Format("{0}:{1}", Enum.GetName(typeof(ErrorCode), errorCode), errorCode);
            return new IndyException(message, (ErrorCode)errorCode);            
        }

        /// <summary>
        /// Gets the error code for the exception.
        /// </summary>
        public ErrorCode ErrorCode { get; private set; }
    }
}
