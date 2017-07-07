using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Base class for all API wrappers.
    /// </summary>
    public abstract class AsyncWrapperBase
    {
        /// <summary>
        /// The command handle.
        /// </summary>
        private static IntPtr _nextCommandHandle = IntPtr.Zero;

        protected static Dictionary<IntPtr, object> TaskCompletionSources { get; }
        protected static ResultOnlyDelegate ResultOnlyCallback { get; }
        protected static ResultWithHandleDelegate ResultWithHandleCallback { get; }

        static AsyncWrapperBase()
        {
            TaskCompletionSources = new Dictionary<IntPtr, object>();

            ResultOnlyCallback = (xCommandHandle, err) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<bool>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(true);
            };

            ResultWithHandleCallback = (xCommandHandle, err, handle) => 
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<IntPtr>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(handle);
            };
        }
        
        protected static TaskCompletionSource<T> CreateTaskCompletionSourceForCommand<T>(IntPtr commandHandle)
        {
            var taskCompletionSource = new TaskCompletionSource<T>();
            TaskCompletionSources.Add(commandHandle, taskCompletionSource);
            return taskCompletionSource;
        }

        /// <summary>
        /// Checks the result from a Sovrin function call.
        /// </summary>
        /// <exception cref="SovrinException">If the result is not a success result a SovrinException will be thrown.</exception>
        /// <param name="result">The result to check.</param>
        protected static void CheckResult(int result)
        {
            if (result != (int)ErrorCode.Success)
                throw SovrinException.fromErrorCode(result);
        }

        /// <summary>
        /// Checks the result of a callback made by the Sovrin library.
        /// </summary>
        /// <typeparam name="T">The type the promise will return.</typeparam>
        /// <param name="taskCompletionSource">The source controlling the async result.</param>
        /// <param name="xCommandHandle">The command handle of the command that was processed.</param>
        /// <param name="errorCode">The error code returned to the callback by the sovrin function.</param>
        /// <returns>true if the error code was success, otherwise false.</returns>
        /// <exception cref="SovrinException">If the errorCode is not a success result a SovrinException will be thrown.</exception>
        protected static bool CheckCallback<T>(TaskCompletionSource<T> taskCompletionSource, IntPtr xCommandHandle, int errorCode)
        {
            if (errorCode != (int)ErrorCode.Success)
            {
                taskCompletionSource.SetException(SovrinException.fromErrorCode(errorCode));
                return false;
            }

            return true;
        }

        protected static IntPtr GetNextCommandHandle()
        {
            _nextCommandHandle = IntPtr.Add(_nextCommandHandle, 1);
            return _nextCommandHandle;
        }

        public static TaskCompletionSource<T> GetTaskCompletionSourceForCommand<T>(IntPtr xCommandHandle)
        {
            if (!TaskCompletionSources.ContainsKey(xCommandHandle))
                throw new ApplicationException(string.Format("No task completion source is currently registered for the command with the handle '{0}'.", xCommandHandle));

            var taskCompletionSource = TaskCompletionSources[xCommandHandle];
            TaskCompletionSources.Remove(xCommandHandle);

            if (taskCompletionSource == null)
                throw new ApplicationException(string.Format("No  task completion source is registered for the command with the handle '{0}'.", xCommandHandle));

            var result = taskCompletionSource as TaskCompletionSource<T>;

            if (result == null)
                throw new ApplicationException(string.Format("The  task completion source registered for the command with the handle '{0}' does not match the type specified.", xCommandHandle));  

            return result;
        }
    }
}
