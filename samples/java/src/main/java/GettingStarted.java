/*
 * #%L
 * Wildfly Camel :: Testsuite
 * %%
 * Copyright (C) 2013 - 2014 RedHat
 * %%
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * #L%
 */


import static org.hyperledger.indy.sdk.IndyConstants.ROLE_ENDORSER;
import static org.hyperledger.indy.sdk.IndyConstants.ROLE_TRUSTEE;
import static utils.PoolUtils.PROTOCOL_VERSION;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import org.hyperledger.indy.sdk.anoncreds.Anoncreds;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreCredentialDefResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreRevocRegResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateCredentialResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateSchemaResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.ProverCreateCredentialRequestResult;
import org.hyperledger.indy.sdk.anoncreds.CredentialsSearchForProofReq;
import org.hyperledger.indy.sdk.blob_storage.BlobStorageReader;
import org.hyperledger.indy.sdk.blob_storage.BlobStorageWriter;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.ledger.LedgerResults.ParseResponseResult;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.json.JSONObject;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import utils.EnvironmentUtils;
import utils.PoolUtils;


/**
 * Start a local indy pool
 * 
 * docker rm -f indy-pool
 * docker run --detach --name=indy-pool -p 9701-9708:9701-9708 indy-pool
 * 
 * Remove dirty client state
 * 
 * rm -rf ~/.indy_client
 */
public class GettingStarted {

	Logger log = LoggerFactory.getLogger(getClass());
	
	static class Context {
		
		// Pool Ledger
		String poolName;
		Pool pool;
		
		// Trustee
		String trusteeWalletConfig;
		String trusteeWalletKey;
		Wallet trusteeWallet;
		String trusteeDid;
		String trusteeVkey;
		
		// Government
		String governmentWalletConfig;
		String governmentWalletKey;
		Wallet governmentWallet;
		String governmentDid;
		String governmentVkey;
		String governmentDidForFaber;
		String governmentVkeyForFaber;
		String governmentDidForAcme;
		String governmentVkeyForAcme;
		String governmentDidForThrift;
		String governmentVkeyForThrift;
		String governmentDidForAlice;
		String governmentVkeyForAlice;
		
		// Faber
		String faberWalletConfig;
		String faberWalletKey;
		Wallet faberWallet;
		String faberDid;
		String faberVkey;
		String faberDidForAlice;
		String faberVkeyForAlice;
		String transcriptSchemaId;
		String transcriptCredDefId;
		
		// Acme
		String acmeWalletConfig;
		String acmeWalletKey;
		Wallet acmeWallet;
		String acmeDid;
		String acmeVkey;
		String acmeDidForAlice;
		String acmeVkeyForAlice;
		String jobCertificateSchemaId;
		String jobCertificateCredDefId;
		String jobCertificateCredOffer;
		String jobCertificateCredRevocId;
		String revocRegistryId;
		
		// Thrift
		String thriftWalletConfig;
		String thriftWalletKey;
		Wallet thriftWallet;
		String thriftDid;
		String thriftVkey;
		String thriftDidForAlice;
		String thriftVkeyForAlice;
		
		// Alice
		String aliceWalletConfig;
		String aliceWalletKey;
		Wallet aliceWallet;
		String aliceDid;
		String aliceVkey;
		String aliceDidForFaber;
		String aliceVkeyForFaber;
		String aliceDidForAcme;
		String aliceVkeyForAcme;
		String aliceDidForThrift;
		String aliceVkeyForThrift;
		String aliceMasterSecretId;
	}

	public static void main(String[] args) throws Exception {
		new GettingStarted().demo(new Context());
	}

	public void demo(Context ctx) throws Exception {
		
		/* Setup Indy Pool Nodes
		 * 
		 * The ledger is intended to store Identity Records that describe a Ledger Entity.
		 * 
		 * Identity Records are public data and may include Public Keys, Service Endpoints, Credential Schemas, and Credential Definitions.
		 * 
		 * Every Identity Record is associated with exactly one DID (Decentralized Identifier) that is globally unique and resolvable (via a ledger) 
		 * without requiring any centralized resolution authority. 
		 * 
		 * To maintain privacy each Identity Owner can own multiple DIDs.
		 */
		
		createAndOpenPoolLedger(ctx);
		
		/* Creating Trustee Wallet and DID
		 * 
		 * Trustees operate nodes. Trustees govern the network. These are the highest privileged DIDs. 
		 * Endorsers are able to write Schemas and Cred_Defs to the ledger, or sign such transactions so they can be written by non-privileged DIDs.
		 * 
		 * We want to ensure a DID has the least amount of privilege it needs to operate, which in many cases is no privilege, provided the resources 
		 * it needs are already written to the ledger, either by a privileged DID or by having the txn signed by a privileged DID (e.g. by an Endorser).
		 * 
		 * An Endorser is a person or organization that the ledger already knows about, that is able to help bootstrap others.
		 */
		
		createTrustee(ctx);
		
		/* Onboarding Government, Faber, Acme, Thrift
		 * 
		 * Each connection is actually a pair of Pairwise-Unique Identifiers (DIDs).
		 * The one DID is owned by one party to the connection and the second by another.
		 * 
		 * Both parties know both DIDs and understand what connection this pair describes.
		 * 
		 * Publishing with a DID verification key allows a person, organization or thing, to verify that someone owns this DID as that person,
		 * organization or thing is the only one who knows the corresponding signing key and any DID-related operations requiring signing with this key.
		 * 
		 * The relationship between them is not shareable with others; 
		 * it is unique to those two parties in that each pairwise relationship uses different DIDs.
		 * 
		 * We call the process of establish a connection Onboarding.
		 */
		
		onboardGovernment(ctx);
		onboardFaberCollege(ctx);
		onboardAcmeCorp(ctx);
		onboardThriftBank(ctx);
		onboardAlice(ctx);
		
		/* Creating Credential Schemas
		 * 
		 * Credential Schema is the base semantic structure that describes the list of attributes which one particular Credential can contain.
		 * 
		 * It’s not possible to update an existing Schema. 
		 * If the Schema needs to be evolved, a new Schema with a new version or name needs to be created.
		 * 
		 * Schemas in indy are very simple JSON documents that specify their name and version, and that list attributes that will appear in a credential. 
		 * Currently, they do not describe data type, recurrence rules, nesting, and other elaborate constructs.
		 */
		
		createTranscriptSchema(ctx);
		createJobCertificateSchema(ctx);
		
		/* Creating Credential Definitions
		 * 
		 * Credential Definition is similar in that the keys that the Issuer uses for the signing of Credentials also satisfies a specific Credential Schema.
		 * 
		 * It references it's associated schema, announces who is going to be issuing credentials with that schema, what type of signature method they plan to use 
		 * (“CL” = “Camenisch Lysyanskya”, the default method used for zero-knowledge proofs by indy), how they plan to handle revocation, and so forth.
		 * 
		 * It’s not possible to update data in an existing Credential Definition. If a CredDef needs to be evolved (for example, a key needs to be rotated), 
		 * then a new Credential Definition needs to be created by a new Issuer DID.
		 * 
		 * A Credential Definition can be created and saved in the Ledger an Endorser. 
		 */
		
		createTranscriptCredentialDefinition(ctx);
		createJobCertificateCredentialDefinition(ctx);
		
		/* Alice gets her Transcript from Faber College
		 * 
		 * A credential is a piece of information about an identity — a name, an age, a credit score...
		 * It is information claimed to be true. In this case, the credential is named, “Transcript”.
		 * 
		 * Credentials are offered by an issuer.
		 * 
		 * An issuer may be any identity owner known to the Ledger and any issuer may issue a credential about any identity owner it can identify.
		 * 
		 * The usefulness and reliability of a credential are tied to the reputation of the issuer with respect to the credential at hand. 
		 * For Alice to self-issue a credential that she likes chocolate ice cream may be perfectly reasonable, but for her to self-issue 
		 * a credential that she graduated from Faber College should not impress anyone.
		 */
		
		getTranscriptFromFaber(ctx);
		
		/* Alice applies for a job at Acme
		 * 
		 * At some time in the future, Alice would like to work for Acme Corp. Normally she would browse to their website, where she would click on a hyperlink to apply for a job. 
		 * Her browser would download a connection request in which her Indy app would open; this would trigger a prompt to Alice, asking her to accept the connection with Acme Corp. 
		 *
		 * After Alice had established connection with Acme, she got the Job-Application Proof Request. 
		 * A proof request is a request made by the party who needs verifiable proof of having certain attributes and the solving of predicates that can be provided by other verified credentials.
		 * 
		 * Acme Corp is requesting that Alice provide a Job Application. 
		 * The Job Application requires a name, degree, status, SSN and also the satisfaction of the condition about the average mark or grades.
		 */
		
		applyForJobWithAcme(ctx);
		
		/* Alice applies for a loan with Thrift Bank
		 * 
		 * Now that Alice has a job, she’d like to apply for a loan. That will require a proof of employment. 
		 * She can get this from the Job-Certificate credential offered by Acme.
		 */
		
		applyForLoanWithThrift(ctx);
		
		/* Thrift accepts the loan application and now requires KYC
		 * 
		 * Thrift Bank sends the second Proof Request where Alice needs to share her personal information with the bank.
		 */
		
		kycProcessWithThrift(ctx);
		
		/* Alice decides to quit her job with Acme
		 */
		
		quitJobWithAcme(ctx);
		
		// Close and Delete Indy Pool Nodes
		
		closeAndDeletePoolLedger(ctx);
	}
	
