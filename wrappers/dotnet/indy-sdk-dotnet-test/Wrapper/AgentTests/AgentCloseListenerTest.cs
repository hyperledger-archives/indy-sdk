using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.Agent;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentCloseListenerTest : AgentIntegrationTest
    {

        private static TaskCompletionSource<Connection> _serverToClientConnectionTaskCompletionSource = new TaskCompletionSource<Connection>();


        public class ListenerConnectionObserver : ConnectionObserver
        {
            public MessageObserver OnConnection(Listener listener, Connection connection, string senderDid, string receiverDid)
            {
                Console.WriteLine("New connection " + connection);

                _serverToClientConnectionTaskCompletionSource.SetResult(connection);

                return _messageObserverForIncoming;
            }
        }

        static AgentCloseListenerTest()
        {
            _incomingConnectionObserver = new ListenerConnectionObserver();
        }


        [TestMethod]
        public async Task TestAgentCloseConnectionWorksForOutgoing()
        {
            var endpoint = "127.0.0.1:9904";

            var myDid = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDid.Did, myDid.Pk, myDid.VerKey, endpoint);

            Signus.StoreTheirDidAsync(_wallet, identityJson).Wait();

            var activeListener = Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Result;

            activeListener.AddIdentityAsync(_pool, _wallet, myDid.Did).Wait();

            Agent.AgentConnectAsync(_pool, _wallet, myDid.Did, myDid.Did, _messageObserver).Wait();

            var serverToClientConnection = _serverToClientConnectionTaskCompletionSource.Task.Result;
            activeListener.CloseAsync().Wait();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                serverToClientConnection.SendAsync("msg")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}
