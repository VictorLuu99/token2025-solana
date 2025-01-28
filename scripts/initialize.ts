import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { Token2025 } from "./idl/token2025";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { findMetadataPda } from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { publicKey } from "@metaplex-foundation/umi";
dotenv.config();

async function initialize() {
  const devnet = true;
  const privateKey = Uint8Array.from(JSON.parse(process.env.PRIV_KEY || ''));
  const keyPair = Keypair.fromSecretKey(privateKey);
  const connection = new Connection(clusterApiUrl(devnet ? "devnet" : "mainnet-beta"), { commitment: 'confirmed' });
  const wallet = new Wallet(keyPair);
  const provider = new AnchorProvider(connection, wallet);
  const IDL: Token2025 = require("./idl/token2025.json");
  const program = new Program(IDL, provider);
  const mint = Keypair.generate();

  const umi = createUmi(devnet ? clusterApiUrl("devnet") : clusterApiUrl("mainnet-beta"));
  const [metadataString,] = findMetadataPda(umi, { mint: publicKey(mint.publicKey.toBase58()) });
  const metadata = new PublicKey(metadataString);
  // check address program
  console.log("program", IDL.address);
  
  console.log("🚀 ~ initialize ~ metadata:", metadata.toBase58());
  
  const tx = new Transaction().add(await program.methods.initialize(
    "Happy New Year 2025",
    "token2025",
    "https://raw.githubusercontent.com/VictorLuu99/token2025-solana/main/assets/metadata.json",
  ).accounts({
    admin: keyPair.publicKey,
    mint: mint.publicKey,
    metadata
  }).instruction());

  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.feePayer = keyPair.publicKey;
  console.log("keyPair.publicKey", keyPair.publicKey.toBase58());
  
  const recoverTx = Transaction.from(tx.serialize({ requireAllSignatures: false }));
  recoverTx.partialSign(keyPair);
  recoverTx.partialSign(mint);
  const txSignature = await connection.sendRawTransaction(recoverTx.serialize({ requireAllSignatures: true }));
  console.log("🚀 ~ initialize ~ txSignature:", txSignature)
  let latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: txSignature,
  });
  console.log("🚀 ~ initialize ~ txSignature:", txSignature)
}

initialize();