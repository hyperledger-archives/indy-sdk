using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using System;
using System.Diagnostics;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Samples
{
    static class AgentDemo
    {
        public static async Task Execute()
        {
            Console.WriteLine("Agent sample -> started");

            var listenerWalletName = "listenerWallet";
            var trusteeWalletName = "trusteeWallet";
            var endpoint = "127.0.0.1:9801";
            var message = "test";
            var trusteeSeed = "000000000000000000000000Trustee1";

            try
            {
                //1. Create Pool
                await PoolUtils.CreatePoolLedgerConfig();

                //2. Create Listener Wallet
                await WalletUtils.CreateWalleatAsync(PoolUtils.DEFAULT_POOL_NAME, listenerWalletName, "default", null, null);

                //3. Create Trustee Wallet
                await WalletUtils.CreateWalleatAsync(PoolUtils.DEFAULT_POOL_NAME, trusteeWalletName, "default", null, null);

                //4. Open pool and wallets in using statements to ensure they are closed when finished.
                using (var pool = await Pool.OpenPoolLedgerAsync(PoolUtils.DEFAULT_POOL_NAME, "{}"))
                using (var listenerWallet = await Wallet.OpenWalletAsync(listenerWalletName, null, null))
                using (var trusteeWallet = await Wallet.OpenWalletAsync(trusteeWalletName, null, null))
                {
                    var senderWallet = trusteeWallet;

                    //5. Create My Did
                    var createMyDidResult = await Signus.CreateAndStoreMyDidAsync(listenerWallet, "{}");
                    var listenerDid = createMyDidResult.Did;
                    var listenerVerkey = createMyDidResult.VerKey;
                    var listenerPk = createMyDidResult.Pk;

                    //6. Create Their Did from Trustee seed
                    var trusteeDidJson = string.Format("{{\"seed\":\"{0}\"}}", trusteeSeed);

                    var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(trusteeWallet, trusteeDidJson);
                    var trusteeDid = trusteeDidResult.Did;
                    var senderDid = trusteeDid;

                    //7. Prepare and Send NYM request with signing
                    var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, listenerDid, listenerVerkey, null, null);
                    await Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest);

                    //8. Prepare and Send Attrib for listener (will be requested from ledger and used by sender at start connection)
                    var rawAttribJson = string.Format("{{\"endpoint\":{{\"ha\":\"{0}\",\"verkey\":\"{1}\"}}}}", endpoint, listenerPk);
                    var attribRequest = await Ledger.BuildAttribRequestAsync(listenerDid, listenerDid, null, rawAttribJson, null);
                    await Ledger.SignAndSubmitRequestAsync(pool, listenerWallet, listenerDid, attribRequest);

                    //9. start listener on endpoint
                    var activeListener = await AgentListener.ListenAsync(endpoint);

                    //10. Allow listener accept incoming connection for specific DID (listener_did)
                    await activeListener.AddIdentityAsync(pool, listenerWallet, listenerDid);

                    //11. Initiate connection from sender to listener
                    var sendingConnection = await AgentConnection.ConnectAsync(pool, senderWallet, senderDid, listenerDid);

                    var connectionEvent = await activeListener.WaitForConnectionAsync();
                    var receivingConnection = connectionEvent.Connection;

                    //12. Send test message from sender to listener
                    await sendingConnection.SendAsync(message);

                    var messageEvent = await receivingConnection.WaitForMessageAsync();
                    Debug.Assert(string.Equals(message, messageEvent.Message));

                    //13. Close connection
                    await sendingConnection.CloseAsync();

                    //14. Close listener
                    await activeListener.CloseAsync();

                    //15. Close wallets and pool
                    await listenerWallet.CloseAsync(); 
                    await trusteeWallet.CloseAsync(); 
                    await pool.CloseAsync();
                }                
            }
            finally
            {
                // 16. Delete Pool ledger config and wallets
                await PoolUtils.DeletePoolLedgerConfigAsync(PoolUtils.DEFAULT_POOL_NAME);
                await WalletUtils.DeleteWalletAsync(listenerWalletName, null);
                await WalletUtils.DeleteWalletAsync(trusteeWalletName, null);
            }

            Console.WriteLine("Agent sample -> completed");
        }
    }
}
