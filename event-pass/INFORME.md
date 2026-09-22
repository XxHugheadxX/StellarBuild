# Event Pass — Smart Contract en Soroban (Stellar Testnet)

**Programa:** Stellar Elite Bolivia — Entregable jueves 17 sep.
**Track elegido:** 🎟️ Event Pass — el ledger verifica que una address compró su pase y lo usó una sola vez.

---

## 1. Resumen

Contrato Soroban que gestiona pases de evento por address:
- Un usuario **compra** un pase (`buy_pass`).
- Puede **consultar** si tiene pase (`has_pass`) y si ya lo usó (`is_used`).
- Puede **usar** el pase una única vez (`use_pass`); un segundo intento es rechazado por el contrato.

## 2. Datos del despliegue (Stellar Testnet)

| Campo | Valor |
|---|---|
| Contract ID | `CDMUF3ZCB5Q7WXHDYJ63JZD6KFL2BN5W6MKE52SR2KR5AEDMGKREDX64` |
| Wasm Hash | `874a401e615f086365be37efedee757e92a1f535e0f7b3f9d8bf0a6fae69dc72` |
| Red | Stellar Testnet |
| Address de prueba (usuario) | `GDMU5KIXBJGLCA7WU63UD77OAN4ONK7O3RCRXZCHYKQGA2DY2V6GR2H4` |
| Identidad CLI usada | `william` |

**Links al Explorer (stellar.expert / testnet):**
- Contrato: https://stellar.expert/explorer/testnet/contract/CDMUF3ZCB5Q7WXHDYJ63JZD6KFL2BN5W6MKE52SR2KR5AEDMGKREDX64
- Deploy tx: https://stellar.expert/explorer/testnet/tx/601b2d865295662f9038a7019660e4bd22c90e14420d6d2273cbd346b986ae48
- `buy_pass` tx: https://stellar.expert/explorer/testnet/tx/2f222f71a78f27d69c99b1f96a41db0c9e411be39a9113e090698eb515b6f351
- `use_pass` tx (exitosa): https://stellar.expert/explorer/testnet/tx/b5c33ca34a68064a571d24692f7dcb7787d9d1c9d8cb2f8232ef6b7501c9ab8f

## 3. Funciones del contrato

```rust
pub fn buy_pass(env: Env, user: Address);   // registra pass=true, used=false
pub fn has_pass(env: Env, user: Address) -> bool;
pub fn is_used(env: Env, user: Address) -> bool;
pub fn use_pass(env: Env, user: Address);   // falla si no tiene pase o si ya fue usado
```

Reglas de `use_pass`:
1. Si `has_pass == false` → `panic!("No pass")`.
2. Si `used == true` → `panic!("Pass already used")`.
3. Si ambas pasan → `used = true`.

## 4. Evidencia de ejecución (flujo real en Testnet)

| Paso | Comando | Resultado |
|---|---|---|
| 1 | `buy_pass --user GDMU...` | ✅ Transaction submitted |
| 2 | `has_pass --user GDMU...` | `true` |
| 3 | `is_used --user GDMU...` | `false` |
| 4 | `use_pass --user GDMU...` | ✅ Transaction submitted |
| 5 | `is_used --user GDMU...` | `true` |
| 6 | `use_pass --user GDMU...` (2do intento) | ❌ Rechazado: `HostError: Error(WasmVm, InvalidAction)` → panic `Pass already used` |

Esto demuestra que la regla "un pase, un solo uso" se cumple **dentro del propio contrato**, no solo en la lógica de una app externa.

## 5. Pruebas locales (`cargo test`)

3 tests, todos ok:
- `test_event_pass` — ciclo completo (compra → tiene pase → no usado → usa → usado).
- `test_no_se_puede_usar_dos_veces` — `#[should_panic(expected = "Pass already used")]`.
- `test_sin_pase_no_puede_usar` — `#[should_panic(expected = "No pass")]`.

```
running 3 tests
test test::test_sin_pase_no_puede_usar - should panic ... ok
test test::test_event_pass ... ok
test test::test_no_se_puede_usar_dos_veces - should panic ... ok

test result: ok. 3 passed; 0 failed
```

