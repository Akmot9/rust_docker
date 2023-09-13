# Utilisez l'image officielle de Rust comme base
FROM rust:1.72 AS builder

# Créez un nouveau répertoire appelé `/usr/src/hello_world`
WORKDIR /usr/src/hello_world

# Copiez le contenu du répertoire actuel dans `/usr/src/hello_world`
COPY . .

# Compilez le code Rust
RUN cargo build --release

# Utilisez une image plus petite pour le binaire
FROM ubuntu:latest

# Créez un répertoire pour le binaire
WORKDIR /usr/local/bin

# Copiez le binaire du builder à l'image actuelle
COPY --from=builder /usr/src/hello_world/target/release/hello_world .

# Commande pour exécuter le binaire
CMD ["./hello_world"]
