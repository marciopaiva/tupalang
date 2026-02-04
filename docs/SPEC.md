# SPEC

Especificação técnica inicial do projeto Tupã.

## Status

Documento em evolução.

## Objetivos

- Definir sintaxe básica.
- Descrever tipos primitivos e compostos.
- Estabelecer regras de compilação e segurança.

## Léxico (rascunho)

- Identificadores: `[_a-zA-Z][_a-zA-Z0-9]*`
- Comentários: `//` até o fim da linha
- Literais: números, strings, booleanos

## Tipos

### Primitivos

- `i64`, `f64`, `bool`, `string`

### Compostos

- `Tensor<T, shape=[...], density=...>`
- `Safe<T, ...constraints>`
- `SafeText<!constraint>`

## Funções

Assinatura básica:

```
fn nome(arg: Tipo) -> Tipo {
	// corpo
}
```

## Diferenciabilidade

- Funções puras podem ser derivadas automaticamente.
- Operador de derivada: `∇`.

## Alignment Types (rascunho)

- Restrições éticas e de segurança expressas no tipo.
- Exemplo: `SafeText<!misinformation>`.

## Esparsidade

- A densidade faz parte do tipo de tensor.
- Seleção automática de kernels sparsos.

## Erros e diagnósticos

- Erros de tipo e restrições devem ser emitidos em compile-time.
- Mensagens devem apontar restrições violadas.