	void createAndOpenPoolLedger(Context ctx) throws Exception {
		
		log.info("Create and Open Pool Ledger");
		
		// Set protocol version 2
		Pool.setProtocolVersion(PROTOCOL_VERSION).get();
		
		// Create ledger config from genesis txn file
		
		log.info("Create and open Ledger");
		ctx.poolName = PoolUtils.createPoolLedgerConfig("pool1");
		ctx.pool = Pool.openPoolLedger(ctx.poolName, "{}").get();
	}
	
	void createTrustee(Context ctx) throws Exception {
		
		// Create Wallet for Trustee
		
		log.info("Create wallet - Trustee");
		ctx.trusteeWalletConfig = new JSONObject().put("id", "Trustee").toString();
		ctx.trusteeWalletKey = new JSONObject().put("key", "trustee_wallet_key").toString();
		Wallet.createWallet(ctx.trusteeWalletConfig, ctx.trusteeWalletKey).get();
		ctx.trusteeWallet = Wallet.openWallet(ctx.trusteeWalletConfig, ctx.trusteeWalletKey).get();
		
		// Create Trustee DID
		
		String trusteeSeed = new JSONObject().put("seed", "000000000000000000000000Trustee1").toString();
		CreateAndStoreMyDidResult didResult = Did.createAndStoreMyDid(ctx.trusteeWallet, trusteeSeed).get();
		ctx.trusteeDid = didResult.getDid();
		ctx.trusteeVkey = didResult.getVerkey();
		
		// Store Trustee DID in the Ledger

		log.info("DID Trustee: did={}, vkey={}", ctx.trusteeDid, ctx.trusteeVkey);
		String nymRequest = Ledger.buildNymRequest(ctx.trusteeDid, ctx.trusteeDid, ctx.trusteeVkey, null, ROLE_TRUSTEE).get();
		signAndSubmitRequest(ctx, ctx.trusteeWallet, ctx.trusteeDid, nymRequest);
	}
	
	void onboardGovernment(Context ctx) throws Exception {
		
		// Create Wallet for Government
		
		log.info("Create wallet - Government");
		ctx.governmentWalletConfig = new JSONObject().put("id", "Government").toString();
		ctx.governmentWalletKey = new JSONObject().put("key", "government_wallet_key").toString();
		Wallet.createWallet(ctx.governmentWalletConfig, ctx.governmentWalletKey).get();
		ctx.governmentWallet = Wallet.openWallet(ctx.governmentWalletConfig, ctx.governmentWalletKey).get();
		
		// Create and store Government DID
		
		String governmentSeed = new JSONObject().put("seed", "000000000000000000000Government1").toString();
		CreateAndStoreMyDidResult didResult = Did.createAndStoreMyDid(ctx.governmentWallet, governmentSeed).get();
		ctx.governmentDid = didResult.getDid();
		ctx.governmentVkey = didResult.getVerkey();
		
		// The Trustee onboards Government all others are onboarded by Government, which holds the ENDORSER role 
		
		log.info("DID Government: did={}, vkey={}", ctx.governmentDid, ctx.governmentVkey);
		String nymRequest = Ledger.buildNymRequest(ctx.trusteeDid, ctx.governmentDid, ctx.governmentVkey, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.trusteeWallet, ctx.trusteeDid, nymRequest);
	}
	
	void onboardFaberCollege(Context ctx) throws Exception {
		
		// Create and store the Government DID for Faber
		
		CreateAndStoreMyDidResult didResult = Did.createAndStoreMyDid(ctx.governmentWallet, "{}").get();
		ctx.governmentDidForFaber = didResult.getDid();
		ctx.governmentVkeyForFaber = didResult.getVerkey();
		
		log.info("DID Government for Faber: did={}, vkey={}", ctx.governmentDidForFaber, ctx.governmentVkeyForFaber);
		String nymRequest = Ledger.buildNymRequest(ctx.governmentDid, ctx.governmentDidForFaber, ctx.governmentVkeyForFaber, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.governmentWallet, ctx.governmentDid, nymRequest);

		// Create Wallet for Faber
		
		log.info("Create wallet - Faber");
		ctx.faberWalletConfig = new JSONObject().put("id", "Faber").toString();
		ctx.faberWalletKey = new JSONObject().put("key", "faber_wallet_key").toString();
		Wallet.createWallet(ctx.faberWalletConfig, ctx.faberWalletKey).get();
		ctx.faberWallet = Wallet.openWallet(ctx.faberWalletConfig, ctx.faberWalletKey).get();
		
		// Create and store Faber DID
		
		String faberSeed = new JSONObject().put("seed", "00000000000000000000000000Faber1").toString();
		didResult = Did.createAndStoreMyDid(ctx.faberWallet, faberSeed).get();
		ctx.faberDid = didResult.getDid();
		ctx.faberVkey = didResult.getVerkey();
		
		log.info("DID Faber: did={}, vkey={}", ctx.faberDid, ctx.faberVkey);
		nymRequest = Ledger.buildNymRequest(ctx.governmentDidForFaber, ctx.faberDid, ctx.faberVkey, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.governmentWallet, ctx.governmentDidForFaber, nymRequest);
	}
	
