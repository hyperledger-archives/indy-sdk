﻿using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentCloseListenerTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentCloseConnectionWorksForOutgoing()
        {
            var endpoint = "127.0.0.1:9604";

            var myDid = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDid.Did, myDid.Pk, myDid.VerKey, endpoint);

            await  Signus.StoreTheirDidAsync(_wallet, identityJson);

            var activeListener = await Agent.AgentListenAsync(endpoint);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDid.Did);

            await Agent.AgentConnectAsync(_pool, _wallet, myDid.Did, myDid.Did);

            var connectionEvent = await activeListener.WaitForConnection();
            var serverToClientConnection = connectionEvent.Connection;

            await activeListener.CloseAsync();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                serverToClientConnection.SendAsync("msg")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}
