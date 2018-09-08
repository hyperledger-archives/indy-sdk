using System.Threading.Tasks;

#if __IOS__
using ObjCRuntime;
#endif

namespace Hyperledger.Indy.Utils
{
    internal static class CallbackHelper
    {
        /// <summary>
        /// Delegate for callbacks that only include the success or failure of command execution.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        internal delegate void IndyMethodCompletedDelegate(int xcommand_handle, int err);

        /// <summary>
        /// Gets the callback to use for completing tasks that don't return a value.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(IndyMethodCompletedDelegate))]
#endif
        public static void TaskCompletingNoValueCallbackMethod(int xcommand_handle, int err)
        {
            var taskCompletionSource = PendingCommands.Remove<bool>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(true);
        }
        public static IndyMethodCompletedDelegate TaskCompletingNoValueCallback = TaskCompletingNoValueCallbackMethod;

        /// <summary>
        /// Gets the callback to use for functions that don't return a value and are not associated with a task.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(IndyMethodCompletedDelegate))]
#endif
        public static void NoValueCallbackMethod(int xcommand_handle, int err)
        {
            CheckCallback(err);
        }
        public static IndyMethodCompletedDelegate NoValueCallback = NoValueCallbackMethod;

        /// <summary>
        /// Checks the result from a Sovrin function call.
        /// </summary>
        /// <exception cref="IndyException">If the result is not a success result a SovrinException will be thrown.</exception>
        /// <param name="result">The result to check.</param>
        public static void CheckResult(int result)
        {
            if (result != (int)ErrorCode.Success)
                throw IndyException.FromSdkError(result);
        }

        /// <summary>
        /// Checks the result of a callback made by the Sovrin library.
        /// </summary>
        /// <typeparam name="T">The type the promise will return.</typeparam>
        /// <param name="taskCompletionSource">The source controlling the async result.</param>
        /// <param name="errorCode">The error code returned to the callback by the indy function.</param>
        /// <returns>true if the error code was success, otherwise false.</returns>
        /// <exception cref="IndyException">If the errorCode is not a success result a SovrinException will be thrown.</exception>
        public static bool CheckCallback<T>(TaskCompletionSource<T> taskCompletionSource, int errorCode)
        {
            if (errorCode != (int)ErrorCode.Success)
            {
                taskCompletionSource.SetException(IndyException.FromSdkError(errorCode));
                return false;
            }

            return true;
        }

        /// <summary>
        /// Checks the result of a callback made by the Sovrin library.
        /// </summary>
        /// <param name="errorCode">The error code returned to the callback by the indy function.</param>
        /// <exception cref="IndyException">If the errorCode is not a success result a SovrinException will be thrown.</exception>
        public static void CheckCallback(int errorCode)
        {
            if (errorCode != (int)ErrorCode.Success)
                throw IndyException.FromSdkError(errorCode);
        }
    }
}
