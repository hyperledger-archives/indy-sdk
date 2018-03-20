#ifndef __VCX_H
#define __VCX_H

#ifdef __cplusplus
extern "C" {
#endif

typedef enum
{
  none = 0,
  initialized,
  offer_sent,
  request_received,
  accepted,
  unfulfilled,
  expired,
  revoked,
} vcx_state_t;

typedef enum
{
  undefined = 0,
  validated = 1,
  invalid = 2,
} vcx_proof_state_t;

typedef unsigned int vcx_error_t;
typedef unsigned int vcx_schema_handle_t;
typedef unsigned int vcx_claimdef_handle_t;
typedef unsigned int vcx_connection_handle_t;
typedef unsigned int vcx_claim_handle_t;
typedef unsigned int vcx_proof_handle_t;
typedef unsigned int vcx_command_handle_t;
typedef unsigned int vcx_bool_t;

typedef struct {

  union {
    vcx_schema_handle_t schema_handle;
    vcx_claimdef_handle_t claimdef_handle;
    vcx_connection_handle_t connection_handle;
    vcx_claim_handle_t claim_handle;
    vcx_proof_handle_t proof_handle;
  } handle;

  vcx_error_t status;
  char *msg;

} vcx_status_t;


/**
 * Initialize the SDK
 */

vcx_error_t vcx_init(vcx_command_handle_t handle, const char *config_path,void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err));
vcx_error_t vcx_error_message(vcx_command_handle_t handle, vcx_error_t error_code, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *error_msg));


/**
 * Schema object
 *
 * For creating, validating and committing a schema to the sovrin ledger.
 */

/** Creates a schema from a json string. Populates a handle to the new schema. */
vcx_error_t vcx_schema_create(vcx_command_handle_t command_handle, const char *source_id, const char *schema_name, const char *schema_data, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_schema_handle_t schema_handle));

/** Populates status with the current state of this claim. */
vcx_error_t vcx_schema_serialize(vcx_command_handle_t command_handle, vcx_schema_handle_t schema_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a claim object from the specified serialization. */
vcx_error_t vcx_schema_deserialize(vcx_command_handle_t command_handle, const char *serialized_schema, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_schema_handle_t schema_handle));

/** Populates data with the contents of the schema handle. */
vcx_error_t vcx_schema_get_attributes(vcx_command_handle_t command_handle, const char *source_id, vcx_schema_handle_t sequence_no,  void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *schema_attrs));

/** Populates sequence_no with the actual sequence number of the schema on the sovrin ledger. */
vcx_error_t vcx_schema_get_sequence_no(vcx_command_handle_t command_handle, vcx_schema_handle_t schema_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_schema_handle_t sequence_no));

/** Release memory associated with schema object. */
vcx_error_t vcx_schema_release(vcx_schema_handle_t handle);


/**
 * claimdef object
 *
 * For creating, validating and committing a claim definition to the sovrin ledger.
 */

/** Creates a claim definition from the given schema.  Populates a handle to the new claimdef. */
vcx_error_t vcx_claimdef_create(vcx_command_handle_t command_handle, const char *source_id, const char *claimdef_name, vcx_schema_handle_t schema_seq_no, vcx_bool_t revocation, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_claimdef_handle_t claimdef_handle));

/** Populates status with the current state of this claim. */
vcx_error_t vcx_claimdef_serialize(vcx_command_handle_t command_handle, vcx_claimdef_handle_t claimdef_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a claim object from the specified serialization. */
vcx_error_t vcx_claimdef_deserialize(vcx_command_handle_t command_handle, const char *serialized_claimdef, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_claimdef_handle_t claimdef_handle));

/** Asynchronously commits the claimdef to the ledger.  */
vcx_error_t vcx_claimdef_commit(vcx_claimdef_handle_t claimdef_handle);

/** Populates sequence_no with the actual sequence number of the claimdef on the sovrin ledger. */
vcx_error_t vcx_claimdef_get_sequence_no(vcx_claimdef_handle_t claimdef_handle, int *sequence_no);

/** Populates data with the contents of the claimdef handle. */
vcx_error_t vcx_claimdef_get(vcx_claimdef_handle_t claimdef_handle, char *data);


/**
 * connection object
 *
 * For creating a connection with an identity owner for interactions such as exchanging
 * claims and proofs.
 */

/** Creates a connection object to a specific identity owner. Populates a handle to the new connection. */
vcx_error_t vcx_connection_create(vcx_command_handle_t command_handle, const char *source_id, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_connection_handle_t connection_handle));

/** Asynchronously request a connection be made. */
vcx_error_t vcx_connection_connect(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, const char *connection_type, void (*cb)(vcx_command_handle_t, vcx_error_t err));

/** Returns the contents of the connection handle or null if the connection does not exist. */
vcx_error_t vcx_connection_serialize(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a connection object from the specified serialization. */
vcx_error_t vcx_connection_deserialize(vcx_command_handle_t command_handle, const char *serialized_claim, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_connection_handle_t connection_handle));

/** Request a state update from the agent for the given connection. */
vcx_error_t vcx_connection_update_state(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Retrieves the state of the connection */
vcx_error_t vcx_connection_get_state(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Releases the connection from memory. */
vcx_error_t vcx_connection_release(vcx_connection_handle_t connection_handle);

/** Get the invite details for the connection. */
vcx_error_t vcx_connection_invite_details(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, int abbreviated, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *details));

/** Creates a connection from the invite details. */
vcx_error_t vcx_connection_create_with_invite(vcx_command_handle_t command_handle, const char *source_id, const char *invite_details, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_connection_handle_t connection_handle));


