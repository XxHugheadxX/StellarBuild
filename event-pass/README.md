# Event Pass — Smart Contract en Soroban (Stellar Testnet)

Contrato inteligente para el track **Event Pass** del reto Stellar Elite Bolivia.

Lógica: el ledger verifica que una address compró su pase y garantiza que solo puede usarlo **una sola vez**. La regla vive dentro del propio contrato, no en una aplicación externa.

---

## 1. ¿Qué hace el contrato?

| Función | Descripción | Requiere firma |
|---|---|---|
| `buy_pass(user: Address)` | Registra que `user` compró un pase (`pass = true`, `used = false`) | Sí (`require_auth`) |
| `has_pass(user: Address) -> bool` | Devuelve si `user` tiene un pase registrado | No (solo lectura) |
| `is_used(user: Address) -> bool` | Devuelve si el pase de `user` ya fue utilizado | No (solo lectura) |
| `use_pass(user: Address)` | Marca el pase como usado. Falla si no tiene pase (`No pass`) o si ya fue usado (`Pass already used`) | Sí (`require_auth`) |

Estado por address, guardado en `persistent storage`:

```
DataKey::Pass(Address) -> bool
DataKey::Used(Address) -> bool
```

## 2. Despliegue en Stellar Testnet

| Campo | Valor |
|---|---|
| Contract ID | `CDMUF3ZCB5Q7WXHDYJ63JZD6KFL2BN5W6MKE52SR2KR5AEDMGKREDX64` |
| Wasm Hash | `874a401e615f086365be37efedee757e92a1f535e0f7b3f9d8bf0a6fae69dc72` |
| Red | Stellar Testnet |

Ver contrato en el explorer:
https://stellar.expert/explorer/testnet/contract/CDMUF3ZCB5Q7WXHDYJ63JZD6KFL2BN5W6MKE52SR2KR5AEDMGKREDX64

## 3. Estructura del proyecto

```
event-pass/
├── Cargo.toml                          # workspace
├── README.md
├── INFORME.md                          # informe detallado de evidencia (evidencias, tx, reflexión)
└── contracts/
    └── hello-world/
        ├── Cargo.toml
        └── src/
            ├── lib.rs                  # contrato EventPass
            └── test.rs                 # tests unitarios
```

## 4. Cómo correr las pruebas localmente

```bash
cd contracts/hello-world   # o desde la raíz del workspace
cargo test
```

Resultado esperado:

```
running 3 tests
test test::test_sin_pase_no_puede_usar - should panic ... ok
test test::test_event_pass ... ok
test test::test_no_se_puede_usar_dos_veces - should panic ... ok

test result: ok. 3 passed; 0 failed
```

Casos cubiertos:
- **Ciclo completo:** sin pase → compra → tiene pase → no usado → usa → usado.
- **Doble uso rechazado:** `#[should_panic(expected = "Pass already used")]`.
- **Uso sin pase rechazado:** `#[should_panic(expected = "No pass")]`.

## 5. Cómo compilar y desplegar desde cero

Requiere [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli) y Rust con el target `wasm32v1-none`.

```bash
# 1. Crear y financiar una identidad de despliegue en testnet
stellar keys generate mi-identidad --network testnet --fund

# 2. Compilar el contrato a WASM
stellar contract build

# 3. Desplegar en testnet
stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello_world.wasm \
  --source-account mi-identidad \
  --network testnet \
  --alias event-pass
```

El comando de deploy imprime el **Contract ID** que se usa en todas las invocaciones siguientes.

## 6. Cómo invocar el contrato (flujo completo)

Reemplaza `<CID>` por el Contract ID del deploy y `<ADDRESS>` por la address del usuario de prueba.

```bash
# Estado inicial (false)
stellar contract invoke --id <CID> --source-account mi-identidad --network testnet \
  -- has_pass --user <ADDRESS>

# Comprar el pase
stellar contract invoke --id <CID> --source-account mi-identidad --network testnet --send=yes \
  -- buy_pass --user <ADDRESS>

# Confirmar que ahora tiene pase (true)
stellar contract invoke --id <CID> --source-account mi-identidad --network testnet \
  -- has_pass --user <ADDRESS>

# Usar el pase
stellar contract invoke --id <CID> --source-account mi-identidad --network testnet --send=yes \
  -- use_pass --user <ADDRESS>

# Confirmar que quedó usado (true)
stellar contract invoke --id <CID> --source-account mi-identidad --network testnet \
  -- is_used --user <ADDRESS>

# Segundo intento de uso -> debe fallar con "Pass already used"
stellar contract invoke --id <CID> --source-account mi-identidad --network testnet --send=yes \
  -- use_pass --user <ADDRESS>
```

## 7. Evidencia registrada en Testnet

Ver el detalle completo, con hashes de transacción reales y links al explorer, en [`INFORME.md`](./INFORME.md).

Resumen del resultado demostrado:

```
has_pass = true
is_used  = true
Segundo use_pass -> rechazado (Pass already used)
```

## 8. Reglas de negocio implementadas

1. Una address sin pase no puede usar `use_pass` (`panic!("No pass")`).
2. Una address con pase puede usarlo exactamente una vez.
3. Un segundo intento de `use_pass` sobre el mismo pase siempre falla (`panic!("Pass already used")`).
4. `buy_pass` y `use_pass` requieren la firma (`require_auth`) de la address del usuario; `has_pass` e `is_used` son de solo lectura y no requieren firma.

## 9. Próximos pasos / aprendizaje pendiente

- Seguridad de contratos (control de acceso, cross-contract calls).
- Manejo de expiración de storage (persistent vs. temporary, TTL/rent).
- Eventos (`env.events().publish`) para que un frontend reaccione sin polling.
- Integración con wallet (Freighter) desde una aplicación web.
- Pruebas de integración contra Testnet, además de las unitarias.
