#ifndef __CXS_H
#define __CXS_H

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
} cxs_claim_state_t;

typedef unsigned int cxs_error_t;
typedef unsigned int cxs_schema_handle_t;
typedef unsigned int cxs_claimdef_handle_t;
typedef unsigned int cxs_connection_handle_t;
typedef unsigned int cxs_claim_handle_t;
typedef unsigned int cxs_proof_handle_t;
typedef unsigned int cxs_command_handle_t;

typedef struct {

  union {
    cxs_schema_handle_t schema_handle;
    cxs_claimdef_handle_t claimdef_handle;
    cxs_connection_handle_t connection_handle;
    cxs_claim_handle_t claim_handle;
    cxs_proof_handle_t proof_handle;
  } handle;

  cxs_error_t status;
  char *msg;

} cxs_status_t;


/**
 * Initialize the SDK
 */

cxs_error_t cxs_init(const char *config_path);


/**
 * Schema object
 *
 * For creating, validating and committing a schema to the sovrin ledger.
 */

/** Creates a schema from a json string. Populates a handle to the new schema. */
cxs_error_t cxs_schema_create(const char *schema_data, cxs_schema_handle_t *schema_handle);

/** Asynchronously commits the schema to the ledger. */
cxs_error_t cxs_schema_commit(cxs_schema_handle_t schema_handle);

/** Populates data with the contents of the schema handle. */
cxs_error_t cxs_schema_get_data(cxs_schema_handle_t schema_handle, char *data);

/** Populates sequence_no with the actual sequence number of the schema on the sovrin ledger. */
cxs_error_t cxs_schema_get_sequence_no(cxs_schema_handle_t schema_handle, int *sequence_no);


/**
 * claimdef object
 *
 * For creating, validating and committing a claim definition to the sovrin ledger.
 */

/** Creates a claim definition from the given schema.  Populates a handle to the new claimdef. */
cxs_error_t cxs_claimdef_create(cxs_schema_handle_t schema_handle, cxs_claimdef_handle_t *claimdef_handle);

/** Asynchronously commits the claimdef to the ledger.  */
cxs_error_t cxs_claimdef_commit(cxs_claimdef_handle_t claimdef_handle);

/** Populates sequence_no with the actual sequence number of the claimdef on the sovrin ledger. */
cxs_error_t cxs_claimdef_get_sequence_no(cxs_claimdef_handle_t claimdef_handle, int *sequence_no);

/** Populates data with the contents of the claimdef handle. */
cxs_error_t cxs_claimdef_get(cxs_claimdef_handle_t claimdef_handle, char *data);


/**
 * connection object
 *
 * For creating a connection with an identity owner for interactions such as exchanging
 * claims and proofs.
 */

/** Creates a connection object to a specific identity owner. Populates a handle to the new connection. */
cxs_error_t cxs_connection_create(const char *recipient_info, cxs_connection_handle_t *connection_handle);

/** Asynchronously request a connection be made. */
cxs_error_t cxs_connection_connect(cxs_connection_handle_t connection_handle, const char *connection_type);

/** Returns the contents of the connection handle or null if the connection does not exist. */
cxs_error_t cxs_connection_serialize(cxs_connection_handle_t connection_handle, void (*cb)(cxs_connection_handle_t connection_handle, cxs_error_t err, const char *state));

/** Populates status with the current state of the asynchronous connection request. */
cxs_error_t cxs_connection_get_state(cxs_connection_handle_t connection_handle, cxs_claim_state_t *status);

/** Releases the connection from memory. */
cxs_error_t cxs_connection_release(cxs_connection_handle_t connection_handle);

/**
 * claim issuer object
 *
 * Used for offering and managing a claim with an identity owner.
 */

/** Creates a claim object from the specified claimdef handle. Populates a handle the new claim. */
cxs_error_t cxs_issuer_create_claim(cxs_command_handle_t, const char *source_id, cxs_claimdef_handle_t claimdef_handle, const char * claim_data, void (*cb)(cxs_command_handle_t command_handle, cxs_error_t err, cxs_claim_handle_t *claim_handle));

/** Asynchronously sends the claim offer to the connection. */
cxs_error_t cxs_issuer_send_claim_offer(cxs_claim_handle_t claim_handle, cxs_connection_handle_t connection_handle);

/** Populates claim_request with the latest claim request received. */
cxs_error_t cxs_issuer_get_claim_request(cxs_claim_handle_t claim_handle, char *claim_request);

/** Sets the claim request in an accepted state. */
cxs_error_t cxs_issuer_accept_claim(cxs_claim_handle_t claim_handle);

/** Asynchronously send the claim to the connection. Populates a handle to the new transaction. */
cxs_error_t cxs_issuer_send_claim(cxs_claim_handle_t claim_handle, cxs_connection_handle_t connection_handle);

/** Terminates a claim for the specified reason. */
cxs_error_t cxs_issuer_terminate_claim(cxs_claim_handle_t claim_handle, cxs_claim_state_t state_type, const char *msg);

/** Populates status with the current state of this claim. */
cxs_error_t cxs_issuer_claim_serialize(cxs_claim_handle_t claim_handle, void (*cb)(cxs_claim_handle_t claim_handle, cxs_error_t err, const char *state));

/** Re-creates a claim object from the specified serialization. */
cxs_error_t cxs_issuer_claim_deserialize(cxs_command_handle_t, const char *serialized_claim, void (*cb)(cxs_command_handle_t command_handle, cxs_error_t err, cxs_claim_handle_t *claim_handle));

/**
 * proof object
 *
 * Used for requesting and managing a proof request with an identity owner.
 */

/** Creates a proof object.  Populates a handle to the new proof. */
cxs_error_t cxs_proof_create(const char *proof_request_data, cxs_proof_handle_t *proof_handle);

/** Sets the specific connection for this proof request. */
cxs_error_t cxs_proof_set_connection(cxs_proof_handle_t proof_handle, cxs_connection_handle_t connection_handle);

/** Asynchronously send a proof request to the connection. */
cxs_error_t cxs_proof_send_request(cxs_proof_handle_t proof_handle);

/** Populate response_data with the latest proof offer received. */
cxs_error_t cxs_proof_get_proof_offer(cxs_proof_handle_t proof_handle, char *proof_offer);

/** Set proof offer as accepted. */
cxs_error_t cxs_proof_accepted(cxs_proof_handle_t proof_handle);

/** Populate status_array with the state of each proof handle. */
cxs_error_t cxs_proof_list_state(cxs_status_t *status_array);

/** Populates status with the current state of this proof request. */
cxs_error_t cxs_proof_get_state(cxs_proof_handle_t proof_handle, char *status);


#ifdef __cplusplus
}
#endif

#endif
