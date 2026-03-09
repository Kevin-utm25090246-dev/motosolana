import * as web3 from "@solana/web3.js";
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { Program, AnchorProvider, web3 } from '@coral-xyz/anchor';
import idl from './idl.json'; import type { SolanaAutohub } from "../target/types/solana_autohub";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SolanaAutohub as anchor.Program<SolanaAutohub>;

// El archivo JSON que genera Anchor al compilar

const programID = new PublicKey("TuProgramIDAqui...");
const network = clusterApiUrl('devnet');
const opts = { preflightCommitment: "processed" };

const getProvider = () => {
  const connection = new Connection(network, opts.preflightCommitment);
  const provider = new AnchorProvider(
    window.solana, // La wallet (Phantom)
    connection,
    opts.preflightCommitment,
  );
  return provider;
}

async function comprarPieza(partPublicKey) {
  const provider = getProvider();
  const program = new Program(idl, programID, provider);

  try {
    await program.methods
      .buyPart()
      .accounts({
        partAccount: partPublicKey,
        buyer: provider.wallet.publicKey,
        sellerAccount: "PublicKeyDelVendedor...",
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
    
    alert("¡Pieza comprada con éxito!");
  } catch (err) {
    console.error("Error en la transacción: ", err);
  }
}