/**
 * claim issuer object
 *
 * Used for offering and managing a claim with an identity owner.
 */

/** Creates a claim object from the specified claimdef handle. Populates a handle the new claim. */
vcx_error_t vcx_issuer_create_claim(vcx_command_handle_t command_handle, const char *source_id, vcx_schema_handle_t schema_seq_no, const char *issuer_did, const char * claim_data, const char * claim_name, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_claim_handle_t claim_handle));

/** Asynchronously sends the claim offer to the connection. */
vcx_error_t vcx_issuer_send_claim_offer(vcx_command_handle_t command_handle, vcx_claim_handle_t claim_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err));

/** Updates the state of the claim from the agency. */
vcx_error_t vcx_issuer_claim_update_state(vcx_command_handle_t command_handle, vcx_claim_handle_t claim_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Retrieves the state of the issuer_claim. */
vcx_error_t vcx_issuer_claim_get_state(vcx_command_handle_t command_handle, vcx_claim_handle_t claim_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Asynchronously send the claim to the connection. Populates a handle to the new transaction. */
vcx_error_t vcx_issuer_send_claim(vcx_command_handle_t command_handle, vcx_claim_handle_t claim_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err));

/** Populates status with the current state of this claim. */
vcx_error_t vcx_issuer_claim_serialize(vcx_command_handle_t command_handle, vcx_claim_handle_t claim_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a claim object from the specified serialization. */
vcx_error_t vcx_issuer_claim_deserialize(vcx_command_handle_t, const char *serialized_claim, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_claim_handle_t claim_handle));

/** Terminates a claim for the specified reason. */
vcx_error_t vcx_issuer_terminate_claim(vcx_command_handle_t command_handle, vcx_claim_handle_t claim_handle, vcx_state_t state_type, const char *msg);

/** Releases the claim from memory. */
vcx_error_t vcx_issuer_claim_release(vcx_claim_handle_t claim_handle);

/** Populates claim_request with the latest claim request received. (not in MVP) */
vcx_error_t vcx_issuer_get_claim_request(vcx_claim_handle_t claim_handle, char *claim_request);

/** Sets the claim request in an accepted state. (not in MVP) */
vcx_error_t vcx_issuer_accept_claim(vcx_claim_handle_t claim_handle);

/**
 * proof object
 *
 * Used for requesting and managing a proof request with an identity owner.
 */

/** Creates a proof object.  Populates a handle to the new proof. */
vcx_error_t vcx_proof_create(vcx_command_handle_t command_handle, const char *source_id, const char *requested_attrs, const char *requested_predicates, const char *name, void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_proof_handle_t proof_handle));

/** Asynchronously send a proof request to the connection. */
vcx_error_t vcx_proof_send_request(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err));

/** Populate response_data with the latest proof offer received. */
vcx_error_t vcx_get_proof(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_proof_state_t state, const char *proof_string));

/** Set proof offer as accepted. */
vcx_error_t vcx_proof_accepted(vcx_proof_handle_t proof_handle);

/** Populates status with the current state of this proof request. */
vcx_error_t vcx_proof_update_state(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Retrieves the state of the proof. */
vcx_error_t vcx_proof_get_state(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Populates status with the current state of this claim. */
vcx_error_t vcx_proof_serialize(vcx_command_handle_t command_handle, vcx_proof_handle_t proof_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a claim object from the specified serialization. */
vcx_error_t vcx_proof_deserialize(vcx_command_handle_t command_handle, const char *serialized_proof, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_proof_handle_t proof_handle));

/** Releases the proof from memory. */
vcx_error_t vcx_proof_release(vcx_proof_handle_t proof_handle);


/**
 * claim object
 *
 * Used for accepting and requesting a claim with an identity owner.
 */

/** Creates a claim object from the specified claimdef handle. Populates a handle the new claim. */
vcx_error_t vcx_claim_create_with_offer(vcx_command_handle_t command_handle, const char *source_id, const char *claim_offer,void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, vcx_claim_handle_t claim_handle));

/** Asynchronously sends the claim request to the connection. */
vcx_error_t vcx_claim_send_request(vcx_command_handle_t command_handle, vcx_claim_handle_t claim_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err));

/** Check for any new claim offers from the connection. */
vcx_error_t vcx_claim_get_offers(vcx_command_handle_t command_handle, vcx_connection_handle_t connection_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *offers));

/** Updates the state of the claim from the agency. */
vcx_error_t vcx_claim_update_state(vcx_command_handle_t command_handle, vcx_claim_handle_t claim_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Retrieves the state of the claim - including storing the claim if it has been sent. */
vcx_error_t vcx_claim_get_state(vcx_command_handle_t command_handle, vcx_claim_handle_t claim_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_state_t state));

/** Populates status with the current state of this claim. */
vcx_error_t vcx_claim_serialize(vcx_command_handle_t command_handle, vcx_claim_handle_t claim_handle, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, const char *state));

/** Re-creates a claim from the specified serialization. */
vcx_error_t vcx_claim_deserialize(vcx_command_handle_t, const char *serialized_claim, void (*cb)(vcx_command_handle_t xcommand_handle, vcx_error_t err, vcx_claim_handle_t claim_handle));

/** Releases the claim from memory. */
vcx_error_t vcx_claim_release(vcx_claim_handle_t claim_handle);

void vcx_set_next_agency_response(int);
#ifdef __cplusplus
}
#endif

#endif
