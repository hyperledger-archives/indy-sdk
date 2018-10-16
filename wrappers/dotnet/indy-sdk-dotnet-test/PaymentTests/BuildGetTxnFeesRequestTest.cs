using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class BuildGetTxnFeesRequestTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildGetTxnFeesRequestWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.BuildGetTxnFeesRequestAsync(wallet, DID_TRUSTEE, paymentMethod)
             );
        }
    }
}
