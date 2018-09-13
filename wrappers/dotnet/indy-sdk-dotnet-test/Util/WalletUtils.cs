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

        public static async Task CreateWallet(string walletName, string key)
        {
            string config = WalletUtils.GetCreateWalletConfig(walletName);
            string cred = WalletUtils.GetOpenWalletCredentials(key);
            await Wallet.CreateWalletAsync(config, cred);
        }

        public static async Task<Wallet> OpenWallet(string wallet, string key)
        {
            string config = WalletUtils.GetCreateWalletConfig(wallet);
            string cred = WalletUtils.GetOpenWalletCredentials(key);

            return await Wallet.OpenWalletAsync(config, cred);
        }

        public static async Task<Wallet> CreateAndOpenWallet(string pool, string wallet, string key)
        {
            await WalletUtils.CreateWallet(pool, wallet);

            return await OpenWallet(wallet, key);
        }

        public static async Task DeleteWallet(string walletName, string key) 
        {
            string config = WalletUtils.GetCreateWalletConfig(walletName);
            string cred = WalletUtils.GetOpenWalletCredentials(key);
            await Wallet.DeleteWalletAsync(config, cred);
        }
    }
}

