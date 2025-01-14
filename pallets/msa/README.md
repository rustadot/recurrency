# MSA Pallet

The MSA Pallet provides functionality for handling Message Source Accounts.

## Summary

The Message Source Account (MSA) is the primary user account system on Recurrency for Messages and Stateful Storage.
All users on Recurrency must have an MSA in order to:

1. Acquire a User Handle
2. Delegate tasks to Providers (defining specific tasks for specific providers)
3. Become a Provider so they may do Provider level tasks on their own behalf
4. Have Message or Stateful Storage data

### MSA Id and Keys

Once a user creates an MSA, they are assigned an MSA Id, a unique number the time of creation with one or more keys attached for control.
(A control key may only be attached to ONE MSA at any single point in time.)

### Actions

The MSA pallet provides for:

- Creating, reading, updating, and deleting operations for MSAs.
- Managing delegation relationships for MSAs.
- Managing keys associated with MSAs.

## Interactions

### Extrinsics

| Name/Description                                                                              | Caller                                     | Payment            | Key Events                                                                                                                                                                                                                                       | Runtime Added |
| --------------------------------------------------------------------------------------------- | ------------------------------------------ | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------- |
| `add_public_key_to_msa`<br />Add MSA control key                                              | MSA Control Key or Provider with Signature | Capacity or Tokens | [`PublicKeyAdded`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.PublicKeyAdded)                                                                                                                         | 1             |
| `create`<br />Create new MSA                                                                  | Token Account                              | Tokens             | [`MsaCreated`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.MsaCreated)                                                                                                                                 | 1             |
| `create_provider`<br />Convert an MSA into a Provider                                         | Testnet: Provider or Mainnet: Governance   | Tokens             | [`ProviderCreated`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.ProviderCreated)                                                                                                                       | 1             |
| `create_provider_via_governance`<br />Convert an MSA into a Provider                          | Recurrency Council                          | Tokens             | [`ProviderCreated`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.ProviderCreated)                                                                                                                       | 12            |
| `create_sponsored_account_with_delegation`<br />Create new MSA via Provider with a Delegation | Provider                                   | Capacity or Tokens | [`MsaCreated`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.MsaCreated), [`DelegationGranted`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.DelegationGranted) | 1             |
| `delete_msa_public_key`<br />Remove MSA control key                                           | Delegator                                  | Free               | [`PublicKeyDeleted`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.PublicKeyDeleted)                                                                                                                     | 1             |
| `grant_delegation`<br />Create or alter a delegation                                          | Provider with Signature                    | Capacity           | [`DelegationGranted`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.DelegationGranted)                                                                                                                   | 1             |
| `propose_to_be_provider`<br />Request the council to convert an MSA to a Provider             | Token Account                              | Tokens             | [`Proposed`](https://paritytech.github.io/polkadot-sdk/master/pallet_collective/pallet/enum.Event.html#variant.Proposed)                                                                                                                         | 12            |
| `retire_msa`<br />Remove all keys and mark the MSA as retired                                 | Delegator                                  | Free               | [`PublicKeyDeleted`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.PublicKeyDeleted), [`MsaRetired`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.MsaRetired)   | 18            |
| `revoke_delegation_by_delegator`<br />Remove delegation                                       | Delegator                                  | Free               | [`DelegationRevoked`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.DelegationRevoked)                                                                                                                   | 1             |
| `revoke_delegation_by_provider`<br />Remove delegation                                        | Provider                                   | Free               | [`DelegationRevoked`](https://rustadot.github.io/recurrency/pallet_msa/pallet/enum.Event.html#variant.DelegationRevoked)                                                                                                                   | 1             |

See [Rust Docs](https://rustadot.github.io/recurrency/pallet_msa/pallet/struct.Pallet.html) for more details.

### State Queries

| Name                              | Description                                                                                                       | Query                              | Runtime Added |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ---------------------------------- | ------------- |
| Get MSA Id                        | Returns the MSA Id connected to the given control key                                                             | `publicKeyToMsaId`                 | 1             |
| Get Current Maximum MSA Id        | Returns the maximum MSA Id in existence                                                                           | `currentMsaIdentifierMaximum`      | 1             |
| Get Current Delegator to Provider | Returns the current relationship between the specified Delegator and specified Provider at the given block number | `delegatorAndProviderToDelegation` | 1             |
| Get Public Key Count for MSA Id   | Returns the number of public keys for the given MSA Id                                                            | `publicKeyCountforMsaId`           | 1             |

See the [Rust Docs](https://rustadot.github.io/recurrency/pallet_msa/pallet/storage_types/index.html) for additional state queries and details.

### RPCs

Note: May be restricted based on node settings and configuration.

| Name                          | Description                                                                | Call                                                                                                                                                                   | Node Version |
| ----------------------------- | -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| Check Delegations             | Test a list of MSAs to see if they have delegated to the provider MSA      | [`checkDelegations`](https://rustadot.github.io/recurrency/pallet_msa_rpc/trait.MsaApiServer.html#tymethod.check_delegations                )                    | v1.0.0+      |
| Delegation Schema Grants      | Fetch the list of Schema Ids that a delegator has granted to a provider    | [`grantedSchemaIdsByMsaId`](https://rustadot.github.io/recurrency/pallet_msa_rpc/trait.MsaApiServer.html#tymethod.get_granted_schemas_by_msa_id)                 | v1.0.0+      |
| Get Control Keys by MSA Id\*  | Fetch the list of current control keys for an MSA from the off-chain index | [`getKeysByMsaId`](https://rustadot.github.io/recurrency/pallet_msa_rpc/trait.MsaApiServer.html#tymethod.get_keys_by_msa_id)                                     | v1.10.0+     |
| Get All Delegations by MSA Id | Retreives all delegations and schemas, active and inactive, for an MSA ID  | ['getAllGrantedDelegationsByMsaId'](https://rustadot.github.io/recurrency/pallet_msa_rpc/trait.MsaApiServer.html#tymethod.get_all_granted_delegations_by_msa_id) | v1.13.0+     |

\* Must be enabled with off-chain indexing

See [Rust Docs](https://rustadot.github.io/recurrency/pallet_msa_rpc/trait.MsaApiServer.html) for more details.
