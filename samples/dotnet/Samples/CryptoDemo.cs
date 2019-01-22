using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Diagnostics;
using System.Drawing;
using System.Text;
using System.Threading.Tasks;
using Console = Colorful.Console;

namespace Hyperledger.Indy.Samples
{
    static class CryptoDemo
    {
        public static async Task Execute()
        {
            Console.Write("Executing crypto sample... ");

            var myWalletConfig = "{\"id\":\"my_wallet\"}";
            var theirWalletConfig = "{\"id\":\"their_wallet\"}";

            var myWalletCredentials = "{\"key\":\"my_wallet_key\"}";
            var theirWalletCredentials = "{\"key\":\"their_wallet_key\"}";

            try
            {
                //1. Create and Open Pool
                await PoolUtils.CreatePoolLedgerConfig();

                //2. Create and Open My Wallet
                await WalletUtils.CreateWalletAsync(myWalletConfig, myWalletCredentials);

                // 3. Create and Open Trustee Wallet
                await WalletUtils.CreateWalletAsync(theirWalletConfig, theirWalletCredentials);

                //4. Open pool and wallets in using statements to ensure they are closed when finished.
                using (var myWallet = await Wallet.OpenWalletAsync(myWalletConfig, myWalletCredentials))
                using (var theirWallet = await Wallet.OpenWalletAsync(theirWalletConfig, theirWalletCredentials))
                {
                    //5. Create My Did
                    var createMyDidResult = await Did.CreateAndStoreMyDidAsync(myWallet, "{}");

                    //6. Create Their Did
                    var createTheirDidResult = await Did.CreateAndStoreMyDidAsync(theirWallet, "{}");
                    var theirDid = createTheirDidResult.Did;
                    var theirVerkey = createTheirDidResult.VerKey;

                    //7. Store Their DID
                    var identityJson = string.Format("{{\"did\":\"{0}\", \"verkey\":\"{1}\"}}", theirDid, theirVerkey);
                    await Did.StoreTheirDidAsync(myWallet, identityJson);

                    //8. Their sign message
                    var msgBytes = Encoding.UTF8.GetBytes("{\n" +
                            "   \"reqId\":1495034346617224651,\n" +
                            "   \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
                            "   \"operation\":{\n" +
                            "       \"type\":\"1\",\n" +
                            "       \"dest\":\"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB\"\n" +
                            "   }\n" +
                            "}");

                    var signatureBytes = await Crypto.SignAsync(theirWallet, theirVerkey, msgBytes);

                    //9. Verify message
                    var valid = await Crypto.VerifyAsync(theirVerkey, msgBytes, signatureBytes);
                    Debug.Assert(valid == true);

                    //10. Close wallets and pool
                    await myWallet.CloseAsync();
                    await theirWallet.CloseAsync();
                }

                Console.WriteLine("OK", Color.Green);
            }
            catch (Exception e)
            {
                Console.WriteLine($"Error: {e.Message}", Color.Red);
            }
            finally
            {
                // 11. Delete wallets and Pool ledger config
                await WalletUtils.DeleteWalletAsync(myWalletConfig, myWalletCredentials);
                await WalletUtils.DeleteWalletAsync(theirWalletConfig, theirWalletCredentials);
                await PoolUtils.DeletePoolLedgerConfigAsync(PoolUtils.DEFAULT_POOL_NAME);
            }
        }
    }
}
