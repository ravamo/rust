### Ejemplo de Ownership

Vamos a crear un programa sencillo que maneje el ownership de un `String` en Rust.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 se mueve a s2, s1 ya no es válido
    println!("{}", s2);  // Esto funciona
    // println!("{}", s1);  // Esto causaría un error de compilación
}
```

### Paso a Paso:

1. **Crear un `String`:**
    ```rust
    let s1 = String::from("hello");
    ```
    - Aquí creamos un `String` y lo almacenamos en la variable `s1`.
    - `s1` es el propietario del `String`, lo que significa que es responsable de liberar la memoria cuando ya no se necesita.

2. **Mover el `String`:**
    ```rust
    let s2 = s1;
    ```
    - En este punto, el ownership del `String` se transfiere de `s1` a `s2`.
    - Después de esta línea, `s1` ya no es válido y no puede ser usado. Solo `s2` es el propietario del `String`.

3. **Imprimir el `String`:**
    ```rust
    println!("{}", s2);  // Esto funciona
    ```
    - Esto funciona correctamente porque `s2` es el propietario actual del `String`.

4. **Intentar Usar `s1` (Error de Compilación):**
    ```rust
    // println!("{}", s1);  // Esto causaría un error de compilación
    ```
    - Si descomentas esta línea, el compilador generará un error porque `s1` ya no es válido después de que su ownership fue transferido a `s2`.

### Explicación de Ownership:

- **Ownership Transferido:**
  - Cuando el `String` `s1` se asigna a `s2`, el ownership se transfiere de `s1` a `s2`. Esto significa que `s2` ahora es responsable de liberar la memoria del `String`.

- **Reglas de Ownership:**
  - **Cada valor en Rust tiene un único propietario.**
  - **Cuando el propietario sale del alcance, el valor se limpia.**
  - **No puede haber dos variables que posean el mismo valor simultáneamente.**

### Ejemplo de Transferencia de Ownership con Funciones:

A continuación, un ejemplo que muestra cómo funciona el ownership cuando se pasan variables a funciones:

```rust
fn main() {
    let s1 = String::from("hello");
    tomar_ownership(s1);  // s1 se mueve a la función, ya no es válido aquí
    // println!("{}", s1);  // Esto causaría un error de compilación

    let x = 5;
    hacer_copia(x);  // x es un tipo Copy, todavía es válido aquí
    println!("{}", x);  // Esto funciona
}

fn tomar_ownership(s: String) {
    println!("{}", s);
}

fn hacer_copia(n: i32) {
    println!("{}", n);
}
```

### Paso a Paso:

1. **Crear un `String`:**
    ```rust
    let s1 = String::from("hello");
    ```

2. **Pasar el `String` a una Función:**
    ```rust
    tomar_ownership(s1);  // s1 se mueve a la función, ya no es válido aquí
    ```
    - Aquí, `s1` se pasa a la función `tomar_ownership`.
    - El ownership de `s1` se transfiere a la función, y `s1` ya no es válido en `main`.

3. **Intentar Usar `s1` (Error de Compilación):**
    ```rust
    // println!("{}", s1);  // Esto causaría un error de compilación
    ```

4. **Crear un `i32`:**
    ```rust
    let x = 5;
    ```

5. **Pasar el `i32` a una Función:**
    ```rust
    hacer_copia(x);  // x es un tipo Copy, todavía es válido aquí
    ```
    - Aquí, `x` se pasa a la función `hacer_copia`.
    - `i32` es un tipo que implementa el trait `Copy`, por lo que `x` no se mueve, sino que se copia.
    - `x` sigue siendo válido en `main`.

6. **Usar `x`:**
    ```rust
    println!("{}", x);  // Esto funciona
    ```

### Explicación de Ownership con Funciones:

- **Transferencia de Ownership:**
  - Cuando se pasa un `String` a una función, el ownership se transfiere a la función.
  - La variable original ya no es válida después de la transferencia de ownership.

- **Tipos que Implementan `Copy`:**
  - Para tipos simples como `i32` que implementan el trait `Copy`, pasar la variable a una función no transfiere el ownership, sino que hace una copia.
  - La variable original sigue siendo válida después de la llamada a la función.