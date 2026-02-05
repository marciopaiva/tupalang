# Issues iniciais sugeridas

Quer começar? Aqui estão ótimos pontos de entrada.

## Prioridade P0
- Lexer básico
  - Tokens: identificadores, literais, operadores, delimitadores
  - Teste: fixtures + snapshots
- Parser de funções
  - `fn name(args) -> type { ... }`
  - Teste: AST mínima e erros localizados

## Prioridade P1
- AST mínima (`fn`, `let`, `return`, `call`)
- Type checker primitivo (inferência simples e verificação de tipo)
- CLI `tupa-cli` (`build` e `run`)
- Exemplo `examples/hello.tp`

## Documentação de erros
- Padrão E#### com mensagem, arquivo:linha:coluna e sugestão
- Saída `--json` para integração com ferramentas
