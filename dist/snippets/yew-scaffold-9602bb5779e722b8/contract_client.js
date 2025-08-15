
import { Client, networks } from "/contract-bindings/index.js";

export async function helloContract(to) {
  const client = new Client({
    contractId: networks.testnet.contractId,
    networkPassphrase: networks.testnet.networkPassphrase,
    rpcUrl: "https://soroban-testnet.stellar.org"
  });

  const tx = await client.hello({ to });
  return tx.result;
}

