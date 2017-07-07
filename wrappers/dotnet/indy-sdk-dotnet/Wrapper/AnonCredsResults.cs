using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Wrapper
{
    public sealed class IssuerCreateAndStoreClaimDefResult
    {
        public IssuerCreateAndStoreClaimDefResult(string claimDefJson, string claimDefUuid)
        {
            ClaimDefJson = claimDefJson;
            ClaimDefUuid = claimDefUuid;
        }

        public string ClaimDefJson { get; }
        public string ClaimDefUuid { get; }
    }

    public sealed class IssuerCreateAndStoreRevocRegResult
    {
        public IssuerCreateAndStoreRevocRegResult(string claimDefJson, string claimDefUuid)
        {
            ClaimDefJson = claimDefJson;
            ClaimDefUuid = claimDefUuid;
        }

        public string ClaimDefJson { get; }
        public string ClaimDefUuid { get; }
    }

    public sealed class IssuerCreateClaimResult
    {
        public IssuerCreateClaimResult(string revocRegUpdateJson, string xClaimJson)
        {
            RevocRegUpdateJson = revocRegUpdateJson;
            XClaimJson = xClaimJson;
        }

        public string RevocRegUpdateJson { get; }
        public string XClaimJson { get; }
    }
}
