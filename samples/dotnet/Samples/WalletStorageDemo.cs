using System;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.Samples.WalletStorage;
using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json;
using Console = Colorful.Console;

namespace Hyperledger.Indy.Samples
{
    public class WalletStorageDemo
    {
        public static async Task Execute()
        {
            Console.Write("Executing wallet storage sample... ");

            var firstWalletConfig = "{\"id\":\"my_wallet\",\"storage_type\":\"inmem\"}";
            var secondWalletConfig = "{\"id\":\"their_wallet\",\"storage_type\":\"inmem\"}";

            var firstWalletCredentials = "{\"key\":\"my_wallet_key\"}";
            var secondWalletCredentials = "{\"key\":\"their_wallet_key\"}";

            try
            {
                await Wallet.RegisterWalletStorageAsync("inmem", new InMemoryWalletStorage());

                // Create and Open First Wallet
                await WalletUtils.CreateWalletAsync(firstWalletConfig, firstWalletCredentials);
                await WalletUtils.CreateWalletAsync(secondWalletConfig, secondWalletCredentials);

                using (var firstWallet = await Wallet.OpenWalletAsync(firstWalletConfig, firstWalletCredentials))
                using (var secondWallet = await Wallet.OpenWalletAsync(secondWalletConfig, secondWalletCredentials))
                {
                    // Create a DID that we will retrieve and compare from imported wallet
                    var myDid = await Did.CreateAndStoreMyDidAsync(firstWallet, "{}");
                    var theirDid = await Did.CreateAndStoreMyDidAsync(secondWallet, "{}");

                    var message = Encoding.UTF8.GetBytes("Hello Indy!");

                    var encrypted = await Crypto.AuthCryptAsync(firstWallet, myDid.VerKey, theirDid.VerKey, message);

                    var decrypted = await Crypto.AuthDecryptAsync(secondWallet, theirDid.VerKey, encrypted);

                    Debug.Assert(message.SequenceEqual(decrypted.MessageData));
                    Debug.Assert(myDid.VerKey.Equals(decrypted.TheirVk));

                    // Close wallets 
                    await firstWallet.CloseAsync();
                    await secondWallet.CloseAsync();
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
                await WalletUtils.DeleteWalletAsync(firstWalletConfig, firstWalletCredentials);
                await WalletUtils.DeleteWalletAsync(secondWalletConfig, secondWalletCredentials);
            }
        }
    }
}
