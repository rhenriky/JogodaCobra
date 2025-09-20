# 🐍 Snake Game - Rust

Um jogo clássico da Cobrinha implementado em Rust usando a biblioteca `crossterm` para interface de terminal.

## ✨ Funcionalidades

- 🎮 Controles responsivos (WASD ou setas direcionais)
- 🍎 Sistema de pontuação com maçãs que valem pontos aleatórios
- ⚡ Velocidade progressiva - fica mais rápido a cada maçã consumida
- 📦 Obstáculos aleatórios (caixas) que aumentam o desafio
- 🧱 Paredes sólidas - colidir com as bordas resulta em game over
- 🔄 Sistema de restart - pressione R para jogar novamente
- 🎯 Interface colorida e intuitiva

## 🚀 Como Executar

### Pré-requisitos
- Rust instalado ([rustup.rs](https://rustup.rs/))

### Executar o jogo
```bash
# Clonar o repositório
git clone <seu-repositorio>
cd snake_game

# Compilar e executar
cargo run

# Ou compilar otimizado
cargo build --release
./target/release/snake_game.exe  # Windows
```

## 🎮 Como Jogar

### Controles
- **WASD** ou **Setas direcionais**: Mover a cobra
- **Q**: Sair do jogo
- **R**: Reiniciar após game over

### Elementos do Jogo
- **O/o**: Cobra (cabeça/corpo)
- **@**: Maçã (1-5 pontos, aumenta velocidade)
- **■**: Obstáculos (caixas vermelhas - evite!)
- **Bordas**: Paredes sólidas

### Objetivo
- Coma as maçãs para aumentar sua pontuação
- Evite colidir com obstáculos, paredes ou seu próprio corpo
- O jogo fica progressivamente mais rápido e desafiador!

## 🛠️ Tecnologias

- **Linguagem**: Rust
- **Interface**: Terminal (crossterm)
- **Geração aleatória**: rand crate

## 📦 Dependências

```toml
[dependencies]
crossterm = "0.27"
rand = "0.8"
```

## 🎯 Características Técnicas

- Renderização eficiente no terminal
- Sistema de colisão preciso
- Geração procedural de obstáculos
- Controle de velocidade dinâmico
- Gerenciamento de estado do jogo robusto

## 🔄 Histórico de Versões

- **v1.0**: Versão inicial com funcionalidades básicas
- **v1.1**: Adicionado sistema de velocidade progressiva
- **v1.2**: Implementados obstáculos aleatórios
- **v1.3**: Paredes sólidas e sistema de restart

---
Desenvolvido com ❤️ em Rust