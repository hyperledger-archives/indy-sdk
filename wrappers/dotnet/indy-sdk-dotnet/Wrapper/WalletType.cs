using System;
using System.Runtime.InteropServices;

namespace Indy.Sdk.Dotnet.Wrapper
{

    /// <summary>
    /// Base type for implementing custom wallet types.
    /// </summary>
    public abstract class WalletType
    {
        /// <summary>
        /// Gets a wallet by its handle.
        /// </summary>
        /// <param name="handle">The handle of the wallet.</param>
        /// <returns>Thw wallet instance associated with the handle.</returns>
        protected abstract CustomWalletBase GetWalletByHandle(int handle);

        internal ErrorCode CreateCallback(string name, string config, string credentials)
        {
            try
            {
                return Create(name, config, credentials);

            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }            
        }

        internal ErrorCode OpenCallback(string name, string config, string runtimeConfig, string credentials, ref int walletHandle)
        {
            try
            {
                return Open(name, config, runtimeConfig, credentials, out walletHandle);
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        internal ErrorCode SetCallback(int walletHandle, string key, string value)
        {
            try
            {
                var wallet = GetWalletByHandle(walletHandle);
                return wallet.Set(key, value);                
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        internal ErrorCode GetCallback(int walletHandle, string key, ref IntPtr valuePtr)
        {
            try
            {
                var wallet = GetWalletByHandle(walletHandle);

                string value;
                var result = wallet.Get(key, out value);

                if (result != ErrorCode.Success)
                    return result;
                
                valuePtr = Marshal.StringToHGlobalAnsi(value);
                wallet.ValuePointers.Add(valuePtr);

                return ErrorCode.Success;
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        internal ErrorCode GetNotExpiredCallback(int walletHandle, string key, ref IntPtr valuePtr)
        {
            try
            {
                var wallet = GetWalletByHandle(walletHandle);

                string value;
                var result = wallet.GetNotExpired(key, out value);

                if (result != ErrorCode.Success)
                    return result;

                valuePtr = Marshal.StringToHGlobalAnsi(value);
                wallet.ValuePointers.Add(valuePtr);

                return ErrorCode.Success;
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        internal ErrorCode ListCallback(int walletHandle, string keyPrefix, ref IntPtr valuesJsonPtr)
        {
            try
            {
                var wallet = GetWalletByHandle(walletHandle);

                string value;
                var result = wallet.List(keyPrefix, out value);

                if (result != ErrorCode.Success)
                    return result;

                valuesJsonPtr = Marshal.StringToHGlobalAnsi(value);
                wallet.ValuePointers.Add(valuesJsonPtr);

                return ErrorCode.Success;
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        internal ErrorCode CloseCallback(int walletHandle)
        {
            try
            {
                return Close(walletHandle);                
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        internal ErrorCode DeleteCallback(string name, string config, string credentials)
        {
            try
            {
                return Delete(name, config, credentials);
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        internal ErrorCode FreeCallback(int walletHandle, IntPtr valuePtr)
        {
            try
            {
                var wallet = GetWalletByHandle(walletHandle);

                Marshal.FreeHGlobal(valuePtr);                
                wallet.ValuePointers.Remove(valuePtr);
                return ErrorCode.Success;
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }

        }

        /// <summary>
        /// Creates a new wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to create.</param>
        /// <param name="config">The configuration for the wallet.</param>
        /// <param name="credentials">The credentials for the wallet.</param>
        public abstract ErrorCode Create(string name, string config, string credentials);

        /// <summary>
        /// Opens a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to open.</param>
        /// <param name="config">The configuration for the wallet.</param>
        /// <param name="runtimeConfig">The runtime configuration for the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        /// <param name="walletHandle">The handle assigned to the wallet.</param>        
        public abstract ErrorCode Open(string name, string config, string runtimeConfig, string credentials, out int walletHandle);

        /// <summary>
        /// Closes the wallet.
        /// </summary>
        public abstract ErrorCode Close(int walletHandle);

        /// <summary>
        /// Deletes a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet being deleted</param>
        /// <param name="config">The configuration of the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        public abstract ErrorCode Delete(string name, string config, string credentials);
    }
}