	void onboardAcmeCorp(Context ctx) throws Exception {
		
		// Create and store the Government DID for Faber
		
		CreateAndStoreMyDidResult didResult = Did.createAndStoreMyDid(ctx.governmentWallet, "{}").get();
		ctx.governmentDidForAcme = didResult.getDid();
		ctx.governmentVkeyForAcme = didResult.getVerkey();
		
		log.info("DID Government for Acme: did={}, vkey={}", ctx.governmentDidForAcme, ctx.governmentVkeyForAcme);
		String nymRequest = Ledger.buildNymRequest(ctx.governmentDid, ctx.governmentDidForAcme, ctx.governmentVkeyForAcme, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.governmentWallet, ctx.governmentDid, nymRequest);

		// Create Wallet for Acme
		
		log.info("Create wallet - Acme");
		ctx.acmeWalletConfig = new JSONObject().put("id", "Acme").toString();
		ctx.acmeWalletKey = new JSONObject().put("key", "acme_wallet_key").toString();
		Wallet.createWallet(ctx.acmeWalletConfig, ctx.acmeWalletKey).get();
		ctx.acmeWallet = Wallet.openWallet(ctx.acmeWalletConfig, ctx.acmeWalletKey).get();
		
		// Create and store Acme DID
		
		String acmeSeed = new JSONObject().put("seed", "000000000000000000000000000Acme1").toString();
		didResult = Did.createAndStoreMyDid(ctx.acmeWallet, acmeSeed).get();
		ctx.acmeDid = didResult.getDid();
		ctx.acmeVkey = didResult.getVerkey();
		
		log.info("DID Acme: did={}, vkey={}", ctx.acmeDid, ctx.acmeVkey);
		nymRequest = Ledger.buildNymRequest(ctx.governmentDidForAcme, ctx.acmeDid, ctx.acmeVkey, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.governmentWallet, ctx.governmentDidForAcme, nymRequest);
	}
	
	void onboardThriftBank(Context ctx) throws Exception {
		
		// Create and store Government DID for Thrift
		
		CreateAndStoreMyDidResult didResult = Did.createAndStoreMyDid(ctx.governmentWallet, "{}").get();
		ctx.governmentDidForThrift = didResult.getDid();
		ctx.governmentVkeyForThrift = didResult.getVerkey();
		
		log.info("DID Government for Thrift: did={}, vkey={}", ctx.governmentDidForThrift, ctx.governmentVkeyForThrift);
		String nymRequest = Ledger.buildNymRequest(ctx.governmentDid, ctx.governmentDidForThrift, ctx.governmentVkeyForThrift, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.governmentWallet, ctx.governmentDid, nymRequest);

		// Create Wallet for Thrift
		
		log.info("Create wallet - Thrift");
		ctx.thriftWalletConfig = new JSONObject().put("id", "Thrift").toString();
		ctx.thriftWalletKey = new JSONObject().put("key", "thrift_wallet_key").toString();
		Wallet.createWallet(ctx.thriftWalletConfig, ctx.thriftWalletKey).get();
		ctx.thriftWallet = Wallet.openWallet(ctx.thriftWalletConfig, ctx.thriftWalletKey).get();
		
		// Create and store Thrift DID
		
		String thriftSeed = new JSONObject().put("seed", "0000000000000000000000000Thrift1").toString();
		didResult = Did.createAndStoreMyDid(ctx.thriftWallet, thriftSeed).get();
		ctx.thriftDid = didResult.getDid();
		ctx.thriftVkey = didResult.getVerkey();
		
		log.info("DID Thrift: did={}, vkey={}", ctx.thriftDid, ctx.thriftVkey);
		nymRequest = Ledger.buildNymRequest(ctx.governmentDidForThrift, ctx.thriftDid, ctx.thriftVkey, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.governmentWallet, ctx.governmentDidForThrift, nymRequest);
	}
	
	void onboardAlice(Context ctx) throws Exception {
		
		// Create and store Government DID for Alice
		
		CreateAndStoreMyDidResult didResult = Did.createAndStoreMyDid(ctx.governmentWallet, "{}").get();
		ctx.governmentDidForAlice = didResult.getDid();
		ctx.governmentVkeyForAlice = didResult.getVerkey();
		
		log.info("DID Government for Alice: did={}, vkey={}", ctx.governmentDidForAlice, ctx.governmentVkeyForAlice);
		String nymRequest = Ledger.buildNymRequest(ctx.governmentDid, ctx.governmentDidForAlice, ctx.governmentVkeyForAlice, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.governmentWallet, ctx.governmentDid, nymRequest);

		// Create and store Faber DID for Alice
		
		didResult = Did.createAndStoreMyDid(ctx.faberWallet, "{}").get();
		ctx.faberDidForAlice = didResult.getDid();
		ctx.faberVkeyForAlice = didResult.getVerkey();
		
		log.info("DID Faber for Alice: did={}, vkey={}", ctx.faberDidForAlice, ctx.faberVkeyForAlice);
		nymRequest = Ledger.buildNymRequest(ctx.faberDid, ctx.faberDidForAlice, ctx.faberVkeyForAlice, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.faberWallet, ctx.faberDid, nymRequest);

		// Create and store Acme DID for Alice
		
		didResult = Did.createAndStoreMyDid(ctx.acmeWallet, "{}").get();
		ctx.acmeDidForAlice = didResult.getDid();
		ctx.acmeVkeyForAlice = didResult.getVerkey();
		
		log.info("DID Acme for Alice: did={}, vkey={}", ctx.acmeDidForAlice, ctx.acmeVkeyForAlice);
		nymRequest = Ledger.buildNymRequest(ctx.acmeDid, ctx.acmeDidForAlice, ctx.acmeVkeyForAlice, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.acmeWallet, ctx.acmeDid, nymRequest);

		// Create and store Thrift DID for Alice
		
		didResult = Did.createAndStoreMyDid(ctx.thriftWallet, "{}").get();
		ctx.thriftDidForAlice = didResult.getDid();
		ctx.thriftVkeyForAlice = didResult.getVerkey();
		
		log.info("DID Thrift for Alice: did={}, vkey={}", ctx.thriftDidForAlice, ctx.thriftVkeyForAlice);
		nymRequest = Ledger.buildNymRequest(ctx.thriftDid, ctx.thriftDidForAlice, ctx.thriftVkeyForAlice, null, ROLE_ENDORSER).get();
		signAndSubmitRequest(ctx, ctx.thriftWallet, ctx.thriftDid, nymRequest);

		// Create Wallet for Alice
		
		log.info("Create wallet - Alice");
		ctx.aliceWalletConfig = new JSONObject().put("id", "Alice").toString();
		ctx.aliceWalletKey = new JSONObject().put("key", "alice").toString();
		Wallet.createWallet(ctx.aliceWalletConfig, ctx.aliceWalletKey).get();
		ctx.aliceWallet = Wallet.openWallet(ctx.aliceWalletConfig, ctx.aliceWalletKey).get();
		
		// Create and store Alice DID
		
		String aliceSeed = new JSONObject().put("seed", "00000000000000000000000000Alice1").toString();
		didResult = Did.createAndStoreMyDid(ctx.aliceWallet, aliceSeed).get();
		ctx.aliceDid = didResult.getDid();
		ctx.aliceVkey = didResult.getVerkey();
		
		log.info("DID Alice: did={}, vkey={}", ctx.aliceDid, ctx.aliceVkey);
		nymRequest = Ledger.buildNymRequest(ctx.governmentDidForAlice, ctx.aliceDid, ctx.aliceVkey, null, null).get();
		signAndSubmitRequest(ctx, ctx.governmentWallet, ctx.governmentDidForAlice, nymRequest);

		// Create and store Alice DID for Faber
		
		didResult = Did.createAndStoreMyDid(ctx.aliceWallet, "{}").get();
		ctx.aliceDidForFaber = didResult.getDid();
		ctx.aliceVkeyForFaber = didResult.getVerkey();
		
		log.info("DID Alice for Faber: did={}, vkey={}", ctx.aliceDidForFaber, ctx.aliceVkeyForFaber);
		nymRequest = Ledger.buildNymRequest(ctx.faberDidForAlice, ctx.aliceDidForFaber, ctx.aliceVkeyForFaber, null, null).get();
		signAndSubmitRequest(ctx, ctx.faberWallet, ctx.faberDidForAlice, nymRequest);

		// Create and store Alice DID for Acme
		
		didResult = Did.createAndStoreMyDid(ctx.aliceWallet, "{}").get();
		ctx.aliceDidForAcme = didResult.getDid();
		ctx.aliceVkeyForAcme = didResult.getVerkey();
		
		log.info("DID Alice for Acme: did={}, vkey={}", ctx.aliceDidForAcme, ctx.aliceVkeyForAcme);
		nymRequest = Ledger.buildNymRequest(ctx.acmeDidForAlice, ctx.aliceDidForAcme, ctx.aliceVkeyForAcme, null, null).get();
		signAndSubmitRequest(ctx, ctx.acmeWallet, ctx.acmeDidForAlice, nymRequest);

		// Create and store Alice DID for Thrift
		
		didResult = Did.createAndStoreMyDid(ctx.aliceWallet, "{}").get();
		ctx.aliceDidForThrift = didResult.getDid();
		ctx.aliceVkeyForThrift = didResult.getVerkey();
		
		log.info("DID Alice for Thrift: did={}, vkey={}", ctx.aliceDidForThrift, ctx.aliceVkeyForThrift);
		nymRequest = Ledger.buildNymRequest(ctx.thriftDidForAlice, ctx.aliceDidForThrift, ctx.aliceVkeyForThrift, null, null).get();
		signAndSubmitRequest(ctx, ctx.thriftWallet, ctx.thriftDidForAlice, nymRequest);
	}

