using System;
using System.Runtime.Serialization;

namespace Indy.Sdk.Dotnet
{
    /// <summary>
    /// Exception indicating a problem originating from Sovrin.
    /// </summary>
    [Serializable]
    public sealed class IndyException : Exception
    {
        /// <summary>
        /// Intializes a new SovrinException with a message and error code.
        /// </summary>
        /// <param name="message">The message for the exception.</param>
        /// <param name="errorCode">The error code for the exception.</param>
        public IndyException(String message, ErrorCode errorCode) : base(message)
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
            if (!Enum.IsDefined(typeof(ErrorCode), errorCode))
                throw new InvalidCastException(string.Format("The error #{0} does not have a corresponding ErrorCode value.", errorCode));

            var message = string.Format("{0}:{1}", Enum.GetName(typeof(ErrorCode), errorCode), errorCode);
            return new IndyException(message, (ErrorCode)errorCode);            
        }

        /// <summary>
        /// Gets the error code for the exception.
        /// </summary>
        public ErrorCode ErrorCode { get; private set; }

        /// <summary>
        /// Gets object data for ISerializable.
        /// </summary>
        /// <param name="info"></param>
        /// <param name="context"></param>
        public override void GetObjectData(SerializationInfo info, StreamingContext context)
        {
            base.GetObjectData(info, context);
        }
    }
}
