## Substrate project structure

```
├── CODEOWNERS
├── CODE_OF_CONDUCT.adoc
├── CONTRIBUTING.adoc
├── Cargo.lock
├── Cargo.tom
├── Dockerfile
├── LICENSE
├── PULL_REQUEST_TEMPLATE.md
├── README.adoc
├── build.rs
├── ci
│   └── script.sh
├── core
│   ├── application-crypto
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── ed25519.rs
│   │       ├── lib.rs
│   │       ├── sr25519.rs
│   │       └── traits.rs
│   ├── basic-authorship
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── basic_authorship.rs
│   │       └── lib.rs
│   ├── cli
│   │   ├── Cargo.toml
│   │   ├── README.adoc
│   │   └── src
│   │       ├── error.rs
│   │       ├── execution_strategy.rs
│   │       ├── informant
│   │       │   └── display.rs
│   │       ├── informant.rs
│   │       ├── lib.rs
│   │       ├── params.rs
│   │       └── traits.rs
│   ├── client
│   │   ├── Cargo.toml
│   │   ├── db
│   │   │   ├── Cargo.toml
│   │   │   └── src
│   │   │       ├── cache
│   │   │       │   ├── list_cache.rs
│   │   │       │   ├── list_entry.rs
│   │   │       │   ├── list_storage.rs
│   │   │       │   └── mod.rs
│   │   │       ├── lib.rs
│   │   │       ├── light.rs
│   │   │       ├── offchain.rs
│   │   │       ├── storage_cache.rs
│   │   │       └── utils.rs
│   │   └── src
│   │       ├── backend.rs
│   │       ├── block_builder
│   │       │   ├── api.rs
│   │       │   ├── block_builder.rs
│   │       │   └── mod.rs
│   │       ├── blockchain.rs
│   │       ├── call_executor.rs
│   │       ├── children.rs
│   │       ├── cht.rs
│   │       ├── client.rs
│   │       ├── error.rs
│   │       ├── genesis.rs
│   │       ├── in_mem.rs
│   │       ├── leaves.rs
│   │       ├── lib.rs
│   │       ├── light
│   │       │   ├── backend.rs
│   │       │   ├── blockchain.rs
│   │       │   ├── call_executor.rs
│   │       │   ├── fetcher.rs
│   │       │   └── mod.rs
│   │       ├── notifications.rs
│   │       └── runtime_api.rs
│   ├── consensus
│   │   ├── aura
│   │   │   ├── Cargo.toml
│   │   │   ├── primitives
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   └── src
│   │   │       ├── digest.rs
│   │   │       └── lib.rs
│   │   ├── babe
│   │   │   ├── Cargo.toml
│   │   │   ├── primitives
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       ├── digest.rs
│   │   │   │       └── lib.rs
│   │   │   └── src
│   │   │       ├── aux_schema.rs
│   │   │       ├── lib.rs
│   │   │       └── tests.rs
│   │   ├── common
│   │   │   ├── Cargo.toml
│   │   │   ├── primitives
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   └── src
│   │   │       ├── block_import.rs
│   │   │       ├── error.rs
│   │   │       ├── evaluation.rs
│   │   │       ├── import_queue
│   │   │       │   ├── basic_queue.rs
│   │   │       │   └── buffered_link.rs
│   │   │       ├── import_queue.rs
│   │   │       ├── lib.rs
│   │   │       ├── offline_tracker.rs
│   │   │       └── select_chain.rs
│   │   ├── rhd
│   │   │   ├── Cargo.toml
│   │   │   └── src
│   │   │       ├── error.rs
│   │   │       ├── lib.rs
│   │   │       ├── misbehaviour_check.rs
│   │   │       └── service.rs
│   │   ├── slots
│   │   │   ├── Cargo.toml
│   │   │   └── src
│   │   │       ├── aux_schema.rs
│   │   │       ├── lib.rs
│   │   │       └── slots.rs
│   │   └── uncles
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── lib.rs
│   ├── executor
│   │   ├── Cargo.toml
│   │   ├── runtime-test
│   │   │   ├── Cargo.toml
│   │   │   ├── build.rs
│   │   │   └── src
│   │   │       └── lib.rs
│   │   └── src
│   │       ├── allocator.rs
│   │       ├── error.rs
│   │       ├── lib.rs
│   │       ├── native_executor.rs
│   │       ├── sandbox.rs
│   │       ├── wasm_executor.rs
│   │       ├── wasm_runtimes_cache.rs
│   │       └── wasm_utils.rs
│   ├── finality-grandpa
│   │   ├── Cargo.toml
│   │   ├── primitives
│   │   │   ├── Cargo.toml
│   │   │   └── src
│   │   │       └── lib.rs
│   │   └── src
│   │       ├── authorities.rs
│   │       ├── aux_schema.rs
│   │       ├── communication
│   │       │   ├── gossip.rs
│   │       │   ├── mod.rs
│   │       │   ├── periodic.rs
│   │       │   └── tests.rs
│   │       ├── consensus_changes.rs
│   │       ├── environment.rs
│   │       ├── finality_proof.rs
│   │       ├── import.rs
│   │       ├── justification.rs
│   │       ├── lib.rs
│   │       ├── light_import.rs
│   │       ├── observer.rs
│   │       ├── service_integration.rs
│   │       ├── tests.rs
│   │       └── until_imported.rs
│   ├── inherents
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── keyring
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── ed25519.rs
│   │       ├── lib.rs
│   │       └── sr25519.rs
│   ├── keystore
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── network
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── behaviour.rs
│   │       ├── chain.rs
│   │       ├── config.rs
│   │       ├── debug_info.rs
│   │       ├── discovery.rs
│   │       ├── error.rs
│   │       ├── legacy_proto
│   │       │   ├── behaviour.rs
│   │       │   ├── handler.rs
│   │       │   ├── mod.rs
│   │       │   ├── tests.rs
│   │       │   └── upgrade.rs
│   │       ├── lib.rs
│   │       ├── on_demand_layer.rs
│   │       ├── protocol
│   │       │   ├── consensus_gossip.rs
│   │       │   ├── event.rs
│   │       │   ├── light_dispatch.rs
│   │       │   ├── message.rs
│   │       │   ├── specialization.rs
│   │       │   ├── sync
│   │       │   │   ├── blocks.rs
│   │       │   │   └── extra_requests.rs
│   │       │   ├── sync.rs
│   │       │   └── util.rs
│   │       ├── protocol.rs
│   │       ├── service.rs
│   │       ├── test
│   │       │   ├── block_import.rs
│   │       │   ├── mod.rs
│   │       │   └── sync.rs
│   │       └── transport.rs
│   ├── offchain
│   │   ├── Cargo.toml
│   │   ├── primitives
│   │   │   ├── Cargo.toml
│   │   │   └── src
│   │   │       └── lib.rs
│   │   └── src
│   │       ├── api.rs
│   │       ├── lib.rs
│   │       └── testing.rs
│   ├── panic-handler
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── peerset
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── lib.rs
│   │   │   └── peersstate.rs
│   │   └── tests
│   │       └── fuzz.rs
│   ├── primitives
│   │   ├── Cargo.toml
│   │   ├── benches
│   │   │   └── benches.rs
│   │   └── src
│   │       ├── changes_trie.rs
│   │       ├── crypto.rs
│   │       ├── ed25519.rs
│   │       ├── hash.rs
│   │       ├── hasher.rs
│   │       ├── hashing.rs
│   │       ├── hexdisplay.rs
│   │       ├── lib.rs
│   │       ├── offchain.rs
│   │       ├── sandbox.rs
│   │       ├── sr25519.rs
│   │       ├── storage.rs
│   │       ├── testing.rs
│   │       ├── tests.rs
│   │       ├── traits.rs
│   │       ├── u32_trait.rs
│   │       └── uint.rs
│   ├── rpc
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── author
│   │       │   ├── error.rs
│   │       │   ├── hash.rs
│   │       │   ├── mod.rs
│   │       │   └── tests.rs
│   │       ├── chain
│   │       │   ├── error.rs
│   │       │   ├── mod.rs
│   │       │   ├── number.rs
│   │       │   └── tests.rs
│   │       ├── errors.rs
│   │       ├── helpers.rs
│   │       ├── lib.rs
│   │       ├── metadata.rs
│   │       ├── state
│   │       │   ├── error.rs
│   │       │   ├── mod.rs
│   │       │   └── tests.rs
│   │       ├── subscriptions.rs
│   │       └── system
│   │           ├── error.rs
│   │           ├── helpers.rs
│   │           ├── mod.rs
│   │           └── tests.rs
│   ├── rpc-servers
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── serializer
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── service
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── chain_ops.rs
│   │   │   ├── chain_spec.rs
│   │   │   ├── components.rs
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   └── lib.rs
│   │   └── test
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── lib.rs
│   ├── session
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── sr-api-macros
│   │   ├── Cargo.toml
│   │   ├── benches
│   │   │   └── bench.rs
│   │   ├── src
│   │   │   ├── decl_runtime_apis.rs
│   │   │   ├── impl_runtime_apis.rs
│   │   │   ├── lib.rs
│   │   │   └── utils.rs
│   │   └── tests
│   │       ├── decl_and_impl.rs
│   │       ├── runtime_calls.rs
│   │       ├── trybuild.rs
│   │       └── ui
│   │           ├── adding_at_parameter.rs
│   │           ├── adding_at_parameter.stderr
│   │           ├── adding_self_parameter.rs
│   │           ├── adding_self_parameter.stderr
│   │           ├── changed_in_unknown_version.rs
│   │           ├── changed_in_unknown_version.stderr
│   │           ├── declaring_old_block.rs
│   │           ├── declaring_old_block.stderr
│   │           ├── declaring_own_block_with_different_name.rs
│   │           ├── declaring_own_block_with_different_name.stderr
│   │           ├── empty_impl_runtime_apis_call.rs
│   │           ├── empty_impl_runtime_apis_call.stderr
│   │           ├── impl_incorrect_method_signature.rs
│   │           ├── impl_incorrect_method_signature.stderr
│   │           ├── impl_two_traits_with_same_name.rs
│   │           ├── impl_two_traits_with_same_name.stderr
│   │           ├── invalid_api_version.rs
│   │           ├── invalid_api_version.stderr
│   │           ├── invalid_api_version_2.rs
│   │           ├── invalid_api_version_2.stderr
│   │           ├── invalid_api_version_3.rs
│   │           ├── invalid_api_version_3.stderr
│   │           ├── missing_block_generic_parameter.rs
│   │           ├── missing_block_generic_parameter.stderr
│   │           ├── missing_path_for_trait.rs
│   │           ├── missing_path_for_trait.stderr
│   │           ├── type_reference_in_impl_runtime_apis_call.rs
│   │           └── type_reference_in_impl_runtime_apis_call.stderr
│   ├── sr-io
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── src
│   │   │   ├── lib.rs
│   │   │   └── offchain
│   │   │       ├── http.rs
│   │   │       └── mod.rs
│   │   ├── with_std.rs
│   │   └── without_std.rs
│   ├── sr-primitives
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── generic
│   │       │   ├── block.rs
│   │       │   ├── checked_extrinsic.rs
│   │       │   ├── digest.rs
│   │       │   ├── era.rs
│   │       │   ├── header.rs
│   │       │   ├── mod.rs
│   │       │   ├── tests.rs
│   │       │   └── unchecked_extrinsic.rs
│   │       ├── lib.rs
│   │       ├── testing.rs
│   │       ├── traits.rs
│   │       ├── transaction_validity.rs
│   │       └── weights.rs
│   ├── sr-sandbox
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── src
│   │   │   └── lib.rs
│   │   ├── with_std.rs
│   │   └── without_std.rs
│   ├── sr-std
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── src
│   │   │   └── lib.rs
│   │   ├── with_std.rs
│   │   └── without_std.rs
│   ├── sr-version
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── state-db
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       ├── noncanonical.rs
│   │       ├── pruning.rs
│   │       └── test.rs
│   ├── state-machine
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── backend.rs
│   │       ├── basic.rs
│   │       ├── changes_trie
│   │       │   ├── build.rs
│   │       │   ├── build_iterator.rs
│   │       │   ├── changes_iterator.rs
│   │       │   ├── input.rs
│   │       │   ├── mod.rs
│   │       │   ├── prune.rs
│   │       │   └── storage.rs
│   │       ├── ext.rs
│   │       ├── lib.rs
│   │       ├── overlayed_changes.rs
│   │       ├── proving_backend.rs
│   │       ├── testing.rs
│   │       ├── trie_backend.rs
│   │       └── trie_backend_essence.rs
│   ├── telemetry
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       ├── worker
│   │       │   └── node.rs
│   │       └── worker.rs
│   ├── test-client
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── client_ext.rs
│   │       └── lib.rs
│   ├── test-runtime
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── client
│   │   │   ├── Cargo.toml
│   │   │   └── src
│   │   │       ├── block_builder_ext.rs
│   │   │       ├── lib.rs
│   │   │       └── trait_tests.rs
│   │   └── src
│   │       ├── genesismap.rs
│   │       ├── lib.rs
│   │       └── system.rs
│   ├── transaction-pool
│   │   ├── Cargo.toml
│   │   ├── graph
│   │   │   ├── Cargo.toml
│   │   │   └── src
│   │   │       ├── base_pool.rs
│   │   │       ├── error.rs
│   │   │       ├── future.rs
│   │   │       ├── lib.rs
│   │   │       ├── listener.rs
│   │   │       ├── pool.rs
│   │   │       ├── ready.rs
│   │   │       ├── rotator.rs
│   │   │       └── watcher.rs
│   │   └── src
│   │       ├── api.rs
│   │       ├── error.rs
│   │       ├── lib.rs
│   │       └── tests.rs
│   ├── trie
│   │   ├── Cargo.toml
│   │   ├── benches
│   │   │   └── bench.rs
│   │   └── src
│   │       ├── error.rs
│   │       ├── lib.rs
│   │       ├── node_codec.rs
│   │       ├── node_header.rs
│   │       └── trie_stream.rs
│   └── utils
│       ├── fork-tree
│       │   ├── Cargo.toml
│       │   └── src
│       │       └── lib.rs
│       ├── wasm-builder
│       │   ├── Cargo.toml
│       │   ├── README.md
│       │   └── src
│       │       ├── lib.rs
│       │       ├── prerequisites.rs
│       │       └── wasm_project.rs
│       └── wasm-builder-runner
│           ├── Cargo.toml
│           ├── README.md
│           └── src
│               └── lib.rs
├── license_header.txt
├── node
│   ├── cli
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── doc
│   │   │   └── shell-completion.adoc
│   │   ├── res
│   │   │   └── flaming-fir.json
│   │   └── src
│   │       ├── chain_spec.rs
│   │       ├── factory_impl.rs
│   │       ├── lib.rs
│   │       └── service.rs
│   ├── executor
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── primitives
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── rpc-client
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── runtime
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   └── src
│   │       ├── constants.rs
│   │       ├── impls.rs
│   │       └── lib.rs
│   └── src
│       └── main.rs
├── node-template
│   ├── Cargo.toml
│   ├── LICENSE
│   ├── README.md
│   ├── build.rs
│   ├── runtime
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   └── src
│   │       ├── lib.rs
│   │       └── template.rs
│   ├── scripts
│   │   └── init.sh
│   └── src
│       ├── chain_spec.rs
│       ├── cli.rs
│       ├── main.rs
│       └── service.rs
├── scripts
│   ├── common.sh
│   ├── docker
│   │   ├── subkey.Dockerfile
│   │   └── substrate.Dockerfile
│   ├── flamingfir-deploy.sh
│   ├── getgoing.sh
│   ├── gitlab
│   │   ├── check_line_width.sh
│   │   └── check_runtime.sh
│   ├── init.sh
│   ├── kubernetes
│   │   ├── Chart.yaml
│   │   ├── README.md
│   │   ├── templates
│   │   │   ├── poddisruptionbudget.yaml
│   │   │   ├── secrets.yaml
│   │   │   ├── service.yaml
│   │   │   ├── serviceaccount.yaml
│   │   │   └── statefulset.yaml
│   │   └── values.yaml
│   ├── node-template-release
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── node-template-release.sh
│   ├── runtime-dep.py
│   ├── sentry-node
│   │   └── docker-compose.yml
│   ├── update-copyright.sh
│   ├── update-deps.sh
│   └── update.sh
├── srml
│   ├── README.adoc
│   ├── assets
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── aura
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       ├── mock.rs
│   │       └── tests.rs
│   ├── authorship
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── babe
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── balances
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       ├── mock.rs
│   │       └── tests.rs
│   ├── collective
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── contracts
│   │   ├── COMPLEXITY.md
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── account_db.rs
│   │       ├── exec.rs
│   │       ├── gas.rs
│   │       ├── lib.rs
│   │       ├── rent.rs
│   │       ├── tests.rs
│   │       └── wasm
│   │           ├── code_cache.rs
│   │           ├── env_def
│   │           │   ├── macros.rs
│   │           │   └── mod.rs
│   │           ├── mod.rs
│   │           ├── prepare.rs
│   │           └── runtime.rs
│   ├── council
│   │   └── src
│   │       └── lib.rs
│   ├── democracy
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       └── vote_threshold.rs
│   ├── elections
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── example
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── executive
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── finality-tracker
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── generic-asset
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       ├── mock.rs
│   │       └── tests.rs
│   ├── grandpa
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       ├── mock.rs
│   │       └── tests.rs
│   ├── im-online
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── indices
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── address.rs
│   │       ├── lib.rs
│   │       ├── mock.rs
│   │       └── tests.rs
│   ├── membership
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── metadata
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── session
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── historical.rs
│   │       ├── lib.rs
│   │       └── mock.rs
│   ├── staking
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── benches.rs
│   │       ├── inflation.rs
│   │       ├── lib.rs
│   │       ├── mock.rs
│   │       ├── phragmen.rs
│   │       └── tests.rs
│   ├── sudo
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── support
│   │   ├── Cargo.toml
│   │   ├── procedural
│   │   │   ├── Cargo.toml
│   │   │   ├── src
│   │   │   │   ├── lib.rs
│   │   │   │   └── storage
│   │   │   │       ├── impls.rs
│   │   │   │       ├── mod.rs
│   │   │   │       └── transformation.rs
│   │   │   └── tools
│   │   │       ├── Cargo.toml
│   │   │       ├── derive
│   │   │       │   ├── Cargo.toml
│   │   │       │   └── src
│   │   │       │       └── lib.rs
│   │   │       └── src
│   │   │           ├── lib.rs
│   │   │           └── syn_ext.rs
│   │   ├── src
│   │   │   ├── dispatch.rs
│   │   │   ├── double_map.rs
│   │   │   ├── event.rs
│   │   │   ├── hashable.rs
│   │   │   ├── inherent.rs
│   │   │   ├── lib.rs
│   │   │   ├── metadata.rs
│   │   │   ├── origin.rs
│   │   │   ├── runtime.rs
│   │   │   ├── storage
│   │   │   │   ├── child.rs
│   │   │   │   ├── hashed
│   │   │   │   │   ├── generator.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── storage_items.rs
│   │   │   │   └── unhashed
│   │   │   │       ├── generator.rs
│   │   │   │       └── mod.rs
│   │   │   ├── traits.rs
│   │   │   └── unsigned.rs
│   │   └── test
│   │       ├── Cargo.toml
│   │       ├── src
│   │       │   └── lib.rs
│   │       └── tests
│   │           ├── final_keys.rs
│   │           ├── genesisconfig.rs
│   │           ├── instance.rs
│   │           ├── issue2219.rs
│   │           ├── reserved_keyword
│   │           │   ├── on_initialize.rs
│   │           │   └── on_initialize.stderr
│   │           └── system.rs
│   ├── system
│   │   ├── Cargo.toml
│   │   ├── benches
│   │   │   └── bench.rs
│   │   └── src
│   │       └── lib.rs
│   ├── timestamp
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   └── treasury
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── subkey
│   ├── Cargo.toml
│   ├── README.adoc
│   └── src
│       ├── cli.yml
│       ├── main.rs
│       └── vanity.rs
└── test-utils
    ├── chain-spec-builder
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   └── src
    │       ├── cli.yml
    │       └── main.rs
    └── transaction-factory
        ├── Cargo.toml
        └── src
            ├── complex_mode.rs
            ├── lib.rs
            ├── modes.rs
            └── simple_modes.rs
```
234 directories, 563 files
