using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Samples
{
    static class SignusDemo
    {
        public static async Task Execute()
        {
            Console.WriteLine("Ledger sample -> started");

            var myWalletName = "myWallet";
            var theirWalletName = "theirWallet";

            //1. Create and Open Pool
            await PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(PoolUtils.DEFAULT_POOL_NAME, "{}");

            //2. Create and Open My Wallet
            await WalletUtils.CreateWalleatAsync(PoolUtils.DEFAULT_POOL_NAME, myWalletName, "default", null, null);
            var myWallet = await Wallet.OpenWalletAsync(myWalletName, null, null);

            // 3. Create and Open Trustee Wallet
            await WalletUtils.CreateWalleatAsync(PoolUtils.DEFAULT_POOL_NAME, theirWalletName, "default", null, null);
            var theirWallet = await Wallet.OpenWalletAsync(theirWalletName, null, null);

            //4. Create My Did
            var createMyDidResult = await Signus.CreateAndStoreMyDidAsync(myWallet, "{}");

            //5. Create Their Did
            var createTheirDidResult = await Signus.CreateAndStoreMyDidAsync(theirWallet, "{}");
            var theirDid = createTheirDidResult.Did;
            var theirVerkey = createTheirDidResult.VerKey;

            // 6. Store Their DID
            var identityJson = string.Format("{{\"did\":\"{0}\", \"verkey\":\"{1}\"}}", theirDid, theirVerkey);
            await Signus.StoreTheirDidAsync(myWallet, identityJson);

            // 7. Their sign message
            var msgBytes = Encoding.UTF8.GetBytes("{\n" +
                    "   \"reqId\":1495034346617224651,\n" +
                    "   \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
                    "   \"operation\":{\n" +
                    "       \"type\":\"1\",\n" +
                    "       \"dest\":\"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB\"\n" +
                    "   }\n" +
                    "}");

            var signatureBytes = await Signus.SignAsync(theirWallet, theirDid, msgBytes);

            // 8. Verify message
            Boolean valid = await Signus.VerifySignatureAsync(myWallet, pool, theirDid, msgBytes, signatureBytes);
            Debug.Assert(valid == true);

            // 9. Close and delete My Wallet
            await myWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(myWalletName, null);

            // 10. Close and delete Their Wallet
            await theirWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(theirWalletName, null);

            //11. Close Pool
            await pool.CloseAsync();

            // 12. Delete Pool ledger config
            await Pool.DeletePoolLedgerConfigAsync(PoolUtils.DEFAULT_POOL_NAME);

            Console.WriteLine("Ledger sample -> completed");
        }
    }
}
