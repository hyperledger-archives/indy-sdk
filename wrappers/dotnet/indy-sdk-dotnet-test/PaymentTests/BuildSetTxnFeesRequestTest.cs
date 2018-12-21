using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class BuildSetTxnFeesRequestTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildSetTxnFeesRequestWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.BuildSetTxnFeesRequestAsync(wallet, DID_TRUSTEE, paymentMethod, fees)
             );
        }

        [TestMethod]
        public async Task testBuildSetTxnFeesRequestWorksForInvalidFees()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                 Payments.BuildSetTxnFeesRequestAsync(wallet, DID_TRUSTEE, paymentMethod, "[txnType1:1, txnType2:2]")
             );
        }
    }
}
