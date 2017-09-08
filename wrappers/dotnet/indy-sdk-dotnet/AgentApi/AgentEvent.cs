using System;

namespace Hyperledger.Indy.AgentApi
{
    /// <summary>
    /// Base class for events raised by <see cref="AgentConnection"/> and <see cref="AgentListener"/>
    /// instances.
    /// </summary>
    public abstract class AgentEvent
    {
        /// <summary>
        /// Initializes a new AgentEvent.
        /// </summary>
        /// <param name="handle">The handle the event is for.</param>
        /// <param name="result">The result for the event.</param>
        internal AgentEvent(IntPtr handle, ErrorCode result)
        {
            Handle = handle;
            Result = result;
        }

        /// <summary>
        /// Gets the handle of the owner of the event.
        /// </summary>
        internal IntPtr Handle { get; }

        /// <summary>
        /// Gets the error result for the event.
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
