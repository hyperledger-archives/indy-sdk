using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class BuildVerifyPaymentRequestTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildVerifyPaymentRequestWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.BuildVerifyPaymentRequestAsync(wallet, DID_TRUSTEE, receipt)
             );
        }
    }
}
