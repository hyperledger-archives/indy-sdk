using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Text;
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

            // 1. Create ledger config from genesis txn file
            await PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(PoolUtils.DEFAULT_POOL_NAME, "{}");

            // 2. Create and Open My Wallet
            await WalletUtils.CreateWalleatAsync(PoolUtils.DEFAULT_POOL_NAME, myWalletName, "default", null, null);
            var myWallet = await Wallet.OpenWalletAsync(myWalletName, null, null);

            // 3. Create and Open Trustee Wallet
            await WalletUtils.CreateWalleatAsync(PoolUtils.DEFAULT_POOL_NAME, theirWalletName, "default", null, null);
            var trusteeWallet = await Wallet.OpenWalletAsync(theirWalletName, null, null);

            // 4. Create My Did
            var createMyDidResult = await Signus.CreateAndStoreMyDidAsync(myWallet, "{}");
            var myDid = createMyDidResult.Did;
            var myVerkey = createMyDidResult.VerKey;

            // 5. Create Did from Trustee1 seed
            var theirDidJson = string.Format("{{\"seed\":\"{0}\"}}", trusteeSeed);

            var createTheirDidResult = await Signus.CreateAndStoreMyDidAsync(trusteeWallet, theirDidJson);
            var trusteeDid = createTheirDidResult.Did;

            // 6. Build Nym Request
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);

            // 7. Trustee Sign Nym Request
            var nymResponseJson = await Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest);

            var nymResponse = JObject.Parse(nymResponseJson);

            Debug.Assert(string.Equals(myDid, nymResponse["result"].Value<string>("dest")));
            Debug.Assert(string.Equals(myVerkey, nymResponse["result"].Value<string>("verkey")));

            // 8. Close and delete My Wallet
            await myWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(myWalletName, null);

            // 9. Close and delete Their Wallet
            await trusteeWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(theirWalletName, null);

            // 10. Close Pool
            await pool.CloseAsync();

            // 11. Delete Pool ledger config
            await Pool.DeletePoolLedgerConfigAsync(PoolUtils.DEFAULT_POOL_NAME);

            Console.WriteLine("Ledger sample -> completed");
        }
    }
}