	void createTranscriptSchema(Context ctx) throws Exception {
		
		// 1. Government creates the Transcript Credential Schema. It can do so with it's Endorser role
		
		IssuerCreateSchemaResult schemaResult = Anoncreds.issuerCreateSchema(ctx.governmentDid, "Transcript", "1.2", 
				new JSONArray(Arrays.asList("first_name","last_name","degree","status","year","average","ssn")).toString()).get();
		log.info(schemaResult.toString());
		
		// 2. Government sends the corresponding Schema transaction to the Ledger
		
		Ledger.buildSchemaRequest(ctx.governmentDid, schemaResult.getSchemaJson()).get();
		ctx.transcriptSchemaId = schemaResult.getSchemaId();
		
		String schemaRequest = Ledger.buildSchemaRequest(ctx.governmentDid, schemaResult.getSchemaJson()).get();
		signAndSubmitRequest(ctx, ctx.governmentWallet, ctx.governmentDid, schemaRequest);
	}
	
	void createJobCertificateSchema(Context ctx) throws Exception {
		
		// 1. Government creates the Job-Certificate Credential Schema
		
		IssuerCreateSchemaResult schemaResult = Anoncreds.issuerCreateSchema(ctx.governmentDid, "Job-Certificate", "0.2", 
				new JSONArray(Arrays.asList("first_name","last_name","salary","employee_status","experience")).toString()).get();
		log.info(schemaResult.toString());

		// 2. Government sends the corresponding Schema transaction to the Ledger
		
		Ledger.buildSchemaRequest(ctx.governmentDid, schemaResult.getSchemaJson()).get();
		ctx.jobCertificateSchemaId = schemaResult.getSchemaId();
		
		String schemaRequest = Ledger.buildSchemaRequest(ctx.governmentDid, schemaResult.getSchemaJson()).get();
		signAndSubmitRequest(ctx, ctx.governmentWallet, ctx.governmentDid, schemaRequest);
	}
	
	void createTranscriptCredentialDefinition(Context ctx) throws Exception {
		
		// 1. Faber get the Transcript Credential Schema
		
		String getSchemaRequest = Ledger.buildGetSchemaRequest(ctx.faberDid, ctx.transcriptSchemaId).get();
		String getSchemaResponse = Ledger.submitRequest(ctx.pool, getSchemaRequest).get();
		ParseResponseResult parseSchemaResult = Ledger.parseGetSchemaResponse(getSchemaResponse).get();
		log.info(getSchemaResponse);
		
		// 2. Faber creates the Credential Definition related to the received Credential Schema
		
		String configJson = new JSONObject().put("support_revocation", false).toString();
		IssuerCreateAndStoreCredentialDefResult createCredDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(ctx.faberWallet, ctx.faberDid, parseSchemaResult.getObjectJson(), "TAG1", null, configJson).get();
		ctx.transcriptCredDefId = createCredDefResult.getCredDefId();

		// 3. Faber sends the corresponding Credential Definition transaction to the Ledger
		
		String credDefRequest = Ledger.buildCredDefRequest(ctx.faberDid, createCredDefResult.getCredDefJson()).get();
		signAndSubmitRequest(ctx, ctx.faberWallet, ctx.faberDid, credDefRequest);
	}
	
	void createJobCertificateCredentialDefinition(Context ctx) throws Exception {
		
		// 1. Acme get the Transcript Credential Schema

		String getSchemaRequest = Ledger.buildGetSchemaRequest(ctx.acmeDid, ctx.jobCertificateSchemaId).get();
		String getSchemaResponse = Ledger.submitRequest(ctx.pool, getSchemaRequest).get();
		log.info(getSchemaResponse);
		
		// 2. Acme creates the Credential Definition related to the received Credential Schema
		
		String configJson = new JSONObject().put("support_revocation", true).toString();
		ParseResponseResult parseSchemaResult = Ledger.parseGetSchemaResponse(getSchemaResponse).get();
		IssuerCreateAndStoreCredentialDefResult createCredDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(ctx.acmeWallet, ctx.acmeDid, parseSchemaResult.getObjectJson(), "TAG1", null, configJson).get();
		ctx.jobCertificateCredDefId = createCredDefResult.getCredDefId();

		// 3. Acme sends the corresponding Credential Definition transaction to the Ledger
		
		String credDefRequest = Ledger.buildCredDefRequest(ctx.acmeDid, createCredDefResult.getCredDefJson()).get();
		signAndSubmitRequest(ctx, ctx.acmeWallet, ctx.acmeDid, credDefRequest);
		
		/* 4. Acme creates Revocation Registry
		 * 
		 * The issuer anticipates revoking Job-Certificate credentials. It decides to create a revocation registry. 
		 * 
		 * One of Hyperledger Indy’s revocation registry types uses cryptographic accumulators for publishing revoked credentials. 
		 * The use of those accumulators requires the publication of “validity tails” outside of the Ledger.
		 *  
		 * For the purpose of this demo, the validity tails are written in a file using a ‘blob storage’.
		 */
		
		BlobStorageWriter tailsWriter = BlobStorageWriter.openWriter("default", getTailsWriterConfig()).get();

		// 5. Acme creates a Revocation Registry for the given Credential Definition.
		
		String revRegDefTag = "Tag2";
		String revRegDefConfig = new JSONObject().put("issuance_type", "ISSUANCE_ON_DEMAND").put("max_cred_num", 5).toString();
		IssuerCreateAndStoreRevocRegResult createRevRegResult = Anoncreds.issuerCreateAndStoreRevocReg(ctx.acmeWallet, ctx.acmeDid, null, revRegDefTag, ctx.jobCertificateCredDefId, revRegDefConfig, tailsWriter).get();
		String revRegEntryJson = createRevRegResult.getRevRegEntryJson();
		String revRegDefJson = createRevRegResult.getRevRegDefJson();
		ctx.revocRegistryId = createRevRegResult.getRevRegId();
		
		// 6. Acme creates and submits the Revocation Registry Definition
		
		String revRegDefRequest = Ledger.buildRevocRegDefRequest(ctx.acmeDid, revRegDefJson).get();
		String revRegDefResponse = signAndSubmitRequest(ctx, ctx.acmeWallet, ctx.acmeDid, revRegDefRequest);
		log.info(revRegDefResponse);
		
		// 7. Acme creates and submits the Revocation Registry Entry
		
		String revRegEntryRequest = Ledger.buildRevocRegEntryRequest(ctx.acmeDid, ctx.revocRegistryId, "CL_ACCUM", revRegEntryJson).get();
		String revRegEntryResponse = signAndSubmitRequest(ctx, ctx.acmeWallet, ctx.acmeDid, revRegEntryRequest);
		log.info(revRegEntryResponse);
	}
	
