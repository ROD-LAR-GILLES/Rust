# Capítulo 1 – Primeros Pasos
Contenido del capítulo 1 (Hello, World!, rustc, introducción a Cargo).

# Capítulo 1 – Primeros Pasos

## Instalación

El primer paso es instalar Rust. La forma recomendada de hacerlo es mediante `rustup`, una herramienta para administrar versiones de Rust y herramientas asociadas.

### Notación de Línea de Comandos

A lo largo de este libro, las líneas que empiezan con `$` indican comandos que debes ingresar en tu terminal o shell. No escribas el carácter `$` como parte del comando.

### Instalación de rustup en Linux o macOS

Abre una terminal e ingresa:

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Sigue las instrucciones en pantalla para proceder con la instalación. Cuando termine, reinicia tu terminal y asegúrate de que Rust esté instalado escribiendo:

```bash
$ rustc --version
```

Deberías ver un número de versión, por ejemplo: `rustc 1.70.0 (90c541806 2023-05-31)`

### Instalación de rustup en Windows

Descarga y ejecuta [rustup-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe). Sigue las instrucciones en pantalla. Después de la instalación, abre una nueva ventana de Command Prompt y verifica tu instalación:

```bash
C:\Users\YourName> rustc --version
```

Deberías ver un número de versión.

### Solución de Problemas

Si tienes problemas con tu variable de entorno `PATH`, consulta la [guía de solución de problemas de instalación de Rust](https://www.rust-lang.org/tools/install).

### Actualización y Desinstalación

Para actualizar Rust:

```bash
$ rustup update
```

Para desinstalar Rust:

```bash
$ rustup self uninstall
```

### Documentación Local

Puedes instalar la documentación local de Rust:

```bash
$ rustup component add rust-docs
$ rustup docs --std
```

Esto abrirá la documentación de la biblioteca estándar en tu navegador.

### Editores de Texto e IDEs

Rust funciona bien con muchos editores y entornos de desarrollo (IDEs). La extensión recomendada para Visual Studio Code es "rust-analyzer". Para otros editores, consulta [la página oficial de soporte de editores](https://www.rust-lang.org/tools).

### Trabajo sin Conexión

Una vez instalado, Rust y su documentación pueden ser utilizados sin conexión a Internet.

---

## ¡Hola, Mundo!

Vamos a escribir el tradicional programa “Hello, world!” en Rust.

### Creando un Directorio de Proyecto

Crea un nuevo directorio para tu proyecto y entra en él:

```bash
$ mkdir hello_world
$ cd hello_world
```

### Escribiendo y Ejecutando un Programa en Rust

Crea un archivo llamado `main.rs` con el siguiente contenido:

```rust
fn main() {
    println!("Hello, world!");
}
```

Para compilar y ejecutar el programa, usa:

```bash
$ rustc main.rs
$ ./main
```

Deberías ver:

```
Hello, world!
```

### Anatomía de un Programa en Rust

- `fn main() { ... }` define el punto de entrada.
- `println!` imprime texto en la consola. El `!` indica que es una macro.
- Las sentencias terminan con punto y coma.

### Compilar y Ejecutar por Separado

`rustc main.rs` produce un ejecutable binario (`main` o `main.exe` en Windows) en el mismo directorio.

---

## ¡Hola, Cargo!

El sistema de construcción oficial y administrador de paquetes de Rust se llama Cargo. Se encarga de compilar, ejecutar y gestionar dependencias.

### Crear un Proyecto con Cargo

Para crear un nuevo proyecto:

```bash
$ cargo new hello_cargo
$ cd hello_cargo
```

Verás una estructura de directorios como:

```
hello_cargo
├── Cargo.toml
└── src
    └── main.rs
```

- `Cargo.toml` es el archivo de manifiesto para dependencias y metadatos.
- `src/main.rs` contiene el código fuente principal.

### Compilar y Ejecutar un Proyecto Cargo

Para compilar el proyecto:

```bash
$ cargo build
```

Para ejecutar el proyecto:

```bash
$ cargo run
```

Esto compilará y ejecutará tu programa, mostrando:

```
   Compiling hello_cargo v0.1.0 (path/to/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.23s
     Running `target/debug/hello_cargo`
Hello, world!
```

Para revisar el código sin generar un binario:

```bash
$ cargo check
```

### Compilar para Producción

Para compilar una versión optimizada para producción:

```bash
$ cargo build --release
```

El ejecutable estará en `target/release/`.

### Cargo como Convención

La mayoría de los proyectos en Rust utilizan Cargo. Simplifica la configuración del proyecto, la gestión de dependencias y la compilación.

### Resumen

- Instala Rust usando `rustup`.
- Escribe y ejecuta un programa simple “Hello, world!”.
- Usa Cargo para administrar proyectos.
- Explora la estructura de directorios y los comandos básicos.