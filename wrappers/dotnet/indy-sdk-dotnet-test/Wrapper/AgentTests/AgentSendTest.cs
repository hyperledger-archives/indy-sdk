using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.Agent;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentSendTest : AgentIntegrationTest
    {
        private static TaskCompletionSource<Connection> serverToClientConnectionFuture = new TaskCompletionSource<Connection>();
        private static TaskCompletionSource<string> serverToClientMsgFuture = new TaskCompletionSource<string>();
        private static TaskCompletionSource<string> clientToServerMsgFuture = new TaskCompletionSource<string>();

        private class ListenerMessageObserver : MessageObserver
        {
            public void OnMessage(Agent.Connection connection, string message)
            {
                Console.WriteLine("Received message '" + message + "' on connection " + connection);

                serverToClientMsgFuture.SetResult(message);
            }
        }

        private class ConnectionMessageObserver : MessageObserver
        {
            public void OnMessage(Agent.Connection connection, string message)
            {
                Console.WriteLine("Received message '" + message + "' on incoming connection " + connection);

                clientToServerMsgFuture.SetResult(message);
            }
        }

        private class ListenerConnectionObserver : ConnectionObserver
        {
            public MessageObserver OnConnection(Agent.Listener listener, Agent.Connection connection, string senderDid, string receiverDid)
            {
                Console.WriteLine("New connection " + connection);

                serverToClientConnectionFuture.SetResult(connection);

                return _messageObserverForIncoming;
            }
        }

        static AgentSendTest()
        {
            _messageObserver = new ListenerMessageObserver();
            _messageObserverForIncoming = new ConnectionMessageObserver();
            _incomingConnectionObserver = new ListenerConnectionObserver();
        }

        [TestMethod]
        public void TestAgentSendWorks()
        {
            var endpoint = "127.0.0.1:9909";

            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            Signus.StoreTheirDidAsync(_wallet, identityJson).Wait();

            var activeListener = Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Result;

            activeListener.AddIdentityAsync(_pool, _wallet, myDidResult.Did).Wait();

            var clientToServerConnection = Agent.AgentConnectAsync(_pool, _wallet, myDidResult.Did, myDidResult.Did, _messageObserver).Result;

            var clientToServerMessage = "msg_from_client";
            var serverToClientMessage = "msg_from_server";

            clientToServerConnection.SendAsync(clientToServerMessage).Wait();

            Assert.AreEqual(clientToServerMessage, clientToServerMsgFuture.Task.Result);

            var serverToClientConnection = serverToClientConnectionFuture.Task.Result;
            serverToClientConnection.SendAsync(serverToClientMessage).Wait();

            Assert.AreEqual(serverToClientMessage, serverToClientMsgFuture.Task.Result);
        }
    }
}
