// contract-bindings/index.js

// Keep Buffer polyfill available
import { Buffer } from "https://esm.sh/buffer"
if (typeof window !== "undefined") {
  window.Buffer = window.Buffer || Buffer
}

// Minimal callable binding stub you can click today
// Replace with real contract call later
export async function hello(args) {
  const to = args?.to ?? "world"
  console.log("contract hello called with:", to)
  return `hello ${to}`
}
