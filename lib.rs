import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
// Client.ts - Interactuando con tu negocio de motos
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MotoBusiness } from "../target/types/moto_business";
import type { MotoBusiness } from "../target/types/moto_business";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.MotoBusiness as anchor.Program<MotoBusiness>;


// Configuración automática del proveedor de SolPG
const pg = anchor.AnchorProvider.local();
anchor.setProvider(pg);
const program = anchor.workspace.MotoBusiness as Program<MotoBusiness>;

async function main() {
  // 1. Generar una cuenta única para nuestra moto
  const motoKeypair = anchor.web3.Keypair.generate();
  
  console.log("🚀 Agregando una nueva moto al inventario...");

  // 2. Llamar a la función 'agregar_moto'
  await program.methods
    .agregarMoto("Kawasaki Ninja 400", new anchor.BN(150000000)) // ~0.15 SOL en Lamports
    .accounts({
      moto: motoKeypair.publicKey,
      autor: program.provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([motoKeypair])
    .rpc();

  console.log("✅ Moto agregada con éxito!");
  console.log("📍 Dirección de la moto:", motoKeypair.publicKey.toBase58());

  // 3. Consultar los datos de la moto en la blockchain
  let motoData = await program.account.moto.fetch(motoKeypair.publicKey);
  console.log(`🏍️ Modelo: ${motoData.modelo}`);
  console.log(`💰 Precio: ${motoData.precio.toString()} Lamports`);
  console.log(`🏷️ ¿En venta?: ${motoData.enVenta ? "SÍ" : "NO"}`);

  // 4. Marcar como vendida
  console.log("\n🛒 Marcando como vendida...");
  await program.methods
    .marcarComoVendida()
    .accounts({
      moto: motoKeypair.publicKey,
      dueno: program.provider.publicKey,
    })
    .rpc();

  // 5. Verificar estado final
  motoData = await program.account.moto.fetch(motoKeypair.publicKey);
  console.log(`🏷️ ¿En venta ahora?: ${motoData.enVenta ? "SÍ" : "NO"}`);
}

main().catch((err) => {
  console.error("❌ Error:", err);
});