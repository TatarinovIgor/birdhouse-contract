STELLAR_NETWORK ?= testnet
# GCGYWKPQCJBZSAV5HKNS5Q45ZURYONJV5Z5O5BVTPRHQMGUH5C7M4PSS
# SBZVYT2CZSNYULKBSFOEEHLT2DBCQPR7TU2F5F65WRRV4D4HK7P4SLEM
SPONSOR_SEED ?= SBZVYT2CZSNYULKBSFOEEHLT2DBCQPR7TU2F5F65WRRV4D4HK7P4SLEM
# GBIVVVJYRBDPPGZV6YQZVK5JQHBQYVSIRZ3UZJ2H3HMDP5STS5ZOISKA
# SAYNZCCBWGWDV5AUSJI57YVWYRZXODMBXCSDNILOTU53FFUC2DKT5YFF
ADMIN_ADDRESS ?= GBBBBFAHGH2SXCD4APGSZF54ZA2ORJ4W7NJ3NFYRI3Z5VPAVAEWSSRIX
ADMIN_SEED ?= SB3YZLQKSSBNAESUONMOGPQ2SXU5RG6M4AOPNKDZN5HZGRMFBSX7J4BH
PAY_ASSET ?= payTestUSD

UPGRADE_CONTRACT ?= CDLBKCXG6XCNVHKEB2EJWVKDHOANVPSTKNMYUROTMK7LDA46QWVKRAVV

build:
	stellar contract build
	stellar contract optimize --wasm .\target\wasm32-unknown-unknown\release\master_contract.wasm

install: build
	stellar contract deploy --wasm .\target\wasm32-unknown-unknown\release\master_contract.optimized.wasm \
			 --network $(STELLAR_NETWORK) --very-verbose \
			 --source-account $(SPONSOR_SEED) \
			 -- --admin $(ADMIN_ADDRESS) \
			 --pay_asset $(PAY_ASSET)

upgrade: build
	$(eval INSTALL_HASH := $(shell stellar contract install -q --network $(STELLAR_NETWORK) \
 			 --wasm .\target\wasm32-unknown-unknown\release\master_contract.optimized.wasm \
			 --source-account "$(SPONSOR_SEED)"))
	@echo "Install hash: $(INSTALL_HASH)"
	stellar contract invoke  --network $(STELLAR_NETWORK)  --source-account $(ADMIN_SEED) \
			 --id $(UPGRADE_CONTRACT) \
 		 	 -- upgrade --new_wasm_hash $(INSTALL_HASH)