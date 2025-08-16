
import { Client, networks } from "/contract-bindings/index.js";

export async function hello({ to }) {
  const client = new Client({
    ...networks.testnet,
    // Replace with your RPC URL
    rpcUrl: "https://soroban-testnet.stellar.org",
    allowHttp: true,
  });

  const tx = await client.hello({ to });
  // In browser, you may need to simulate or sign depending on contract
  console.log("AssembledTransaction result:", tx.result);
  return tx.result;
}

