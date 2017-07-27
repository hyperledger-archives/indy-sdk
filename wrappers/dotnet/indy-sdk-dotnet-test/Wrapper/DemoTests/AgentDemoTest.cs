using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;
using System;

namespace Indy.Sdk.Dotnet.Test.Wrapper.DemoTests
{
    [TestClass]
    public class AgentDemoTest : IndyIntegrationTestBase
    {
        private static MessageObserver _messageObserver = new MyMessageObserver();
        private static MessageObserver _messageObserverForIncoming = new MyIncomingMessageObserver();
        private static ConnectionObserver _incomingConnectionObserver = new MyIncomingConnectionObserver();
        private static TaskCompletionSource<string> _clientToServerMsgFuture = new TaskCompletionSource<string>();

        [TestMethod]
        public void TestAgentDemo()
        {
            var endpoint = "127.0.0.1:9801";
            var listenerWalletName = "listenerWallet";
            var trusteeWalletName = "trusteeWallet";
            var message = "test";

            //1. Create and Open Pool
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = Pool.OpenPoolLedgerAsync(poolName, "{}").Result;

            //2. Create and Open Listener Wallet
            Wallet.CreateWalletAsync(poolName, listenerWalletName, "default", null, null).Wait();
            var listenerWallet = Wallet.OpenWalletAsync(listenerWalletName, null, null).Result;

            //3. Create and Open Trustee Wallet
            Wallet.CreateWalletAsync(poolName, trusteeWalletName, "default", null, null).Wait();
            var trusteeWallet = Wallet.OpenWalletAsync(trusteeWalletName, null, null).Result;
            var senderWallet = trusteeWallet;

            //4. Create My Did
            var createMyDidResult = Signus.CreateAndStoreMyDidAsync(listenerWallet, "{}").Result;
            var listenerDid = createMyDidResult.Did;
            var listenerVerkey = createMyDidResult.VerKey;
            var listenerPk = createMyDidResult.Pk;

            //5. Create Their Did from Trustee seed
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(trusteeWallet, trusteeDidJson).Result;
            var trusteeDid = trusteeDidResult.Did;
            var senderDid = trusteeDid;

            // 6. Prepare and Send NYM request with signing
            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, listenerDid, listenerVerkey, null, null).Result;
            Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest).Wait();

            // 7. Prepare and Send Attrib for listener (will be requested from ledger and used by sender at start connection)
            var rawJson = string.Format("{{\"endpoint\":{{\"ha\":\"{0}\",\"verkey\":\"{1}\"}}}}", endpoint, listenerPk);
            var attribRequest = Ledger.BuildAttribRequestAsync(listenerDid, listenerDid, null, rawJson, null).Result;
            Ledger.SignAndSubmitRequestAsync(pool, listenerWallet, listenerDid, attribRequest).Wait();

            // 8. start listener on endpoint
            var activeListener = Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Result;

            // 9. Allow listener accept incoming connection for specific DID (listener_did)
            activeListener.AddIdentityAsync(pool, listenerWallet, listenerDid).Wait();

            // 10. Initiate connection from sender to listener
            var connection = Agent.AgentConnectAsync(pool, senderWallet, senderDid, listenerDid, _messageObserver).Result;

            // 11. Send test message from sender to listener
            connection.SendAsync("test").Wait();

            Assert.AreEqual(message, _clientToServerMsgFuture.Task.Result);

            // 12. Close connection
            connection.CloseAsync().Wait();

            // 13. Close listener
            activeListener.CloseAsync().Wait();

            // 14. Close and delete Issuer Wallet
            listenerWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(listenerWalletName, null).Wait();

            // 15. Close and delete Prover Wallet
            trusteeWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(trusteeWalletName, null).Wait();

            //16. Close Pool
            pool.CloseAsync().Wait();
        }

        //Implentations of observers.
        private class MyMessageObserver : MessageObserver
        {
            public void OnMessage(Agent.Connection connection, string message)
            {
                Console.WriteLine("Received message '" + message + "' on connection " + connection);
            }
        }

        private class MyIncomingMessageObserver : MessageObserver
        {
            public void OnMessage(Agent.Connection connection, string message)
            {
                Console.WriteLine("Received message '" + message + "' on incoming connection " + connection);

                _clientToServerMsgFuture.SetResult(message);
            }
        }

        private class MyIncomingConnectionObserver : ConnectionObserver
        {
            public MessageObserver OnConnection(Agent.Listener listener, Agent.Connection connection, string senderDid, string receiverDid)
            {
                Console.WriteLine("New connection " + connection);

                return _messageObserverForIncoming;
            }
        }
    }
}
