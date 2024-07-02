# Nombre del binario o el archivo de salida
BINARY_NAME = mi_proyecto

# El target (objetivo) para la compilación cruzada
TARGET = aarch64-unknown-linux-gnu

# Linker para la compilación cruzada
LINKER = aarch64-linux-gnu-gcc

# Comando de cargo para compilar
CARGO = cargo

# Archivo de configuración para especificar el linker
CARGO_CONFIG = .cargo/config.toml

# Crear un directorio .cargo si no existe
$(CARGO_CONFIG):
	@mkdir -p $(dir $(CARGO_CONFIG))
	@echo "[target.$(TARGET)]" > $(CARGO_CONFIG)
	@echo "linker = \"$(LINKER)\"" >> $(CARGO_CONFIG)

# Compilación por defecto
.PHONY: build
build:
	$(CARGO) build

# Compilación en modo release
.PHONY: release
release:
	$(CARGO) build --release

# Compilación cruzada para aarch64-unknown-linux-gnu
.PHONY: build-cross
build-cross: $(CARGO_CONFIG)
	$(CARGO) build --target $(TARGET)

# Compilación cruzada en modo release
.PHONY: release-cross
release-cross: $(CARGO_CONFIG)
	$(CARGO) build --release --target $(TARGET)

# Ejecutar clippy para análisis estático del código
.PHONY: clippy
clippy:
	$(CARGO) clippy -- -D warnings

# Limpiar los archivos generados por la compilación
.PHONY: clean
clean:
	$(CARGO) clean
	@rm -f $(CARGO_CONFIG)

# Mostrar ayuda
.PHONY: help
help:
	@echo "Comandos disponibles:"
	@echo "  make build           - Compilar el proyecto en modo debug"
	@echo "  make release         - Compilar el proyecto en modo release"
	@echo "  make build-cross     - Compilar el proyecto para $(TARGET)"
	@echo "  make release-cross   - Compilar el proyecto para $(TARGET) en modo release"
	@echo "  make clippy          - Ejecutar clippy para análisis estático del código"
	@echo "  make clean           - Limpiar los archivos generados por la compilación"
