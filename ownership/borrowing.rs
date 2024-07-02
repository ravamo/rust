```rust
// Definimos el struct Estudiante con tres campos:
// nombre: un String que representa el nombre del estudiante
// horas: un entero que representa el número de horas que ha estudiado
// nota: un float que representa la nota del estudiante
struct Estudiante {
    nombre: String,
    horas: i32,
    nota: f64,
}

// Función para calcular la nota media de un slice de estudiantes
// La función toma una referencia al slice de estudiantes, asegurando que no se muevan ni se copien
fn calcular_nota_media(estudiantes: &[Estudiante]) -> f64 {
    // Comprobamos si el slice está vacío para evitar una división por cero
    if estudiantes.is_empty() {
        return 0.0;
    }

    // Sumamos todas las notas de los estudiantes
    let suma_notas: f64 = estudiantes.iter().map(|est| est.nota).sum();
    // Calculamos la media dividiendo la suma de las notas por el número de estudiantes
    suma_notas / estudiantes.len() as f64
}

fn main() {
    // Creamos un vector de estudiantes
    let estudiantes = vec![
        Estudiante {
            nombre: String::from("Alice"),
            horas: 10,
            nota: 85.0,
        },
        Estudiante {
            nombre: String::from("Bob"),
            horas: 15,
            nota: 90.0,
        },
        Estudiante {
            nombre: String::from("Charlie"),
            horas: 8,
            nota: 78.0,
        },
    ];

    // Llamamos a la función calcular_nota_media pasando una referencia al vector de estudiantes
    let nota_media = calcular_nota_media(&estudiantes);

    // Imprimimos la nota media
    println!("La nota media de los estudiantes es: {}", nota_media);
}
```

### Explicación del Código:

1. **Definición del Struct `Estudiante`**:
    - `nombre`: Almacena el nombre del estudiante como un `String`.
    - `horas`: Almacena el número de horas que el estudiante ha estudiado como un `i32`.
    - `nota`: Almacena la nota del estudiante como un `f64`.

2. **Función `calcular_nota_media`**:
    - Toma una referencia inmutable a un slice de `Estudiante` (`&[Estudiante]`), evitando mover o copiar los datos.
    - Comprueba si el slice está vacío para evitar una división por cero.
    - Usa `iter()` para iterar sobre los estudiantes y `map()` para extraer las notas, que luego se suman.
    - Calcula la media dividiendo la suma de las notas por el número de estudiantes.

3. **Función `main`**:
    - Crea un vector de `Estudiante` con algunos datos de ejemplo.
    - Llama a `calcular_nota_media`, pasando una referencia al vector de estudiantes.
    - Imprime la nota media calculada.

### Comentarios y Notas:

- **Borrowing**: En Rust, cuando pasamos `&estudiantes` a `calcular_nota_media`, estamos prestando (borrowing) una referencia inmutable del vector de estudiantes, permitiendo que la función lo lea pero no lo modifique.
- **Iteradores y `map()`**: Usar iteradores y el método `map()` es una forma idiomática y eficiente de procesar colecciones en Rust.
- **Error Handling**: La función maneja el caso de un slice vacío devolviendo `0.0`, una simple estrategia de manejo de errores en este contexto.