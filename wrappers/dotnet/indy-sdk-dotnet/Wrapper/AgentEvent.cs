using System;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Event raised by an Agent.
    /// </summary>
    public abstract class AgentEvent
    {
        /// <summary>
        /// Initializes a new AgentEvent.
        /// </summary>
        /// <param name="handle">The handle the event is for.</param>
        /// <param name="result">The result for the event.</param>
        public AgentEvent(IntPtr handle, ErrorCode result)
        {
            Handle = handle;
            Result = result;
        }

        /// <summary>
        /// Gets the handle of the owner of the message.
        /// </summary>
        public IntPtr Handle { get; }

        /// <summary>
        /// Gets the error result for the 
        /// </summary>
        public ErrorCode Result { get; }

        /// <summary>
        /// Gets whether or not the result was success.
        /// </summary>
        public bool IsSuccess
        {
            get { return Result == ErrorCode.Success; }
        }
    }
}
