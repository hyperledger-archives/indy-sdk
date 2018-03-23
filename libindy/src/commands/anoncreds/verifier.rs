extern crate serde_json;
extern crate indy_crypto;

use errors::common::CommonError;
use errors::indy::IndyError;

use services::anoncreds::AnoncredsService;
use services::anoncreds::types::{
    ClaimDefinition,
    Schema,
    ProofRequestJson,
    ProofJson,
    Predicate,
    RevocationRegistry};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use utils::json::JsonDecodable;
use self::indy_crypto::bn::BigNumber;

use self::indy_crypto::authz::AuthzAccumulators;

pub enum VerifierCommand {
    VerifyProof(
        String, // proof request json
        String, // proof json
        String, // schemas json
        String, // claim defs jsons
        String, // revoc regs json
        Option<String>, // accumulator
        Box<Fn(Result<bool, IndyError>) + Send>)
}

pub struct VerifierCommandExecutor {
    anoncreds_service: Rc<AnoncredsService>,
}

impl VerifierCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>) -> VerifierCommandExecutor {
        VerifierCommandExecutor {
            anoncreds_service,
        }
    }

    pub fn execute(&self, command: VerifierCommand) {
        match command {
            VerifierCommand::VerifyProof(proof_request_json,
                                         proof_json, schemas_json,
                                         claim_defs_jsons, revoc_regs_json, accumulator, cb) => {
                info!(target: "verifier_command_executor", "VerifyProof command received");
                self.verify_proof(&proof_request_json, &proof_json, &schemas_json,
                                  &claim_defs_jsons, &revoc_regs_json, accumulator.as_ref().map(String::as_str), cb);
            }
        };
    }

    fn verify_proof(&self,
                    proof_request_json: &str,
                    proof_json: &str,
                    schemas_json: &str,
                    claim_defs_jsons: &str,
                    revoc_regs_json: &str,
                    accumulator: Option<&str>,
                    cb: Box<Fn(Result<bool, IndyError>) + Send>) {
        let result = self._verify_proof(proof_request_json, proof_json,
                                        schemas_json, claim_defs_jsons, revoc_regs_json,
                                        accumulator);
        cb(result)
    }

    fn _verify_proof(&self,
                     proof_request_json: &str,
                     proof_json: &str,
                     schemas_json: &str,
                     claim_defs_jsons: &str,
                     revoc_regs_json: &str,
                     accumulator: Option<&str>) -> Result<bool, IndyError> {
        let proof_req: ProofRequestJson = ProofRequestJson::from_json(proof_request_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid proof_request_json: {}", err.to_string())))?;

        let schemas: HashMap<String, Schema> = serde_json::from_str(schemas_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid schemas_json: {}", err.to_string())))?;

        let claim_defs: HashMap<String, ClaimDefinition> = serde_json::from_str(claim_defs_jsons)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid claim_defs_jsons: {}", err.to_string())))?;

        let revoc_regs: HashMap<String, RevocationRegistry> = serde_json::from_str(revoc_regs_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid revoc_regs_json: {}", err.to_string())))?;

        let proof_claims: ProofJson = ProofJson::from_json(&proof_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid proof_json: {}", err.to_string())))?;

        let accumulators = match accumulator {
            Some(ref a) => Some(AuthzAccumulators {
                provisioned: BigNumber::from_dec(a)?,
                // Next value is a dummy
                revoked: BigNumber::from_dec("3081076980007713925959604505993528370091252037219432570372268594215374344448178557927330505659829548196001541910397044574503804349888018481637021558820201495180652853839389373736354855089915654973646390593825402963369085598243175499331073210996369094524584737045812083227000057867534282991348704391352741963150846904684740081827853667405049291950563131087880761067102663540026285677395840413088174715082923530727315147001628040210198449287076046431564773892455551663145805008468300855353569160670769107273734038564797672775330380086322494841998600095197724618324340015371990124082923120036387971884629120210018406941724331997393028520399574577331410576856213987051688696744981047884734763621690605729499908487228784147873046418572825090779840794201637455940120941399814886616382625317460565896684910105909504707053360780917358988981169951477861282629772514247925341760072523828218731362450851436561547397656679975389798488887044194917116583673849515191323195914776662418177189603969638270971498700371796973381598884749348990545341638774881936625120806067766409748553292974329171798703826943631238764095392265996716384324391039958314190118742197999180964544894121268375540809742303181386694824266665061651735305169041165995833012997227")?
            }),
            None => None
        };

        let requested_attrs: HashSet<String> =
            proof_req.requested_attrs
                .keys()
                .map(|uuid| uuid.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let requested_predicates: HashSet<Predicate> =
            proof_req.requested_predicates
                .values()
                .map(|uuid| uuid.clone())
                .into_iter()
                .collect::<HashSet<Predicate>>();

        let received_revealed_attrs: HashSet<String> =
            proof_claims.requested_proof.revealed_attrs
                .keys()
                .map(|uuid| uuid.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_unrevealed_attrs: HashSet<String> =
            proof_claims.requested_proof.unrevealed_attrs
                .keys()
                .map(|uuid| uuid.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_attrs = received_revealed_attrs
            .union(&received_unrevealed_attrs)
            .map(|attr| attr.clone())
            .collect::<HashSet<String>>();

        let received_predicates: HashSet<Predicate> =
            proof_claims.proofs
                .values()
                .flat_map(|k| k.proof.primary_proof.ge_proofs.iter()
                    .map(|p| p.predicate.clone()))
                .into_iter()
                .collect::<HashSet<Predicate>>();

        if requested_attrs != received_attrs {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Requested attributes {:?} do not correspond to received {:?}", requested_attrs, received_attrs))));
        }

        // TODO: Uncomment me; The structure of attributes and predicates is different
        /*if requested_predicates != received_predicates {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Requested predicates {:?} do not correspond to received {:?}", requested_predicates, received_predicates))));
        }

        let received_revealed_attrs_values: HashSet<(String, String)> =
            proof_claims.requested_proof.revealed_attrs
                .values()
                .map(|&(ref uuid, _, ref encoded_value)| (uuid.clone(), encoded_value.clone()))
                .collect::<HashSet<(String, String)>>();

        let received_revealed_attrs_values_from_equal_proof: HashSet<(String, String)> = proof_claims.proofs.iter()
            .flat_map(|(uuid, proof)|
                proof.proof.primary_proof.eq_proof.revealed_attrs.values().map(move |encoded_value| (uuid.clone(), encoded_value.clone()))
            )
            .into_iter()
            .collect::<HashSet<(String, String)>>();

        if received_revealed_attrs_values != received_revealed_attrs_values_from_equal_proof { return Ok(false); }*/

        let result = self.anoncreds_service.verifier.verify(&proof_claims,
                                                            &proof_req.nonce,
                                                            &claim_defs,
                                                            &revoc_regs,
                                                            &schemas, accumulators.as_ref())?;

        Ok(result)
    }
}