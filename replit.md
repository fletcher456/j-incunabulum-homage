# Overview

This repository contains a multi-component system featuring three different server implementations and a J programming language interpreter. The primary focus is on the J language interpreter with a web-based calculator interface, alongside experimental C and simplified Rust servers. The project demonstrates parsing, semantic analysis, and evaluation of J language expressions through a browser-based interface.

# System Architecture

## Multi-Server Architecture
The system is designed to run three parallel servers:
1. **C Webserver**: Basic HTTP server written in C (port 5000)
2. **J Web REPL**: J language interpreter with web interface (port 5000) 
3. **Simple Server**: Rust-based HTTP server with J language processing (port 5000)

## J Language Interpreter Pipeline
The core J interpreter follows a 4-phase processing pipeline:
- **Tokenization**: Raw J code → Tokens (numbers, verbs, vectors)
- **Parsing**: Tokens → Abstract Syntax Tree using LALRPOP parser generator
- **Semantic Analysis**: AST → Resolved AST (monadic/dyadic verb resolution)
- **Evaluation**: Resolved AST → Results (arrays/scalars)

## Web Interface Architecture
- **Frontend**: HTML/CSS/JavaScript calculator with button-based input
- **Backend**: Rust HTTP server serving static files and `/j_eval` endpoint
- **Communication**: JSON-based request/response for expression evaluation

# Key Components

## Core J Language Modules
- **`tokenizer.rs`**: Lexical analysis converting strings to token streams
- **`j_grammar.lalrpop`**: LALR(1) grammar specification for J language syntax
- **`semantic_analyzer.rs`**: Context resolution for monadic vs dyadic verb interpretation
- **`evaluator.rs`**: Expression evaluation and array operations
- **`j_array.rs`**: Multi-dimensional array data structure with display formatting

## Parser Generator Integration
- **LALRPOP**: Used for generating robust, precedence-aware parsers
- **Build Process**: Grammar compilation happens at build time via `build.rs`
- **Context Handling**: Addresses J's right-to-left evaluation and operator precedence

## Web Server Components
- **`main.rs`**: HTTP server setup, routing, and static file serving
- **`j_repl.html`**: Calculator interface with matrix display capabilities
- **Request Handling**: POST `/j_eval` endpoint for expression processing

# Data Flow

## Expression Processing Flow
1. **User Input**: Button clicks or direct input in web interface
2. **HTTP Request**: JSON payload sent to `/j_eval` endpoint
3. **Tokenization**: Input string parsed into J language tokens
4. **LALRPOP Parsing**: Tokens processed through generated parser to create AST
5. **Semantic Resolution**: AST analyzed to resolve verb contexts (monadic/dyadic)
6. **Evaluation**: Resolved AST executed to produce results
7. **Response**: Results formatted and returned as JSON
8. **Display**: Web interface renders results with proper matrix formatting

## Array Operations Flow
- **Creation**: Numbers and vectors parsed into `JArray` structures
- **Operations**: Monadic (iota, identity) and dyadic (plus, reshape) verb processing
- **Formatting**: Multi-dimensional array display with aligned columns
- **Memory Management**: Rust ownership ensures safe array manipulation

# External Dependencies

## Build Dependencies
- **LALRPOP**: Parser generator for LALR(1) grammars
- **Cargo Build System**: Rust compilation and dependency management

## Runtime Dependencies  
- **tiny_http**: Lightweight HTTP server for Rust backend
- **lalrpop-util**: Runtime support for generated parsers

## Development Tools
- **Nix Package Manager**: Environment setup with Rust and C toolchains
- **Multi-language Support**: C compiler (gcc) and Rust (cargo) in unified environment

# Deployment Strategy

## Current Deployment
- **Replit Environment**: Cloud-based development with parallel server execution
- **Port Management**: Multiple servers configured for port 5000 with workflow coordination
- **Hot Reloading**: Development workflow supports rapid iteration and testing

## Potential WASM Deployment
Documentation indicates exploration of WebAssembly compilation for client-side execution, eliminating server dependency and enabling single-file browser deployment.

## Build Process
- **Multi-stage Compilation**: LALRPOP grammar generation followed by Rust compilation
- **Incremental Builds**: Cargo manages dependency compilation and caching
- **Cross-platform**: Nix environment ensures consistent builds across platforms

# Changelog

Changelog:
- June 23, 2025. Initial setup

# User Preferences

Preferred communication style: Simple, everyday language.