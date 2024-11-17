# Makefile usage

## Build

the following command build a smart contract form source code and optimize it wasm

```
make build
```

## Deploy new version of the smart contract

the following command build and deploy a new version of smart contract

```
make install SPONSOR_SEED=S....D ADMIN_ADDRESS=GC...D PAY_ASSET=MyTestUSD
```

where

| parameter     | Example                                                  | Description                                                                                      |
|---------------|----------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| SPONSOR_SEED  | SBZVYT2CZSNYULKBSFOEEHLT2DBCQPR7TU2F5F65WRRV4D4HK7P4SLEM | A seed of stellar sponsor account with enough XLM balance                                        |
| ADMIN_ADDRESS | GBIVVVJYRBDPPGZV6YQZVK5JQHBQYVSIRZ3UZJ2H3HMDP5STS5ZOISKA | A public address of admin stellar key pair that will be manage the smart contract                |
| PAY_ASSET     | MyTestUSD                                                | An asset code that will be used for payout, initilizazed with ADMIN_ADDRESS as an issuer account |

## Upgrade deployed smart contract

the following command build and upgraded the smart contract with new version

```
make upgrade SPONSOR_SEED=S...M ADMIN_ADDRESS=GC...A ADMIN_SEED=S...F UPGRADE_CONTRACT=C...V
```

where

| parameter        | Example                                                  | Description                                                                       |
|------------------|----------------------------------------------------------|-----------------------------------------------------------------------------------|
| SPONSOR_SEED     | SBZVYT2CZSNYULKBSFOEEHLT2DBCQPR7TU2F5F65WRRV4D4HK7P4SLEM | A seed of stellar sponsor account with enough XLM balance                         |
| ADMIN_ADDRESS    | GBIVVVJYRBDPPGZV6YQZVK5JQHBQYVSIRZ3UZJ2H3HMDP5STS5ZOISKA | A public address of admin stellar key pair that will be manage the smart contract |
| ADMIN_SEED       | SAYNZCCBWGWDV5AUSJI57YVWYRZXODMBXCSDNILOTU53FFUC2DKT5YFF | A seed of admin stellar key pair that manages the smart contract                  |
| UPGRADE_CONTRACT | CDLBKCXG6XCNVHKEB2EJWVKDHOANVPSTKNMYUROTMK7LDA46QWVKRAVV | A stellar smart contract address that should be upgraded                          |

