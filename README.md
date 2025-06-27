# Auto Hash

`auto-hash` es una pequeña herramienta en Rust diseñada para automatizar ataques de máscara con [hashcat](https://hashcat.net/hashcat/). Permite definir varios patrones y tipos de hash a la vez, expandiendo patrones variables y mostrando los resultados generados por hashcat.

## Requisitos

- Tener **hashcat** instalado y disponible en el `PATH` del sistema.
- Rust estable para compilar el proyecto.

## Uso

Los argumentos principales se proporcionan en formato largo (`--argumento`). Los más comunes son:

- `--input-file <ruta>`: archivo con hashes a procesar.
- `--output <ruta>`: archivo donde se guardarán los hashes crackeados por hashcat.
- `--min <n>` y `--max <n>`: límites de longitud cuando se usa el parámetro `?x` en un patrón.
- `--alphabet <caracteres>`: conjunto de caracteres que se asigna a `?1` mediante `--custom-charset1` de hashcat.
- `--patterns <patrón>...`: uno o más patrones de máscara que serán enviados a hashcat.
- `--t <tipo>...`: lista de identificadores de tipo de hash (como los utilizados por hashcat en `-m`).

### ¿Cómo funciona `?x`?

Si un patrón contiene el marcador `?x`, el programa lo expandirá reemplazándolo por `?1` repetido tantas veces como indiquen `--min` y `--max`. Por ejemplo:

```
?d?x
```

con `--min 1 --max 3` se convertirá en los patrones:

```
?d?1
?d?1?1
?d?1?1?1
```

De esta forma se generan múltiples combinaciones de longitud variable sin escribir cada patrón manualmente.

## Ejemplos

```bash
# Ejecutar en modo desarrollo
cargo run -- \
    --input-file hashes.txt \
    --output resultados.txt \
    --min 1 --max 4 \
    --alphabet abc123 \
    --patterns ?d?x \
    --t 0
```

```bash
# Versión compilada
./target/release/auto-hash \
    --input-file hashes.txt \
    --output resultados.txt \
    --min 1 --max 4 \
    --alphabet abc123 \
    --patterns ?d?x \
    --t 0
```

## Compilación

Para generar la versión optimizada ejecute:

```bash
cargo build --release
```

El binario quedará en `target/release/auto-hash`. A partir de ahí puede ejecutar la aplicación pasando los argumentos antes descritos.

