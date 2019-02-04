
// these are packages from .NET Core and other Nuget packages
using System;
using Newtonsoft.Json.Linq;

/*
Example demonstrating how to add DID with the role of Trust Anchor as Steward.

Uses seed to obtain Steward's DID which already exists on the ledger.
Then it generates new DID/Verkey pair for Trust Anchor.
Using Steward's DID, NYM transaction request is built to add Trust Anchor's DID and Verkey
on the ledger with the role of Trust Anchor.
Once the NYM is successfully written on the ledger, it generates new DID/Verkey pair that represents
a client, which are used to create GET_NYM request to query the ledger and confirm Trust Anchor's Verkey.

For the sake of simplicity, a single wallet is used. In the real world scenario, three different wallets
would be used and DIDs would be exchanged using some channel of communication
*/

// These packages are from the Hyperledger Nuget package
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.WalletApi;

public class WriteDIDAndQueryVerkey
{
    public static void Demo()
    {

        Console.WriteLine("Step 1 -- set up some constants");

        string walletName = "myWallet";
        string poolName = "pool";
        string stewardSeed = "000000000000000000000000Steward1";
        string poolConfig = "{\"genesis_txn\": \"/home/vagrant/code/evernym/indy-sdk/cli/docker_pool_transactions_genesis\"}";


        // Step 2
        // Tell SDK which pool you are going to use. You should have already started
        // this pool using docker compose or similar.
        Console.WriteLine("Step 2 -- Creating a new local pool ledger configuration that can be used later to connect pool nodes.");
        Pool.CreatePoolLedgerConfigAsync(poolName, poolConfig);

        Console.WriteLine("          Open pool ledger and get the pool handle from libindy.");
        Pool pool = Pool.OpenPoolLedgerAsync(poolName, poolConfig).Result;

        Console.WriteLine("          Creates a new identity wallet.");
        Wallet.CreateWalletAsync(poolName, walletName, "default", string.Empty, string.Empty);

        Console.WriteLine("          Open identity wallet and get the wallet handle from libindy.");
        Wallet wallet = Wallet.OpenWalletAsync(walletName, string.Empty, string.Empty).Result;


        // Step 3
        // First, put a steward DID and its keypair in the wallet. This doesn't write anything to the ledger,
        // but it gives us a key that we can use to sign a ledger transaction that we're going to submit later.
        Console.WriteLine("Step 3 -- Generating and storing steward DID and Verkey.");

        // The DID and public verkey for this steward key are already in the ledger; they were part of the genesis
        // transactions we told the SDK to start with in the previous step. But we have to also put the DID, verkey,
        // and private signing key into our wallet, so we can use the signing key to submit an acceptably signed
        // transaction to the ledger, creating our *next* DID (which is truly new). This is why we use a hard-coded seed
        // when creating this DID--it guarantees that the same DID and key material are created that the genesis txns
        // expect.
        string didJson = "{\"seed\": \"" + stewardSeed + "\"}";
        CreateAndStoreMyDidResult stewardResult = Did.CreateAndStoreMyDidAsync(wallet, didJson).Result;
        string defaultStewardDid = stewardResult.Did;
        Console.WriteLine("          Steward DID    : {0}", defaultStewardDid);
        Console.WriteLine("          Steward VerKey : {0}", stewardResult.VerKey);

        // Now, create a new DID and verkey for a trust anchor, and store it in our wallet as well.Don't use a seed;
        // this DID and its keyas are secure and random. Again, we're not writing to the ledger yet.
        Console.WriteLine("          Generating and storing Trust Anchor DID and Verkey");
        CreateAndStoreMyDidResult trustAnchorResult = Did.CreateAndStoreMyDidAsync(wallet, "{}").Result;
        string trustAnchorDID = trustAnchorResult.Did;
        string trustAnchorVerkey = trustAnchorResult.VerKey;
        Console.WriteLine("          Trust Anchor DID    : {0}", trustAnchorDID);
        Console.WriteLine("          Trust Anchor VerKey : {0}", trustAnchorVerkey);



        // Step 4
        // Here, we are building the transaction payload that we'll send to write the Trust Anchor identity to the ledger.
        // We submit this transaction under the authority of the steward DID that the ledger already recognizes.
        // This call will look up the private key of the steward DID in our wallet, and use it to sign the transaction.
        Console.WriteLine("Step 4 -- Build NYM request to add Trust Anchor to the ledger");
        string nymRequest = Ledger.BuildNymRequestAsync(defaultStewardDid, trustAnchorDID, trustAnchorVerkey, null, "TRUST_ANCHOR").Result;
        Console.WriteLine("          Nym Request JSON : {0}", nymRequest);

        // Now that we have the transaction ready, send it. The building and the sending are separate steps because some
        // clients may want to prepare transactions in one piece of code (e.g., that has access to privileged backend systems),
        // and communicate with the ledger in a different piece of code (e.g., that lives outside the safe internal
        // network).
        Console.WriteLine("          Build NYM request to add Trust Anchor to the ledger");
        String nymResponseJson = Ledger.SignAndSubmitRequestAsync(pool, wallet, defaultStewardDid, nymRequest).Result;
        Console.WriteLine("          NYM transaction response : {0}", nymResponseJson);

        // At this point, we have successfully written a new identity to the ledger. Our next step will be to query it.



        // Step 5
        // Here we are creating a third DID. This one is never written to the ledger, but we do have to have it in the
        // wallet, because every request to the ledger has to be signed by some requester. By creating a DID here, we
        // are forcing the wallet to allocate a keypair and identity that we can use to sign the request that's going
        // to read the trust anchor's info from the ledger.
        Console.WriteLine("Step 5 -- Generating and storing DID and Verkey to query the ledger with");
        CreateAndStoreMyDidResult clientResult = Did.CreateAndStoreMyDidAsync(wallet, "{}").Result;
        string clientDID = trustAnchorResult.Did;
        string clientVerkey = trustAnchorResult.VerKey;
        Console.WriteLine("          Client DID    : {0}", clientDID);
        Console.WriteLine("          Client VerKey : {0}", clientVerkey);

        Console.WriteLine("          Building the GET_NYM request to query Trust Anchor's Verkey as the Client");
        string getNymRequest = Ledger.BuildGetNymRequestAsync(clientDID, trustAnchorDID).Result;
        Console.WriteLine("          GET_NYM request json : {0}", getNymRequest);

        Console.WriteLine("          Sending the GET_NYM request to the ledger");
        string getNymResponse = Ledger.SubmitRequestAsync(pool, getNymRequest).Result;
        Console.WriteLine("          GET_NYM result json : {0}", getNymResponse);

        // See whether we received the same info that we wrote the ledger in step 4.
        Console.WriteLine("          Comparing Trust Anchor Verkey as written by Steward and as retrieved in Client's query");
        var getNymResponseObj = JObject.Parse(getNymResponse);
        string getNymVerKey = getNymResponseObj["verKey"].ToString();
        Console.WriteLine("          Written by Steward  : {0}", trustAnchorVerkey);
        Console.WriteLine("          Queried from Ledger : {0}", getNymVerKey);
        Console.WriteLine("          Matching            : {0}", (string.Compare(getNymVerKey, trustAnchorVerkey) == 0, true, false));

        // Do some cleanup.
        Console.WriteLine("          Close and delete wallet");
        wallet.CloseAsync();

        Console.WriteLine("          Close pool");
        pool.CloseAsync();

        Console.WriteLine("          Delete pool ledger config");
        Pool.DeletePoolLedgerConfigAsync(poolName);

    }
}
