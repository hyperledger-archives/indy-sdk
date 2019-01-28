using System;
using System.Diagnostics;
using System.Drawing;
using System.Threading.Tasks;
using Hyperledger.Indy.NonSecretsApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Console = Colorful.Console;

namespace Hyperledger.Indy.Samples
{
    public class NonSecretsDemo
    {
        public static async Task Execute()
        {
            Console.Write("Executing non-secrets sample... ");

            var myWalletConfig = "{\"id\":\"my_wallet\"}";
            var myWalletCredentials = "{\"key\":\"my_wallet_key\"}";

            try
            {
                // Create and Open First Wallet
                await WalletUtils.CreateWalletAsync(myWalletConfig, myWalletCredentials);

                using (var myWallet = await Wallet.OpenWalletAsync(myWalletConfig, myWalletCredentials))
                {
                    var id = "myRecordId";
                    var value = "myRecordValue";
                    var type = "record_type";
                    var tagsJson = JsonConvert.SerializeObject(new { tagName = "tagValue", tagName2 = "tagValue2" });
                    var queryJson = JsonConvert.SerializeObject(new { tagName = "tagValue" });

                    // Add a new record to the wallet
                    await NonSecrets.AddRecordAsync(myWallet, type, id, value, tagsJson);

                    // Retrieve the record by type and id
                    var recordJson = await NonSecrets.GetRecordAsync(myWallet, type, id, "{}");
                    var record = JObject.Parse(recordJson);

                    Debug.Assert(record["id"].ToObject<string>() == id);
                    Debug.Assert(record["value"].ToObject<string>() == value);

                    // Open wallet search inside using statement to properly dispose and close the search handle
                    using (var walletSearch = await NonSecrets.OpenSearchAsync(myWallet, type, queryJson, "{}"))
                    {
                        // Invoke fetch next records
                        var searchJson = await walletSearch.NextAsync(myWallet, 5);
                        var search = JObject.Parse(searchJson);

                        // There should be one record returned
                        Debug.Assert(search["records"].ToObject<JObject[]>().Length == 1);
                    }

                    // Close wallets 
                    await myWallet.CloseAsync();
                }

                Console.WriteLine("OK", Color.Green);
            }
            catch (Exception e)
            {
                Console.WriteLine($"Error: {e.Message}", Color.Red);
            }
            finally
            {
                // Delete wallets
                await WalletUtils.DeleteWalletAsync(myWalletConfig, myWalletCredentials);
            }
        }
    }
}
