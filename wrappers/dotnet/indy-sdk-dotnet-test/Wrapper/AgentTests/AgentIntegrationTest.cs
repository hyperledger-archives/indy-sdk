using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    public abstract class AgentIntegrationTest
    {
        protected static Wallet _wallet;
        protected static Pool _pool;
        protected string _poolName;
        private string _walletName = "agentWallet";

        [TestInitialize]
        public void SetUp()
        {
            InitHelper.Init();
            StorageUtils.CleanupStorage();

            _poolName = PoolUtils.CreatePoolLedgerConfig();

            var config2 = "{}";
            _pool = Pool.OpenPoolLedgerAsync(_poolName, config2).Result;

            Wallet.CreateWalletAsync(_poolName, _walletName, "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync(_walletName, null, null).Result;
        }

        [TestCleanup]
        public void TearDown()
        {
            _wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(_walletName, null).Wait();

            _pool.CloseAsync().Wait();
            StorageUtils.CleanupStorage();
        }

        public static MessageObserver _messageObserver = new ConnectionMessageObserver();
        protected static MessageObserver _messageObserverForIncoming = new ListenerMessageObserver();
        public static ConnectionObserver _incomingConnectionObserver = new ListenerConnectionObserver();


        private class ListenerMessageObserver : MessageObserver
        {
            public void OnMessage(Agent.Connection connection, string message)
            {
                Console.WriteLine("Received message '" + message + "' on connection " + connection);
            }
        }

        private class ConnectionMessageObserver : MessageObserver
        {
            public void OnMessage(Agent.Connection connection, string message)
            {
                Console.WriteLine("Received message '" + message + "' on incoming connection " + connection);
            }
        }

        private class ListenerConnectionObserver : ConnectionObserver
        {
            public MessageObserver OnConnection(Agent.Listener listener, Agent.Connection connection, string senderDid, string receiverDid)
            {
                Console.WriteLine("New connection " + connection);

                return _messageObserverForIncoming;
            }
        }
    }
}
