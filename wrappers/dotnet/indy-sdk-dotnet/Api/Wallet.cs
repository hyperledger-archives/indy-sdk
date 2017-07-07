using Indy.Sdk.Dotnet.Wrapper;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Api
{
    public class Wallet
    {
        internal Wrapper.Wallet WalletWrapper{ get; }

        private Wallet(Wrapper.Wallet walletWrapper)
        {
            WalletWrapper = walletWrapper;
        }

        public static Task CreateAsync(string poolName, string name, string type, string config, string credentials)
        {
            return Wrapper.Wallet.CreateWalletAsync(poolName, name, type, config, credentials);
        }

        public static Task DeleteAsync(string name, string credentials)
        {
            return Wrapper.Wallet.DeleteWalletAsync(name, credentials);
        }

        public static async Task<Wallet> OpenAsync(string name, string runtimeConfig, string credentials)
        {
            var walletHandle = await Wrapper.Wallet.OpenWalletAsync(name, runtimeConfig, credentials);
            return new Wallet(walletHandle);
        }

        public Task CloseAsync()
        {
            return this.WalletWrapper.CloseAsync();
        }

        public Task SetSeqNoForValueAsync(string walletKey)
        {
            return this.WalletWrapper.SetSeqNoForValueAsync(walletKey);
        }

        public Task<CreateAndStoreMyDidResult> CreateAndStoreMyDidAsync(string didJson)
        {
            return Signus.CreateAndStoreMyDidAsync((Wrapper.Wallet)WalletWrapper, didJson);
        }

        public Task<string> SignAsync(string did, string msg)
        {
            return Signus.SignAsync((Wrapper.Wallet)WalletWrapper, did, msg);
        }

    }
}
