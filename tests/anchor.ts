import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaAutohub } from "../target/types/solana_autohub";
import { assert } from "chai";
import type { SolanaAutohub } from "../target/types/solana_autohub";

describe("solana_autohub", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaAutohub as anchor.Program<SolanaAutohub>;
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaAutohub as Program<SolanaAutohub>;

  it("¡Lista y compra una pieza de motor!", async () => {
    // Generamos una cuenta para la pieza
    const partAccount = anchor.web3.Keypair.generate();
    const buyer = provider.wallet;

    // 1. Llamada a 'list_part'
    await program.methods
      .listPart("Turbo Garrett", "Nissan Skyline R34", new anchor.BN(1000000000)) // 1 SOL
      .accounts({
        partAccount: partAccount.publicKey,
        seller: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([partAccount])
      .rpc();

    // 2. Verificar que se guardó correctamente
    let account = await program.account.sparePart.fetch(partAccount.publicKey);
    assert.equal(account.brand, "Turbo Garrett");
    assert.equal(account.isSold, false);

    // 3. Llamada a 'buy_part'
    await program.methods
      .buyPart()
      .accounts({
        partAccount: partAccount.publicKey,
        buyer: buyer.publicKey,
        sellerAccount: provider.wallet.publicKey, // En este test, el provider es el vendedor
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // 4. Verificar que ahora está vendida
    account = await program.account.sparePart.fetch(partAccount.publicKey);
    assert.equal(account.isSold, true);
  });
});