STELLAR_NETWORK ?= testnet
# GCGYWKPQCJBZSAV5HKNS5Q45ZURYONJV5Z5O5BVTPRHQMGUH5C7M4PSS
# SBZVYT2CZSNYULKBSFOEEHLT2DBCQPR7TU2F5F65WRRV4D4HK7P4SLEM
SPONSOR_SEED ?= SBZVYT2CZSNYULKBSFOEEHLT2DBCQPR7TU2F5F65WRRV4D4HK7P4SLEM

build:
	stellar contract build
	stellar contract optimize --wasm .\target\wasm32-unknown-unknown\release\demand_contract.wasm

install: build
	$(eval INSTALL_HASH := $(shell stellar contract install -q --network $(STELLAR_NETWORK) \
 			 --wasm .\target\wasm32-unknown-unknown\release\demand_contract.optimized.wasm \
			 --source-account "$(SPONSOR_SEED)"))
	@echo "Install hash: $(INSTALL_HASH)"