#!/bin/bash
# Genera los instaladores de la aplicación para Linux (.deb, AppImage)
cd "$(dirname "$0")"

echo "📦 Iniciando la generación del instalador..."

# Asegurarse de que las dependencias del frontend estén instaladas
# npm install

# Aumentar el límite de archivos abiertos para evitar errores durante el bundling
ulimit -n 65536

# Ejecutar el build de Tauri
npm run tauri build

if [ $? -eq 0 ]; then
    echo "✅ Instaladores generados con éxito en src-tauri/target/release/bundle/"
else
    echo "❌ Error al generar los instaladores."
    exit 1
fi
