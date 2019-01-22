using System;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Threading.Tasks;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json;
using Console = Colorful.Console;

namespace Hyperledger.Indy.Samples
{
    public class WalletDemo
    {
        public static async Task Execute()
        {
            Console.Write("Executing wallet sample... ");

            var firstWalletConfig = "{\"id\":\"my_wallet\"}";
            var secondWalletConfig = "{\"id\":\"their_wallet\"}";

            var firstWalletCredentials = "{\"key\":\"my_wallet_key\"}";
            var secondWalletCredentials = "{\"key\":\"their_wallet_key\"}";

            try
            {
                // Create and Open First Wallet
                await WalletUtils.CreateWalletAsync(firstWalletConfig, firstWalletCredentials);

                using (var firstWallet = await Wallet.OpenWalletAsync(firstWalletConfig, firstWalletCredentials))
                {
                    // Create a DID that we will retrieve and compare from imported wallet
                    var myDid = await Did.CreateAndStoreMyDidAsync(firstWallet, "{}");

                    var path = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString());
                    var exportConfig = JsonConvert.SerializeObject(new
                    {
                        path = path,
                        key = Guid.NewGuid().ToString()
                    });

                    await firstWallet.ExportAsync(exportConfig);

                    // Import the exported wallet into a new wallet
                    await Wallet.ImportAsync(secondWalletConfig, secondWalletCredentials, exportConfig);

                    // Open the second wallet
                    using (var secondWallet = await Wallet.OpenWalletAsync(secondWalletConfig, secondWalletCredentials))
                    {
                        // Retrieve stored key
                        var myKey = await Did.KeyForLocalDidAsync(secondWallet, myDid.Did);

                        // Compare the two keys
                        Debug.Assert(myKey == myDid.VerKey);

                        await secondWallet.CloseAsync();
                    }

                    // Close wallets 
                    await firstWallet.CloseAsync();
                    File.Delete(path);

                    Console.WriteLine("OK", Color.Green);
                }
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
