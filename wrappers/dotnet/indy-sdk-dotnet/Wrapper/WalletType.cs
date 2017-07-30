using System;
using System.Runtime.InteropServices;
using static Indy.Sdk.Dotnet.IndyNativeMethods;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Base type for implementing custom wallet types.
    /// </summary>
    public abstract class WalletType
    {
        /// <summary>
        /// Initializes a new WalletType.
        /// </summary>
        public WalletType()
        {
            CreateCallback = CreateHandler;
            OpenCallback = OpenHandler;
            SetCallback = SetHandler;
            GetCallback = GetHandler;
            GetNotExpiredCallback = GetNotExpiredHandler;
            ListCallback = ListHandler;
            CloseCallback = CloseHandler;
            DeleteCallback = DeleteHandler;
            FreeCallback = FreeHandler;
        }

        /// <summary>
        /// The delegate to call when a wallet is being created.
        /// </summary>
        internal WalletTypeCreateDelegate CreateCallback { get; }

        /// <summary>
        /// The delegate to call when a wallet is being opened.
        /// </summary>
        internal WalletTypeOpenDelegate OpenCallback { get; }

        /// <summary>
        /// The delegate to call when a value is set  on a wallet.
        /// </summary>
        internal WalletTypeSetDelegate SetCallback { get; }

        /// <summary>
        /// The delegate to call when a value is requested from a wallet.
        /// </summary>
        internal WalletTypeGetDelegate GetCallback { get; }

        /// <summary>
        /// The delegate to call when an unexpired value is requested from a wallet.
        /// </summary>
        internal WalletTypeGetNotExpiredDelegate GetNotExpiredCallback { get; }

        /// <summary>
        /// The delegate to call when a list of values is requested from a wallet.
        /// </summary>
        internal WalletTypeListDelegate ListCallback { get; }

        /// <summary>
        /// The delegate to call when a wallet is being closed.
        /// </summary>
        internal WalletTypeCloseDelegate CloseCallback { get; }

        /// <summary>
        /// The delegate to call when a wallet is being deleted.
        /// </summary>
        internal WalletTypeDeleteDelegate DeleteCallback { get; }

        /// <summary>
        /// The delegate to call when a value returned by a wallet is to be freed.
        /// </summary>
        internal WalletTypeFreeDelegate FreeCallback { get; }

        /// <summary>
        /// Gets a wallet by its handle.
        /// </summary>
        /// <param name="handle">The handle of the wallet.</param>
        /// <returns>Thw wallet instance associated with the handle.</returns>
        protected abstract CustomWalletBase GetWalletByHandle(int handle);

        /// <summary>
        /// Handler for wallet creation.
        /// </summary>
        /// <param name="name">The name of the wallet to create.</param>
        /// <param name="config">The configuration for the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode CreateHandler(string name, string config, string credentials)
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

        /// <summary>
        /// Handler for opening a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to open.</param>
        /// <param name="config">The wallet configuration that was registered on creation.</param>
        /// <param name="runtimeConfig">The runtime configuration to use for the wallet.</param>
        /// <param name="credentials">The wallet credentials.</param>
        /// <param name="walletHandle">A handle returned for the opened wallet instance.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode OpenHandler(string name, string config, string runtimeConfig, string credentials, ref int walletHandle)
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

        /// <summary>
        /// Handler for setting a value on an opened wallet instance.
        /// </summary>
        /// <param name="walletHandle">The handle of the wallet instance.</param>
        /// <param name="key">The key of the value to set.</param>
        /// <param name="value">The value to set.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode SetHandler(int walletHandle, string key, string value)
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

        /// <summary>
        /// Handler for getting a value from an opened wallet instance.
        /// </summary>
        /// <param name="walletHandle">The handle of the wallet instance.</param>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="valuePtr">The returned pointer to the value.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode GetHandler(int walletHandle, string key, ref IntPtr valuePtr)
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

        /// <summary>
        /// Handler for getting an unexpired value from an open wallet instance.
        /// </summary>
        /// <param name="walletHandle">The handle of the wallet instance.</param>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="valuePtr">The returned pointer to the value.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode GetNotExpiredHandler(int walletHandle, string key, ref IntPtr valuePtr)
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

        /// <summary>
        /// Handler for getting a list of values from an open wallet instance.
        /// </summary>
        /// <param name="walletHandle">The handle of the wallet instance.</param>
        /// <param name="keyPrefix">The prefix to filter keys by.</param>
        /// <param name="valuesJsonPtr">The returned pointer to the value.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode ListHandler(int walletHandle, string keyPrefix, ref IntPtr valuesJsonPtr)
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

        /// <summary>
        /// Handler for closing an open wallet instance.
        /// </summary>
        /// <param name="walletHandle">The handle of the wallet instance.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode CloseHandler(int walletHandle)
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

        /// <summary>
        /// Handler for deleting a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to delete.</param>
        /// <param name="config">The configuration of the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode DeleteHandler(string name, string config, string credentials)
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

        /// <summary>
        /// Handler for for freeing a value returned by an open wallet instance.
        /// </summary>
        /// <param name="walletHandle">The handle of the wallet instance.</param>
        /// <param name="valuePtr">The pointer to the value to free.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode FreeHandler(int walletHandle, IntPtr valuePtr)
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
        /// Allows an implementer to create a new wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to create.</param>
        /// <param name="config">The configuration for the wallet.</param>
        /// <param name="credentials">The credentials for the wallet.</param>
        public abstract ErrorCode Create(string name, string config, string credentials);

        /// <summary>
        ///  Allows an implementer to open a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to open.</param>
        /// <param name="config">The configuration for the wallet.</param>
        /// <param name="runtimeConfig">The runtime configuration for the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        /// <param name="walletHandle">The handle assigned to the wallet.</param>        
        public abstract ErrorCode Open(string name, string config, string runtimeConfig, string credentials, out int walletHandle);

        /// <summary>
        ///  Allows an implementer to close a wallet.
        /// </summary>
        public abstract ErrorCode Close(int walletHandle);

        /// <summary>
        ///  Allows an implementer to delete a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet being deleted</param>
        /// <param name="config">The configuration of the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        public abstract ErrorCode Delete(string name, string config, string credentials);
    }
}