	void getTranscriptFromFaber(Context ctx) throws Exception {
		
		/* 1. Faber creates a Credential Offer for Alice
		 *
		 * The value of this Transcript Credential is that it is provably issued by Faber College
		 */
		
		String transcriptCredOffer = Anoncreds.issuerCreateCredentialOffer(ctx.faberWallet, ctx.transcriptCredDefId).get();
		String transcriptCredDefId = new JSONObject(transcriptCredOffer).getString("cred_def_id");
		
		/* 2. Alice gets Credential Definition from Ledger
		 * 
		 * Alice wants to see the attributes that the Transcript Credential contains. 
		 * These attributes are known because a Credential Schema for Transcript has been written to the Ledger.
		 */
		
		String getSchemaRequest = Ledger.buildGetSchemaRequest(ctx.faberDidForAlice, ctx.transcriptSchemaId).get();
		String getSchemaResponse = Ledger.submitRequest(ctx.pool, getSchemaRequest).get();
		ParseResponseResult parseSchemaResult = Ledger.parseGetSchemaResponse(getSchemaResponse).get();
		log.info("Transcript Schema" + parseSchemaResult.getObjectJson());
		
		/* 3. Alice creates a Master Secret
		 * 
		 * A Master Secret is an item of Private Data used by a Prover to guarantee that a credential uniquely applies to them.
		 *  
		 * The Master Secret is an input that combines data from multiple Credentials to prove that the Credentials 
		 * have a common subject (the Prover). A Master Secret should be known only to the Prover.
		 */
		
		ctx.aliceMasterSecretId = Anoncreds.proverCreateMasterSecret(ctx.aliceWallet, null).get();
		
		/* 4. Alice get the Credential Definition
		 * 
		 * Alice also needs to get the Credential Definition corresponding to the Credential Definition Id in the Transcript Credential Offer.
		 */
		
		String credDefResponse = submitRequest(ctx, Ledger.buildGetCredDefRequest(ctx.aliceDid, transcriptCredDefId).get());
		ParseResponseResult parsedCredDefResponse = Ledger.parseGetCredDefResponse(credDefResponse).get();
		String transcriptCredDef = parsedCredDefResponse.getObjectJson();
		
		// 5. Alice creates a Credential Request of the issuance of the Transcript Credential

		ProverCreateCredentialRequestResult credentialRequestResult = Anoncreds.proverCreateCredentialReq(ctx.aliceWallet, ctx.aliceDidForFaber, transcriptCredOffer, transcriptCredDef, ctx.aliceMasterSecretId).get();
		String credentialRequestMetadataJson = credentialRequestResult.getCredentialRequestMetadataJson();
		String credentialRequestJson = credentialRequestResult.getCredentialRequestJson();
		
		/* 6. Faber creates the Transcript Credential for Alice
		 * 
		 * Encoding is not standardized by Indy except that 32-bit integers are encoded as themselves.
		 */
		
		String credValuesJson = new JSONObject()
			.put("first_name", new JSONObject().put("raw", "Alice").put("encoded", "1139481716457488690172217916278103335"))
			.put("last_name", new JSONObject().put("raw", "Garcia").put("encoded", "5321642780241790123587902456789123452"))
			.put("degree", new JSONObject().put("raw", "Bachelor of Science, Marketing").put("encoded", "12434523576212321"))
			.put("status", new JSONObject().put("raw", "graduated").put("encoded", "2213454313412354"))
			.put("ssn", new JSONObject().put("raw", "123-45-6789").put("encoded", "3124141231422543541"))
			.put("year", new JSONObject().put("raw", "2015").put("encoded", "2015"))
			.put("average", new JSONObject().put("raw", "5").put("encoded", "5")).toString();
		
		IssuerCreateCredentialResult issuerCredentialResult = Anoncreds.issuerCreateCredential(ctx.faberWallet, transcriptCredOffer, credentialRequestJson, credValuesJson, null, 0).get();
		String transcriptCredJson = issuerCredentialResult.getCredentialJson();
		log.info("IssuedCredential: " + transcriptCredJson);
		
		// 7. Alice stores Transcript Credential from Faber in her Wallet
		
		String transcriptCredentialId = Anoncreds.proverStoreCredential(ctx.aliceWallet, null, credentialRequestMetadataJson, transcriptCredJson, transcriptCredDef, null).get();
		log.info("Transcript Credential Id: " + transcriptCredentialId);
	}
	
