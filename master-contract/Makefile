build:
	stellar contract build
	stellar contract optimize --wasm .\target\wasm32-unknown-unknown\release\master_contract.wasm

install: build
	stellar contract deploy --wasm .\target\wasm32-unknown-unknown\release\master_contract.optimized.wasm \
			--source-account SBE452ZHMIPBKENO4VYKKRLKZ3HLSNA7OOFZ7KE3ZPHNGXBTOOQMAOB6 \
			 --network testnet --very-verbose
