using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DemoTests
{
    [TestClass]
    public class AgentDemoTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentDemo()
        {
            var endpoint = "127.0.0.1:9801";
            var listenerWalletName = "listenerWallet";
            var trusteeWalletName = "trusteeWallet";
            var message = "test";

            //1. Create and Open Pool
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, "{}");

            //2. Create and Open Listener Wallet
            await Wallet.CreateWalletAsync(poolName, listenerWalletName, TYPE, null, null);
            var listenerWallet = await Wallet.OpenWalletAsync(listenerWalletName, null, null);

            //3. Create and Open Trustee Wallet
            await Wallet.CreateWalletAsync(poolName, trusteeWalletName, TYPE, null, null);
            var trusteeWallet = await Wallet.OpenWalletAsync(trusteeWalletName, null, null);
            var senderWallet = trusteeWallet;

            //4. Create My Did
            var createMyDidResult = await Signus.CreateAndStoreMyDidAsync(listenerWallet, "{}");
            var listenerDid = createMyDidResult.Did;
            var listenerVerkey = createMyDidResult.VerKey;
            var listenerPk = createMyDidResult.Pk;

            //5. Create Their Did from Trustee seed
            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(trusteeWallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;
            var senderDid = trusteeDid;

            // 6. Prepare and Send NYM request with signing
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, listenerDid, listenerVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest);

            // 7. Prepare and Send Attrib for listener (will be requested from ledger and used by sender at start connection)
            var rawJson = string.Format("{{\"endpoint\":{{\"ha\":\"{0}\",\"verkey\":\"{1}\"}}}}", endpoint, listenerPk);
            var attribRequest = await Ledger.BuildAttribRequestAsync(listenerDid, listenerDid, null, rawJson, null);
            await Ledger.SignAndSubmitRequestAsync(pool, listenerWallet, listenerDid, attribRequest);

            // 8. start listener on endpoint
            var activeListener = await AgentListener.ListenAsync(endpoint);

            // 9. Allow listener accept incoming connection for specific DID (listener_did)
            await activeListener.AddIdentityAsync(pool, listenerWallet, listenerDid);

            // 10. Initiate connection from sender to listener
            var connection = await AgentConnection.ConnectAsync(pool, senderWallet, senderDid, listenerDid);

            // 11. Send test message from sender to listener
            await connection.SendAsync("test");

            var serverConnectionEvent = await activeListener.WaitForConnectionAsync();
            var serverConnection = serverConnectionEvent.Connection;
            var messageEvent = await serverConnection.WaitForMessageAsync();

            Assert.AreEqual(message, messageEvent.Message);

            // 12. Close connection
            await connection.CloseAsync();

            // 13. Close listener
            await activeListener.CloseAsync();

            // 14. Close and delete Issuer Wallet
            await listenerWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(listenerWalletName, null);

            // 15. Close and delete Prover Wallet
            await trusteeWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(trusteeWalletName, null);

            //16. Close Pool
            await pool.CloseAsync();
        }        
    }
}
