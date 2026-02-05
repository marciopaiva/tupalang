# Plano de MVP

Objetivo: entregar um compilador mínimo que parseia, verifica tipos simples e gera um binário nativo para `hello.tp`.

## Escopo
- Lexer: tokens básicos, comentários `//`, strings e números
- Parser: AST para funções, let, if, match e chamadas simples
- Type Checker: tipos primitivos (`i64`, `f64`, `bool`, `string`) e inferência básica
- Codegen (LLVM): funções, aritmética e `print`
- CLI: `tupa-cli` com `build` e `run`

## Critérios de aceite
- `examples/hello.tp` compila e executa
- Erros de tipo claros, com arquivo:linha:coluna
- Sem dependência de runtime externo para o binário gerado

## Como vamos validar
- Fixtures e snapshots para lexer e parser
- Casos de tipos com sucesso e erro (E####)
- Execução do binário em CI (Linux/Windows/macOS)