	void applyForJobWithAcme(Context ctx) throws Exception {
		
		/* 1. Acme creates a Job Application Proof Request
		 * 
		 * Notice that some attributes are verifiable and others are not.
		 * 
		 * The proof request says that degree, and graduation status, ssn and year must be formally asserted by an issuer and schema_key. 
		 * Notice also that the first_name, last_name and phone_number are not required to be verifiable. 
		 * 
		 * By not tagging these credentials with a verifiable status, Acme’s credential request is saying it will accept 
		 * Alice’s own credential about her names and phone number.
		 */
		
		String nonce = Anoncreds.generateNonce().get();
		JSONArray transcriptRestrictions = new JSONArray().put(new JSONObject().put("cred_def_id", ctx.transcriptCredDefId));
		
		String proofRequestJson = new JSONObject()
			.put("nonce", nonce)
			.put("name", "Job-Application")
			.put("version", "0.1")
			.put("requested_attributes", new JSONObject()
				.put("attr1_referent", new JSONObject().put("name", "first_name"))
				.put("attr2_referent", new JSONObject().put("name", "last_name"))
				.put("attr3_referent", new JSONObject().put("name", "degree").put("restrictions", transcriptRestrictions))
				.put("attr4_referent", new JSONObject().put("name", "status").put("restrictions", transcriptRestrictions))
				.put("attr5_referent", new JSONObject().put("name", "ssn").put("restrictions", transcriptRestrictions))
				.put("attr6_referent", new JSONObject().put("name", "year").put("restrictions", transcriptRestrictions)))
			.put("requested_predicates", new JSONObject()
				.put("predicate1_referent", new JSONObject()
					.put("name", "average")
					.put("p_type", ">=")
					.put("p_value", 4)
					.put("restrictions", transcriptRestrictions)))
			.toString();
		
		log.info("Job-Application Proof Request: " + proofRequestJson);

		// 2. Alice searches her Wallet for Credentials that she can use for the creating of Proof for the Job-Application Proof Request
		
		CredentialsSearchForProofReq credentialsSearch = CredentialsSearchForProofReq.open(ctx.aliceWallet, proofRequestJson, null).get();
		
		JSONArray credentialsForAttribute3 = new JSONArray(credentialsSearch.fetchNextCredentials("attr3_referent", 100).get());
		String credentialIdForAttribute3 = credentialsForAttribute3.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		JSONArray credentialsForAttribute4 = new JSONArray(credentialsSearch.fetchNextCredentials("attr4_referent", 100).get());
		String credentialIdForAttribute4 = credentialsForAttribute4.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		JSONArray credentialsForAttribute5 = new JSONArray(credentialsSearch.fetchNextCredentials("attr5_referent", 100).get());
		String credentialIdForAttribute5 = credentialsForAttribute5.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		JSONArray credentialsForAttribute6 = new JSONArray(credentialsSearch.fetchNextCredentials("attr6_referent", 100).get());
		String credentialIdForAttribute6 = credentialsForAttribute6.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		JSONArray credentialsForPredicate1 = new JSONArray(credentialsSearch.fetchNextCredentials("predicate1_referent", 100).get());
		String credentialIdForPredicate1 = credentialsForPredicate1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		
		credentialsSearch.close();
		
		/* 3. Alice provides Job Application Proof
		 * 
		 * Alice divides these attributes into the three groups:
		 * 
		 * - attributes values of which will be revealed
		 * - attributes values of which will be unrevealed
		 * - attributes for which creating of verifiable proof is not required
		 */
		
		String credentialsJson = new JSONObject()
			.put("self_attested_attributes", new JSONObject()
					.put("attr1_referent", "Alice")
					.put("attr2_referent", "Garcia"))
			.put("requested_attributes", new JSONObject()
				.put("attr3_referent", new JSONObject()
					.put("cred_id", credentialIdForAttribute3)
					.put("revealed", true))
				.put("attr4_referent", new JSONObject()
					.put("cred_id", credentialIdForAttribute4)
					.put("revealed", true))
				.put("attr5_referent", new JSONObject()
					.put("cred_id", credentialIdForAttribute5)
					.put("revealed", true))
				.put("attr6_referent", new JSONObject()
					.put("cred_id", credentialIdForAttribute6)
					.put("revealed", true)))
			.put("requested_predicates", new JSONObject()
				.put("predicate1_referent", new JSONObject()
					.put("cred_id",credentialIdForPredicate1)))
			.toString();
		
		// 4. Alice gets the Credential Schema and correspoding Credential Definition
		
		JSONObject schemasMap = new JSONObject();
		JSONObject credDefsMap = new JSONObject();
		
		populateCredentialInfo(ctx.pool, ctx.aliceDidForFaber, schemasMap, credDefsMap, credentialsForAttribute3);
		populateCredentialInfo(ctx.pool, ctx.aliceDidForFaber, schemasMap, credDefsMap, credentialsForAttribute4);
		populateCredentialInfo(ctx.pool, ctx.aliceDidForFaber, schemasMap, credDefsMap, credentialsForAttribute5);
		populateCredentialInfo(ctx.pool, ctx.aliceDidForFaber, schemasMap, credDefsMap, credentialsForAttribute6);
		
		String schemas = schemasMap.toString();
		String credDefs = credDefsMap.toString();
		String revocState = new JSONObject().toString();

		// 5. Alice creates the Proof for Acme Job-Application Proof Request
		
		String proofJson = Anoncreds.proverCreateProof(ctx.aliceWallet, proofRequestJson, credentialsJson, ctx.aliceMasterSecretId, schemas, credDefs, revocState).get();
		JSONObject proof = new JSONObject(proofJson);
		log.info("Proof: " + proof);
		
		/* 6. Acme verifies the Job Application Proof from Alice
		 * 
		 * To do it Acme first must get every Credential Schema and corresponding Credential Definition for each identifier presented in the Proof, the same way that Alice did it. 
		 * Now Acme has everything to check Job-Application Proof from Alice.
		 */
		
		JSONObject selfAttestedAttrs = proof.getJSONObject("requested_proof").getJSONObject("self_attested_attrs");
		JSONObject revealedAttrs = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs");
		log.info("SelfAttestedAttrs: " + selfAttestedAttrs);
		log.info("RevealedAttrs: " + revealedAttrs);
		
		assertEquals("Alice", selfAttestedAttrs.getString("attr1_referent"));
		assertEquals("Garcia", selfAttestedAttrs.getString("attr2_referent"));
		assertEquals("Bachelor of Science, Marketing", revealedAttrs.getJSONObject("attr3_referent").getString("raw"));
		assertEquals("graduated", revealedAttrs.getJSONObject("attr4_referent").getString("raw"));
		assertEquals("123-45-6789", revealedAttrs.getJSONObject("attr5_referent").getString("raw"));
		assertEquals("2015", revealedAttrs.getJSONObject("attr6_referent").getString("raw"));
		
		String revocRegDefs = new JSONObject().toString();
		String revocRegs = new JSONObject().toString();
		
		Boolean accepted = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemas, credDefs, revocRegDefs, revocRegs).get();
		assertTrue("Proof not accepted", accepted);

		// 7. Acme creates a Credential Offer for Alice
		
