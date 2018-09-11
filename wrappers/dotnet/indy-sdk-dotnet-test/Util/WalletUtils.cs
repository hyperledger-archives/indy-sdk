using System;
using System.Threading.Tasks;
using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json;


namespace Hyperledger.Indy.Test.Util
{
    public static class WalletUtils
    {
    
        public static string GetCreateWalletConfig(string walletName)
        {
            WalletConfig config = new WalletConfig() { id = walletName };

            return JsonConvert.SerializeObject(config, Formatting.Indented);
        }

        public static string GetOpenWalletCredentials(string key)
        {
            Credentials cred = new Credentials() { key = key };
            return JsonConvert.SerializeObject(cred, Formatting.Indented);
        }

        public static async Task CreateDefaultWallet(string pool, string wallet)
        {
            string config = WalletUtils.GetCreateWalletConfig(wallet);
            await Wallet.CreateWalletAsync(pool, config);
        }

        public static async Task OpenWallet(string wallet, string key)
        {
            string config = WalletUtils.GetCreateWalletConfig(wallet);
            string cred = WalletUtils.GetOpenWalletCredentials(key);

            await Wallet.OpenWalletAsync(config, cred);
        }

        public static async Task CreateAndOpenWallet(string pool, string wallet, string key)
        {
            string config = WalletUtils.GetCreateWalletConfig(wallet);
            string cred = WalletUtils.GetOpenWalletCredentials(key);

            Task result = WalletUtils.CreateDefaultWallet(pool, wallet);
            await Wallet.OpenWalletAsync(config, cred);
        }
    }
}

