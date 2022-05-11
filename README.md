# Futureland Core
This repo contains the code for the smart contract that implements Futureland's core functionality.

## Setup
See [Near instructions for setting up Near's rust sdk](https://docs.near.org/docs/develop/contracts/rust/intro)

## Building
To build, run
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deploying and running on testnet
To deploy to testnet, run

```bash
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/futureland_core.wasm
```

To update the `CONTRACT_NAME` environment variable, run
```bash
source neardev/dev-account.env
```

To call a smart contract function, run
```bash
near call $CONTRACT_NAME  <function name> <function args> --accountId $CONTRACT_NAME
```

### Example Usage:
```bash
# deploy smart contract
$ near dev-deploy target/wasm32-unknown-unknown/release/futureland_core.wasm
# set up the $CONTRACT_NAME environment variable
$ source neardev/*.env
# initialize smart contract
$ near call $CONTRACT_NAME  new --accountId $CONTRACT_NAME
# create a new project (returns its id)
$ near call $CONTRACT_NAME create_project '{"project_name": "past land", "project_description": "the opposite of futureland"}' --accountId clarkpoon.testnet
# get information about a project (with id 0)
$ near call $CONTRACT_NAME get_project '{"project_id": 0}' --accountId clarkpoon.testnet
```
You can replace clarkpoon.testnet with your near account id in the example above

### Common Issues

If you get an error like this
```
Scheduling a call: dev-1651179569064-59716888327982.create_project({"owner_id": "clarkpoon.testnet", "total_shares": "1000", "project_name": "test project", "project_symbol": "TP"})
Doing account.functionCall()
Receipt: HYebgAuveRKLKMQ5FV3LhM6NvMs1oBPU4rha8s4ZsBGZ
	Failure [dev-1651179569064-59716888327982]: Error: {"index":0,"kind":{"ExecutionError":"Smart contract panicked: The account is already registered"}}
ServerTransactionError: {"index":0,"kind":{"ExecutionError":"Smart contract panicked: The account is already registered"}}
    at Object.parseResultError (/usr/local/lib/node_modules/near-cli/node_modules/near-api-js/lib/utils/rpc_errors.js:31:29)
    at Account.signAndSendTransactionV2 (/usr/local/lib/node_modules/near-cli/node_modules/near-api-js/lib/account.js:160:36)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)
    at async scheduleFunctionCall (/usr/local/lib/node_modules/near-cli/commands/call.js:57:38)
    at async Object.handler (/usr/local/lib/node_modules/near-cli/utils/exit-on-error.js:52:9) {
  type: 'FunctionCallError',
  context: undefined,
  index: 0,
  kind: {
    ExecutionError: 'Smart contract panicked: The account is already registered'
  },
  transaction_outcome: {
    block_hash: '6T2Hz3DaRu8AUqSywMdk7B7XRr5MpiQ27mRvEmTg5Wcf',
    id: 'BRC7FYA1t6HPGgQ7wohSrBFhk9M6eAZNuQGiCtF4TQFN',
    outcome: {
      executor_id: 'dev-1651179569064-59716888327982',
      gas_burnt: 2428189312080,
      logs: [],
      metadata: [Object],
      receipt_ids: [Array],
      status: [Object],
      tokens_burnt: '242818931208000000000'
    },
    proof: [ [Object] ]
  }
}
```
You can fix it by deleting the `neardev/` directory and redeploying the contract