## 6. Estructura del proyecto

```
stellar-event-pass/
├── Cargo.toml
└── contracts/
    └── hello-world/
        ├── Cargo.toml
        └── src/
            ├── lib.rs     (contrato EventPass)
            └── test.rs    (3 tests)
```

## 7. Comandos usados (para reproducir / grabar el video)

```bash
# Build
stellar contract build

# Deploy
stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello_world.wasm \
  --source-account william --network testnet --alias event-pass

# Comprar pase
stellar contract invoke --id CDMUF3ZCB5Q7WXHDYJ63JZD6KFL2BN5W6MKE52SR2KR5AEDMGKREDX64 \
  --source-account william --network testnet --send=yes \
  -- buy_pass --user GDMU5KIXBJGLCA7WU63UD77OAN4ONK7O3RCRXZCHYKQGA2DY2V6GR2H4

# Consultar estado
stellar contract invoke --id CDMUF3ZCB5Q7WXHDYJ63JZD6KFL2BN5W6MKE52SR2KR5AEDMGKREDX64 \
  --source-account william --network testnet \
  -- has_pass --user GDMU5KIXBJGLCA7WU63UD77OAN4ONK7O3RCRXZCHYKQGA2DY2V6GR2H4

stellar contract invoke --id CDMUF3ZCB5Q7WXHDYJ63JZD6KFL2BN5W6MKE52SR2KR5AEDMGKREDX64 \
  --source-account william --network testnet \
  -- is_used --user GDMU5KIXBJGLCA7WU63UD77OAN4ONK7O3RCRXZCHYKQGA2DY2V6GR2H4

# Usar pase
stellar contract invoke --id CDMUF3ZCB5Q7WXHDYJ63JZD6KFL2BN5W6MKE52SR2KR5AEDMGKREDX64 \
  --source-account william --network testnet --send=yes \
  -- use_pass --user GDMU5KIXBJGLCA7WU63UD77OAN4ONK7O3RCRXZCHYKQGA2DY2V6GR2H4

# Segundo intento (debe fallar)
stellar contract invoke --id CDMUF3ZCB5Q7WXHDYJ63JZD6KFL2BN5W6MKE52SR2KR5AEDMGKREDX64 \
  --source-account william --network testnet --send=yes \
  -- use_pass --user GDMU5KIXBJGLCA7WU63UD77OAN4ONK7O3RCRXZCHYKQGA2DY2V6GR2H4
```

## 8. Para el front (referencia rápida para integrar con wallet / Freighter)

- **Network:** Testnet (`https://soroban-testnet.stellar.org` RPC, passphrase `Test SDF Network ; September 2015`).
- **Contract ID:** `CDMUF3ZCB5Q7WXHDYJ63JZD6KFL2BN5W6MKE52SR2KR5AEDMGKREDX64`
- **Métodos a invocar desde el front:**
  - `buy_pass(user: Address)` — requiere firma del `user` (`require_auth`).
  - `has_pass(user: Address) -> bool` — solo lectura, no requiere firma.
  - `is_used(user: Address) -> bool` — solo lectura, no requiere firma.
  - `use_pass(user: Address)` — requiere firma del `user` (`require_auth`).
- **Flujo de UI sugerido:**
  1. Conectar wallet (Freighter) → obtener address.
  2. Botón "Comprar pase" → `buy_pass` → refrescar `has_pass`/`is_used`.
  3. Mostrar estado: "Tienes pase" / "Ya usado" según `has_pass` / `is_used`.
  4. Botón "Usar pase" → `use_pass`; si falla, mostrar mensaje "Pase ya utilizado" o "No tienes pase".

## 9. Reflexión — próximos pasos de aprendizaje

Lo siguiente que toca profundizar: seguridad de contratos (control de acceso, reentrancia en cross-contract calls), manejo y expiración de storage (persistent vs. temporary, TTL/rent), eventos (`env.events().publish`) para que el front pueda escuchar cambios de estado sin polling, integración con Freighter/wallet desde el front, y pruebas de integración contra Testnet (no solo unitarias).

---
**Estado final demostrado:** `has_pass = true`, `is_used = true`, segundo `use_pass` rechazado. ✅
