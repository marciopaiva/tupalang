# Glossário

## Propósito

Definir termos-chave usados na linguagem e na documentação.

## Termos

- **Alinhamento**: conjunto de restrições éticas verificadas em tempo de compilação.
- **Built-in Function**: funções pré-definidas no namespace `tupa::` (`weighted`, `warn`, `pass`, `confirm`, `cooldown`).
- **Config DSL**: blocos de configuração declarativa (`config Nome { tipo campo, ... }`) como nós AST de primeira classe para pré-condições de pipeline.
- **Restrição**: condição que deve ser provada para um tipo `Safe<T, ...>` (por exemplo `!nan`, `!hate_speech`).
- **Densidade**: parâmetro de tipo que controla a esparsidade de tensores.
- **EBNF**: notação formal para gramática de linguagens.
- **Extension**: funções de passo personalizadas registradas via trait `TupaExtension` ou sistema de plugins.
- **Hot Reload**: recarregamento automático de pipeline na mudança de arquivo via observação de arquivos com `notify`.
- **Nabla (`∇`)**: operador nativo de diferenciabilidade.
- **Plugin**: biblioteca compartilhada (`.so`/`.dll`) carregada dinamicamente que exporta funções de passo via entry points C.
- **Schema Registry**: armazenamento versionado de schemas com suporte a migrações para evolução de inputs/outputs de pipeline.
- **Tipo Safe**: tipo anotado com restrições, por exemplo `Safe<f64, !nan>` ou `Safe<string, !misinformation>`.
- **Span**: intervalo de texto usado para apontar erros (linha/coluna).
- **Verificador de tipos**: verificador de tipos estático do compilador.
