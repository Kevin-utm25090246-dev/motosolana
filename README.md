✨ Características Principales
Inventario On-Chain: Registro de piezas (marca, modelo, precio) directamente en la blockchain de Solana.

Pagos en SOL: Transacciones rápidas y seguras entre cliente y taller.

Transparencia: Historial de ventas inmutable para evitar fraudes en piezas usadas o de alto rendimiento.

Escalabilidad: Construido sobre el framework Anchor para máxima eficiencia.

🛠️ Stack Tecnológico
Smart Contracts: Anchor Framework (Rust)

Blockchain: Solana (Devnet/Mainnet)

Frontend: React / Next.js (próximamente)

Wallet: Phantom o Solflare

🚀 Instalación y Configuración
Sigue estos pasos para clonar el repositorio y probar el programa localmente.

1. Prerrequisitos
Asegúrate de tener instalado:

Rust

Solana CLI

Anchor CLI

2. Clonar el repositorio
Bash
git clone https://github.com/tu-usuario/solana-autohub.git
cd solana-autohub
3. Compilar el programa
Bash
anchor build
4. Ejecutar los Tests
Para verificar que la lógica de listado y compra funciona correctamente:

Bash
anchor test
📂 Estructura del Proyecto
programs/solana_autohub/src/lib.rs: Contiene el Smart Contract principal en Rust.

tests/solana_autohub.ts: Scripts de prueba en TypeScript para validar el flujo de negocio.

app/: Directorio sugerido para la interfaz de usuario (Client-side).

📝 Roadmap
[x] Contrato inteligente básico (Listar y Comprar).

[ ] Implementar NFTs como certificados de autenticidad para piezas de lujo.

[ ] Creación de un Dashboard para el mecánico (Gestión de órdenes).

[ ] Integración con USDC para precios estables.

🤝 Contribuciones
¡Las contribuciones son bienvenidas! Si tienes una idea para mejorar el sistema de inventario o la integración con sensores IoT para carros, por favor abre un Issue o envía un Pull Request.

📄 Licencia
Este proyecto está bajo la licencia MIT.
