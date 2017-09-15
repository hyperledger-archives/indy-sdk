using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.LedgerApi;
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
    static class AgentDemo
    {
        public static async Task Demo()
        {
            Console.WriteLine("Agent sample -> started");

            var listenerWalletName = "listenerWallet";
            var trusteeWalletName = "trusteeWallet";
            var endpoint = "127.0.0.1:9801";
            var message = "test";
            var trusteeSeed = "000000000000000000000000Trustee1";

            //1. Create and Open Pool
            await PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(PoolUtils.DEFAULT_POOL_NAME, "{}");

            //2. Create and Open Listener Wallet
            await WalletUtils.CreateWalleatAsync(PoolUtils.DEFAULT_POOL_NAME, listenerWalletName, "default", null, null);
            var listenerWallet = await Wallet.OpenWalletAsync(listenerWalletName, null, null);

            //3. Create and Open Trustee Wallet
            await WalletUtils.CreateWalleatAsync(PoolUtils.DEFAULT_POOL_NAME, trusteeWalletName, "default", null, null);
            var trusteeWallet = await Wallet.OpenWalletAsync(trusteeWalletName, null, null);
            var senderWallet = trusteeWallet;

            //4. Create My Did
            var createMyDidResult = await Signus.CreateAndStoreMyDidAsync(listenerWallet, "{}");
            var listenerDid = createMyDidResult.Did;
            var listenerVerkey = createMyDidResult.VerKey;
            var listenerPk = createMyDidResult.Pk;

            //5. Create Their Did from Trustee seed
            var trusteeDidJson = string.Format("{{\"seed\":\"{0}\"}}", trusteeSeed);

            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(trusteeWallet, trusteeDidJson);
            var trusteeDid = trusteeDidResult.Did;
            var senderDid = trusteeDid;

            // 6. Prepare and Send NYM request with signing
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, listenerDid, listenerVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest);

            // 7. Prepare and Send Attrib for listener (will be requested from ledger and used by sender at start connection)
            var rawAttribJson = string.Format("{{\"endpoint\":{{\"ha\":\"{0}\",\"verkey\":\"{1}\"}}}}", endpoint, listenerPk);
            var attribRequest = await Ledger.BuildAttribRequestAsync(listenerDid, listenerDid, null, rawAttribJson, null);
            await Ledger.SignAndSubmitRequestAsync(pool, listenerWallet, listenerDid, attribRequest);

            // 8. start listener on endpoint
            var activeListener = await AgentListener.ListenAsync(endpoint);

            // 9. Allow listener accept incoming connection for specific DID (listener_did)
            await activeListener.AddIdentityAsync(pool, listenerWallet, listenerDid);

            // 10. Initiate connection from sender to listener
            var connection = await AgentConnection.ConnectAsync(pool, senderWallet, senderDid, listenerDid);

            var connectionEvent = await activeListener.WaitForConnectionAsync();
            var listenerConnection = connectionEvent.Connection;

            // 11. Send test message from sender to listener
            await connection.SendAsync(message);

            var listenerReceivedMessageEvent = await listenerConnection.WaitForMessageAsync();
            Debug.Assert(string.Equals(message, listenerReceivedMessageEvent.Message));

            // 12. Close connection
            await connection.CloseAsync();

            // 13. Close listener
            await activeListener.CloseAsync();

            // 14. Close and delete Listener Wallet
            await listenerWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(listenerWalletName, null);

            // 15. Close and delete Sender Wallet
            await trusteeWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(trusteeWalletName, null);

            // 16. Close Pool
            await pool.CloseAsync();

            // 17. Delete Pool ledger config
            await Pool.DeletePoolLedgerConfigAsync(PoolUtils.DEFAULT_POOL_NAME);

            Console.WriteLine("Agent sample -> completed");
        }
    }
}