		ctx.jobCertificateCredOffer = Anoncreds.issuerCreateCredentialOffer(ctx.acmeWallet, ctx.jobCertificateCredDefId).get();
	}

	void applyForLoanWithThrift(Context ctx) throws Exception {
		
		/* 1. Alice get the Credential Definition
		 * 
		 * Alice needs to get the Credential Definition corresponding to the Credential Definition Id in the Job-Certificate Credential Offer.
		 */
		
		String credDefResponse = submitRequest(ctx, Ledger.buildGetCredDefRequest(ctx.aliceDid, ctx.jobCertificateCredDefId).get());
		ParseResponseResult parsedCredDefResponse = Ledger.parseGetCredDefResponse(credDefResponse).get();
		String jobCertificateCredDef = parsedCredDefResponse.getObjectJson();
		
		// 2. Alice creates a Credential Request
		
		ProverCreateCredentialRequestResult credentialRequestResult = Anoncreds.proverCreateCredentialReq(ctx.aliceWallet, ctx.aliceDidForAcme, ctx.jobCertificateCredOffer, jobCertificateCredDef, ctx.aliceMasterSecretId).get();
		String credentialRequestMetadataJson = credentialRequestResult.getCredentialRequestMetadataJson();
		String credentialRequestJson = credentialRequestResult.getCredentialRequestJson();
		
		/* 3. Acme issues a Job-Certificate Credential for Alice
		 * 
		 * One difference with the ussuance of the Transcript by Faber here is that a Job-Certificate can be revoked and the credential creation 
		 * takes the ID of the revocation registry created earlier by Acme and a handle to the blob storage containing the validity tails
		 */
		
		String credValuesJson = new JSONObject()
			.put("first_name", new JSONObject().put("raw", "Alice").put("encoded", "1139481716457488690172217916278103335"))
			.put("last_name", new JSONObject().put("raw", "Garcia").put("encoded", "5321642780241790123587902456789123452"))
			.put("employee_status", new JSONObject().put("raw", "Permanent").put("encoded", "2143135425425143112321314321"))
			.put("salary", new JSONObject().put("raw", "2400").put("encoded", "2400"))
			.put("experience", new JSONObject().put("raw", "10").put("encoded", "10")).toString();
		
		String revocRegId = ctx.revocRegistryId;
		BlobStorageReader blobStorageReader = BlobStorageReader.openReader("default", getTailsWriterConfig()).get();
		int blobStorageReaderHandle = blobStorageReader.getBlobStorageReaderHandle();
			    			    		
		IssuerCreateCredentialResult createCredentialResult = Anoncreds.issuerCreateCredential(ctx.acmeWallet, ctx.jobCertificateCredOffer, credentialRequestJson, credValuesJson, revocRegId, blobStorageReaderHandle).get();
		String jobCertificateCredJson = createCredentialResult.getCredentialJson();
		String revocRegDeltaJson = createCredentialResult.getRevocRegDeltaJson();
		ctx.jobCertificateCredRevocId = createCredentialResult.getRevocId();
		log.info("IssuedCredential: " + jobCertificateCredJson);
		log.info("RevocRegDelta: " + revocRegDeltaJson);
		log.info("RevocId: " + ctx.jobCertificateCredRevocId);

		/* 4. Acme publishs a revocation registry entry
		 * 
		 * Other parties can then verify the revocation state of the credential
		 */
		
		String revocRegEntryRequest = Ledger.buildRevocRegEntryRequest(ctx.acmeDid, revocRegId, "CL_ACCUM", revocRegDeltaJson).get();
		signAndSubmitRequest(ctx, ctx.acmeWallet, ctx.acmeDid, revocRegEntryRequest);
		
		// 5. Alice requests the Revocation Registry Definition before storing the Credential
		
		String revocRegDefRequest = Ledger.buildGetRevocRegDefRequest(ctx.aliceDidForAcme, revocRegId).get();
		String revocRegDefResponse = Ledger.submitRequest(ctx.pool, revocRegDefRequest).get();
		ParseResponseResult parseResponseResult = Ledger.parseGetRevocRegDefResponse(revocRegDefResponse).get();
		String revocRegDefJson = parseResponseResult.getObjectJson();
		log.info("RevocRegDefResponse: " + revocRegDefJson);
		
		/* 6. Alice stores Job-Certificate Credential from Acme in her Wallet
		 * 
		 * She can use it when she applies for her loan, in much the same way that she used her transcript when applying for a job.
		 * 
		 * There is a disadvantage in this approach to data sharing though, — it may disclose more data than what is strictly necessary. 
		 * If all Alice needs to do is provide proof of employment, this can be done with an anonymous credential instead.
		 *  
		 * Anonymous credentials may prove certain predicates without disclosing actual values (e.g., Alice is employed full-time, 
		 * with a salary greater than X, along with her hire date, but her actually salary remains hidden).
		 *  
		 * A compound proof can be created, drawing from credentials from both Faber College and Acme Corp, that discloses only what is necessary.
		 */
		
		String jobCredentialId = Anoncreds.proverStoreCredential(ctx.aliceWallet, null, credentialRequestMetadataJson, jobCertificateCredJson, jobCertificateCredDef, revocRegDefJson).get();
		log.info("Job-Certificate Credential Id: " + jobCredentialId);

		/* 7. Alice gets a Loan-Application-Basic Proof Request from Thrift Bank
		 * 
		 * The last line indicates that the Job-Certificate provided should not be revoked by the application time.
		 */
		
		String nonce = Anoncreds.generateNonce().get();
		Long timestamp = System.currentTimeMillis() / 1000;
		JSONArray jobCertificateRestrictions = new JSONArray().put(new JSONObject().put("cred_def_id", ctx.jobCertificateCredDefId));
		
		String proofRequestJson = new JSONObject()
				.put("nonce", nonce)
				.put("name", "Loan-Application-Basic")
				.put("version", "0.1")
				.put("requested_attributes", new JSONObject()
					.put("attr1_referent", new JSONObject()
						.put("name", "employee_status")
						.put("restrictions", jobCertificateRestrictions)))
				.put("requested_predicates", new JSONObject()
					.put("predicate1_referent", new JSONObject()
						.put("name", "salary")
						.put("p_type", ">=")
						.put("p_value", 2000)
						.put("restrictions", jobCertificateRestrictions))
					.put("predicate2_referent", new JSONObject()
						.put("name", "experience")
						.put("p_type", ">=")
						.put("p_value", 1)
						.put("restrictions", jobCertificateRestrictions)))
				.put("non_revoked", new JSONObject().put("to", timestamp))
				.toString();
			
		log.info("Loan-Application Proof Request: " + proofRequestJson);

		// 8. Alice searches her Wallet for Credentials that she can use for creating a Proof for the Loan-Application Proof Request
		
		CredentialsSearchForProofReq credentialsSearch = CredentialsSearchForProofReq.open(ctx.aliceWallet, proofRequestJson, null).get();
		
		JSONArray credentialsForAttribute1 = new JSONArray(credentialsSearch.fetchNextCredentials("attr1_referent", 100).get());
		String credentialIdForAttribute1 = credentialsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		JSONArray credentialsForPredicate1 = new JSONArray(credentialsSearch.fetchNextCredentials("predicate1_referent", 100).get());
		String credentialIdForPredicate1 = credentialsForPredicate1.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		
		JSONArray credentialsForPredicate2 = new JSONArray(credentialsSearch.fetchNextCredentials("predicate2_referent", 100).get());
		String credentialIdForPredicate2 = credentialsForPredicate2.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		
		credentialsSearch.close();
		
		// 9. Alice gets the Credential Schema and correspoding Credential Definition
		
		JSONObject schemasMap = new JSONObject();
		JSONObject credDefsMap = new JSONObject();
		
		populateCredentialInfo(ctx.pool, ctx.aliceDidForAcme, schemasMap, credDefsMap, credentialsForAttribute1);
		
		String schemas = schemasMap.toString();
		String credDefs = credDefsMap.toString();

		// 10. Proover (Alice) obtains the Revocation State
		
		// [TODO] Explain how the prover (Alice) obtains revocRegDeltaJson, it comes from Anoncreds.issuerCreateCredential
		String revocStateJson = Anoncreds.createRevocationState(blobStorageReaderHandle, revocRegDefJson, revocRegDeltaJson, timestamp, ctx.jobCertificateCredRevocId).get();
		String revocState = new JSONObject().put(revocRegId, new JSONObject().put("" + timestamp, new JSONObject(revocStateJson))).toString();
		log.info("Revocation State: " + revocStateJson);
		
		// 11. Alice provides Loan-Application Proof
		
		String credentialsJson = new JSONObject()
			.put("self_attested_attributes", new JSONObject())
			.put("requested_attributes", new JSONObject()
				.put("attr1_referent", new JSONObject()
					.put("cred_id", credentialIdForAttribute1)
					.put("revealed", true)
					.put("timestamp", timestamp)))
			.put("requested_predicates", new JSONObject()
				.put("predicate1_referent", new JSONObject()
					.put("cred_id",credentialIdForPredicate1)
					.put("timestamp", timestamp))
				.put("predicate2_referent", new JSONObject()
					.put("cred_id",credentialIdForPredicate2)
					.put("timestamp", timestamp)))
			.toString();
		
		/* 12. Alice creates the Proof for Thrift Loan-Application Proof Request
		 * 
		 * Alice sends just the Loan-Application-Basic proof to the bank. This allows her to minimize the PII (personally identifiable information) 
		 * that she has to share when all she’s trying to do right now is prove basic eligibility.
		 */
		
		String proofJson = Anoncreds.proverCreateProof(ctx.aliceWallet, proofRequestJson, credentialsJson, ctx.aliceMasterSecretId, schemas, credDefs, revocState).get();
		JSONObject proof = new JSONObject(proofJson);
		log.info("Proof: " + proof);

		// 13. Thrift Bank verifies the Proof
		
		JSONObject revealedAttrs = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs");
		log.info("RevealedAttrs: " + revealedAttrs);
		
		assertEquals("Permanent", revealedAttrs.getJSONObject("attr1_referent").getString("raw"));
		
		String revocRegDefs = new JSONObject().put(revocRegId, new JSONObject(revocRegDefJson)).toString();
		String revocRegs = new JSONObject().put(revocRegId, new JSONObject().put("" + timestamp, new JSONObject(revocRegDeltaJson))).toString();
		
		Boolean accepted = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemas, credDefs, revocRegDefs, revocRegs).get();
		assertTrue("Proof not accepted", accepted);
	}

	void kycProcessWithThrift(Context ctx) throws Exception {

		// 1. Alice gets a second Proof Request from Thrift Bank
		
		String nonce = Anoncreds.generateNonce().get();
		String proofRequestJson = new JSONObject()
				.put("nonce", nonce)
				.put("name", "Loan-Application-KYC")
				.put("version", "0.1")
				.put("requested_attributes", new JSONObject()
					.put("attr1_referent", new JSONObject().put("name", "first_name"))
					.put("attr2_referent", new JSONObject().put("name", "last_name"))
					.put("attr3_referent", new JSONObject().put("name", "ssn")))
				.put("requested_predicates", new JSONObject())
				.toString();
			
		log.info("Loan-Application-KYC Proof Request: " + proofRequestJson);
		
		// 2. Alice searches her Wallet for Credentials that she can use for creating a Proof for the Loan-Application-KYC Proof Request
		
		CredentialsSearchForProofReq credentialsSearch = CredentialsSearchForProofReq.open(ctx.aliceWallet, proofRequestJson, null).get();
		
		JSONArray credentialsForAttribute1 = new JSONArray(credentialsSearch.fetchNextCredentials("attr1_referent", 100).get());
		String credentialIdForAttribute1 = credentialsForAttribute1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		JSONArray credentialsForAttribute2 = new JSONArray(credentialsSearch.fetchNextCredentials("attr2_referent", 100).get());
		String credentialIdForAttribute2 = credentialsForAttribute2.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		JSONArray credentialsForAttribute3 = new JSONArray(credentialsSearch.fetchNextCredentials("attr3_referent", 100).get());
		String credentialIdForAttribute3 = credentialsForAttribute3.getJSONObject(0).getJSONObject("cred_info").getString("referent");
		
		credentialsSearch.close();

		// 3. Alice gets the Credential Schema and correspoding Credential Definition
		
		JSONObject schemasMap = new JSONObject();
		JSONObject credDefsMap = new JSONObject();
		
		populateCredentialInfo(ctx.pool, ctx.aliceDidForThrift, schemasMap, credDefsMap, credentialsForAttribute1);
		populateCredentialInfo(ctx.pool, ctx.aliceDidForThrift, schemasMap, credDefsMap, credentialsForAttribute2);
		populateCredentialInfo(ctx.pool, ctx.aliceDidForThrift, schemasMap, credDefsMap, credentialsForAttribute3);
		
		String schemas = schemasMap.toString();
		String credDefs = credDefsMap.toString();
		String revocState = new JSONObject().toString();

		// 4. Alice provides Loan-Application-KYC Proof
		
		String credentialsJson = new JSONObject()
			.put("self_attested_attributes", new JSONObject())
			.put("requested_attributes", new JSONObject()
				.put("attr1_referent", new JSONObject()
					.put("cred_id", credentialIdForAttribute1)
					.put("revealed", true))
				.put("attr2_referent", new JSONObject()
					.put("cred_id", credentialIdForAttribute2)
					.put("revealed", true))
				.put("attr3_referent", new JSONObject()
					.put("cred_id", credentialIdForAttribute3)
					.put("revealed", true)))
			.put("requested_predicates", new JSONObject())
			.toString();

		// 5. Alice creates the Proof for Thrift Loan-Application-KYC Proof Request

		String proofJson = Anoncreds.proverCreateProof(ctx.aliceWallet, proofRequestJson, credentialsJson, ctx.aliceMasterSecretId, schemas, credDefs, revocState).get();
		JSONObject proof = new JSONObject(proofJson);
		log.info("Proof: " + proof);

		// 6. Thrift verifies the Proof
		
		JSONObject revealedAttrs = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs");
		log.info("RevealedAttrs: " + revealedAttrs);
		
		assertEquals("Alice", revealedAttrs.getJSONObject("attr1_referent").getString("raw"));
		assertEquals("Garcia", revealedAttrs.getJSONObject("attr2_referent").getString("raw"));
		assertEquals("123-45-6789", revealedAttrs.getJSONObject("attr3_referent").getString("raw"));
		
		String revocRegDefs = new JSONObject().toString();
		String revocRegs = new JSONObject().toString();
		
		Boolean accepted = Anoncreds.verifierVerifyProof(proofRequestJson, proofJson, schemas, credDefs, revocRegDefs, revocRegs).get();
		assertTrue("Proof not accepted", accepted);
	}

	void quitJobWithAcme(Context ctx) throws Exception {
		
		// 1. Acme revokes the Job-Certificate Credential
		
		String revocRegId = ctx.revocRegistryId;
		String credRevocId = ctx.jobCertificateCredRevocId; 
		BlobStorageReader blobStorageReader = BlobStorageReader.openReader("default", getTailsWriterConfig()).get();
		int blobStorageReaderHandle = blobStorageReader.getBlobStorageReaderHandle();
		
		Anoncreds.issuerRevokeCredential(ctx.acmeWallet, blobStorageReaderHandle, revocRegId, credRevocId);
	}

	private String getTailsWriterConfig() {
		return new JSONObject().put("base_dir", EnvironmentUtils.getIndyHomePath("tails")).put("uri_pattern", "").toString();
	}
	
	private void populateCredentialInfo(Pool pool, String did, JSONObject schemas, JSONObject credDefs, JSONArray credentials) throws Exception {
		for (JSONObject o : array2List(credentials)) {
			JSONObject credInfo = o.getJSONObject("cred_info");
			String schemaId = credInfo.getString("schema_id");
			String credDefId = credInfo.getString("cred_def_id");
			if (schemas.isNull(schemaId)) {
				String getSchemaRequest = Ledger.buildGetSchemaRequest(did, schemaId).get();
				String getSchemaResponse = Ledger.submitRequest(pool, getSchemaRequest).get();
				ParseResponseResult parseSchemaResult = Ledger.parseGetSchemaResponse(getSchemaResponse).get();
				String schemaJson = parseSchemaResult.getObjectJson();
				schemas.put(schemaId, new JSONObject(schemaJson));
			}
			if (credDefs.isNull(credDefId)) {
				String getCredDefRequest = Ledger.buildGetCredDefRequest(did, credDefId).get();
				String getCredDefResponse = Ledger.submitRequest(pool, getCredDefRequest).get();
				ParseResponseResult parseCredDefResponse = Ledger.parseGetCredDefResponse(getCredDefResponse).get();
				String credDefJson = parseCredDefResponse.getObjectJson();
				credDefs.put(credDefId, new JSONObject(credDefJson));
			}
		}
	}

	private List<JSONObject> array2List(JSONArray credentials) {
		List<JSONObject> result = new ArrayList<>();
		credentials.forEach(o -> result.add((JSONObject) o));
		return result;
	}

	private String signAndSubmitRequest(Context ctx, Wallet endorserWallet, String endorserDid, String request) throws Exception {
		return submitRequest(ctx, Ledger.signRequest(endorserWallet, endorserDid, request).get());
	}

	private String submitRequest(Context ctx, String req) throws Exception {
		String res = Ledger.submitRequest(ctx.pool, req).get();
		if ("REPLY".equals(new JSONObject(res).get("op"))) {
			log.info("SubmitRequest: " + req);
			log.info("SubmitResponse: " + res);
		} else {
			log.warn("SubmitRequest: " + req);
			log.warn("SubmitResponse: " + res);
		}
		return res.toString();
	}

	private void closeAndDeletePoolLedger(Context ctx) throws Exception {
		
		log.info("Close Wallets");
		
		closeAndDeleteWallet(ctx.aliceWallet, ctx.aliceWalletConfig, ctx.aliceWalletKey);
		closeAndDeleteWallet(ctx.thriftWallet, ctx.thriftWalletConfig, ctx.thriftWalletKey);
		closeAndDeleteWallet(ctx.acmeWallet, ctx.acmeWalletConfig, ctx.acmeWalletKey);
		closeAndDeleteWallet(ctx.faberWallet, ctx.faberWalletConfig, ctx.faberWalletKey);
		closeAndDeleteWallet(ctx.governmentWallet, ctx.governmentWalletConfig, ctx.governmentWalletKey);
		closeAndDeleteWallet(ctx.trusteeWallet, ctx.trusteeWalletConfig, ctx.trusteeWalletKey);
		
		log.info("Close and Delete Pool Ledger");

		ctx.pool.closePoolLedger().get();
		Pool.deletePoolLedgerConfig(ctx.poolName).get();
	}

	private void closeAndDeleteWallet(Wallet wallet, String config, String key) throws Exception {
		if (wallet != null) {
			wallet.closeWallet().get();
			Wallet.deleteWallet(config, key).get();
		}
	}
	
	private void assertEquals(String exp, String was) {
		if (exp == null || !exp.equals(was)) 
			throw new IllegalStateException(String.format("Expected %s, but got %s", exp, was));
	}
	
	private void assertTrue(String msg, boolean was) {
		if (!was) 
			throw new IllegalStateException(msg);
	}
}
