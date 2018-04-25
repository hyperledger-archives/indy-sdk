using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json.Linq;
using System;
using System.Diagnostics;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Samples
{
    static class LedgerDemo
    {
        public static async Task Execute()
        {
            Console.WriteLine("Ledger sample -> started");

            var myWalletName = "myWallet";
            var theirWalletName = "theirWallet";
            var trusteeSeed = "000000000000000000000000Trustee1";

            try
            {
                // 1. Create ledger config from genesis txn file
                await PoolUtils.CreatePoolLedgerConfig();

                // 2. Create and Open My Wallet
                await WalletUtils.CreateWalletAsync(PoolUtils.DEFAULT_POOL_NAME, myWalletName, "default", null, null);

                // 3. Create and Open Trustee Wallet
                await WalletUtils.CreateWalletAsync(PoolUtils.DEFAULT_POOL_NAME, theirWalletName, "default", null, null);

                //4. Open pool and wallets in using statements to ensure they are closed when finished.
                using (var pool = await Pool.OpenPoolLedgerAsync(PoolUtils.DEFAULT_POOL_NAME, "{}"))
                using (var myWallet = await Wallet.OpenWalletAsync(myWalletName, null, null))
                using (var trusteeWallet = await Wallet.OpenWalletAsync(theirWalletName, null, null))
                {
                    //5. Create My Did
                    var createMyDidResult = await Did.CreateAndStoreMyDidAsync(myWallet, "{}");
                    var myDid = createMyDidResult.Did;
                    var myVerkey = createMyDidResult.VerKey;

                    //6. Create Did from Trustee1 seed
                    var theirDidJson = string.Format("{{\"seed\":\"{0}\"}}", trusteeSeed);

                    var createTheirDidResult = await Did.CreateAndStoreMyDidAsync(trusteeWallet, theirDidJson);
                    var trusteeDid = createTheirDidResult.Did;

                    //7. Build Nym Request
                    var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);

                    //8. Trustee Sign Nym Request
                    var nymResponseJson = await Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest);

                    var nymResponse = JObject.Parse(nymResponseJson);

                    Debug.Assert(string.Equals(myDid, nymResponse["result"].Value<string>("dest")));
                    Debug.Assert(string.Equals(myVerkey, nymResponse["result"].Value<string>("verkey")));

                    //9. Close wallets and pool
                    await myWallet.CloseAsync();
                    await trusteeWallet.CloseAsync();
                    await pool.CloseAsync();
                }
            }
            finally
            {
                //10. Delete wallets and Pool ledger config
                await WalletUtils.DeleteWalletAsync(myWalletName, null);
                await WalletUtils.DeleteWalletAsync(theirWalletName, null);
                await PoolUtils.DeletePoolLedgerConfigAsync(PoolUtils.DEFAULT_POOL_NAME);
            }

            Console.WriteLine("Ledger sample -> completed");
        }
    }
}
