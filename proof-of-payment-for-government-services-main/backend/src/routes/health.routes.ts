import { Router } from "express";

export const healthRouter = Router();

healthRouter.get("/", (_request, response) => {
  response.json({
    ok: true,
    project: "Proof of Payment for Government Services",
    network: process.env.STELLAR_NETWORK ?? "testnet",
    rpcUrl: process.env.STELLAR_RPC_URL ?? "https://soroban-testnet.stellar.org",
  });
});
