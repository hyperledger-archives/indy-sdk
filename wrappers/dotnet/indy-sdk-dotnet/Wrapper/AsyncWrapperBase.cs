using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.IndyNativeMethods;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Base class for all asynchronous wrapper classes.
    /// </summary>
    public abstract class AsyncWrapperBase
    {
        /// <summary>
        /// The next command handle to use.
        /// </summary>
        private static int _nextCommandHandle = 0;

        /// <summary>
        /// Gets the map of command handles and their task completion sources.
        /// </summary>
        private static IDictionary<int, object> _taskCompletionSources = new ConcurrentDictionary<int, object>();

        /// <summary>
        /// Gets the callback to use for functions that don't return a value.
        /// </summary>
        internal static NoValueDelegate _noValueCallback = (xcommand_handle, err) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<bool>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(true);
        };

        /// <summary>
        /// Checks the result from a Sovrin function call.
        /// </summary>
        /// <exception cref="IndyException">If the result is not a success result a SovrinException will be thrown.</exception>
        /// <param name="result">The result to check.</param>
        protected static void CheckResult(int result)
        {
            if (result != (int)ErrorCode.Success)
                throw IndyException.fromErrorCode(result);
        }

        /// <summary>
        /// Checks the result of a callback made by the Sovrin library.
        /// </summary>
        /// <typeparam name="T">The type the promise will return.</typeparam>
        /// <param name="taskCompletionSource">The source controlling the async result.</param>
        /// <param name="errorCode">The error code returned to the callback by the sovrin function.</param>
        /// <returns>true if the error code was success, otherwise false.</returns>
        /// <exception cref="IndyException">If the errorCode is not a success result a SovrinException will be thrown.</exception>
        protected static bool CheckCallback<T>(TaskCompletionSource<T> taskCompletionSource, int errorCode)
        {
            if (errorCode != (int)ErrorCode.Success)
            {
                taskCompletionSource.SetException(IndyException.fromErrorCode(errorCode));
                return false;
            }

            return true;
        }

        /// <summary>
        /// Checks the result of a callback made by the Sovrin library.
        /// </summary>
        /// <param name="errorCode">The error code returned to the callback by the sovrin function.</param>
        /// <exception cref="IndyException">If the errorCode is not a success result a SovrinException will be thrown.</exception>
        protected static void CheckCallback(int errorCode)
        {
            if (errorCode != (int)ErrorCode.Success)
                throw IndyException.fromErrorCode(errorCode);
        }


        /// <summary>
        /// Gets the next command handle.
        /// </summary>
        /// <returns>The next command handle.</returns>
        protected static int GetNextCommandHandle()
        {
            return Interlocked.Increment(ref _nextCommandHandle);
        }

        /// <summary>
        /// Adds a new TaskCompletionSource to track.
        /// </summary>
        /// <typeparam name="T">The type of the TaskCompletionSource result.</typeparam>
        /// <param name="taskCompletionSource">The TaskCompletionSource to track.</param>
        /// <returns>The command handle to use for tracking the task completion source.</returns>
        protected static int AddTaskCompletionSource<T>(TaskCompletionSource<T> taskCompletionSource)
        {
            var commandHandle = GetNextCommandHandle();
            _taskCompletionSources.Add(commandHandle, taskCompletionSource);
            return commandHandle;
        }

        /// <summary>
        /// Gets and temoves a TaskCompletionResult from tracking.
        /// </summary>
        /// <typeparam name="T">The type of the TaskCompletionResult that was tracked.</typeparam>
        /// <param name="commandHandle">The command handle used for tracking the TaskCompletionResult.</param>
        /// <returns>The TaskCompletionResult associated with the command handle.</returns>
        protected static TaskCompletionSource<T> RemoveTaskCompletionSource<T>(int commandHandle)
        {
            Debug.Assert(_taskCompletionSources.ContainsKey(commandHandle), string.Format("No task completion source is currently registered for the command with the handle '{0}'.", commandHandle));
                
            var taskCompletionSource = _taskCompletionSources[commandHandle];
            _taskCompletionSources.Remove(commandHandle);
            var result = taskCompletionSource as TaskCompletionSource<T>;

            Debug.Assert(result != null, string.Format("No  task completion source of the specified type is registered for the command with the handle '{0}'.", commandHandle));

            return result;
        }

    }
}
