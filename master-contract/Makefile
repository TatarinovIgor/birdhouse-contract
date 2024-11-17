# GCGYWKPQCJBZSAV5HKNS5Q45ZURYONJV5Z5O5BVTPRHQMGUH5C7M4PSS
# SBZVYT2CZSNYULKBSFOEEHLT2DBCQPR7TU2F5F65WRRV4D4HK7P4SLEM
SPONSOR_SEED ?= SBZVYT2CZSNYULKBSFOEEHLT2DBCQPR7TU2F5F65WRRV4D4HK7P4SLEM
# GBIVVVJYRBDPPGZV6YQZVK5JQHBQYVSIRZ3UZJ2H3HMDP5STS5ZOISKA
# SAYNZCCBWGWDV5AUSJI57YVWYRZXODMBXCSDNILOTU53FFUC2DKT5YFF
ADMIN_ADDRESS ?= GBIVVVJYRBDPPGZV6YQZVK5JQHBQYVSIRZ3UZJ2H3HMDP5STS5ZOISKA
ADMIN_SEED ?= SAYNZCCBWGWDV5AUSJI57YVWYRZXODMBXCSDNILOTU53FFUC2DKT5YFF
PAY_ASSET ?= payTestUSD

UPGRADE_CONTRACT ?= CDLBKCXG6XCNVHKEB2EJWVKDHOANVPSTKNMYUROTMK7LDA46QWVKRAVV

build:
	stellar contract build
	stellar contract optimize --wasm .\target\wasm32-unknown-unknown\release\master_contract.wasm

install: build
	stellar contract deploy --wasm .\target\wasm32-unknown-unknown\release\master_contract.optimized.wasm \
			 --network testnet --very-verbose \
			 --source-account $(SPONSOR_SEED) \
			 -- --admin $(ADMIN_ADDRESS) \
			 --pay_asset $(PAY_ASSET)

upgrade: build
	$(eval INSTALL_HASH := $(shell stellar contract install -q --network testnet \
 			 --wasm .\target\wasm32-unknown-unknown\release\master_contract.optimized.wasm \
			 --source-account "$(SPONSOR_SEED)"))
	@echo "Filename: $(INSTALL_HASH)"
	stellar contract invoke  --network testnet  --source-account $(ADMIN_SEED) \
			 --id $(UPGRADE_CONTRACT) \
 		 	 -- upgrade --new_wasm_hash $(INSTALL_HASH)