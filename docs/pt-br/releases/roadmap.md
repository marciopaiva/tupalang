# Roadmap

## Propósito

Resumir o plano de evolução do projeto.

## Curto prazo

- v0.8.1: fortalecer o suporte a estratégias de trading para sistemas reais de política.
- Adicionar outputs estruturados por step e `reason` de primeira classe.
- Adicionar predicados reutilizáveis para composição de estratégia.
- Adicionar suporte a score ponderado para avaliação de política.
- Adicionar primitivas declarativas para confirmação e cooldown.

- Consolidar a SPEC v0.1 (ajustes finos e exemplos validados).
- Melhorar o typechecker (restrições e diagnósticos).
- Estabilizar o IR textual do codegen e as saídas do CLI.
- Expandir exemplos safe e goldens negativos.

## Médio prazo

- Linguagem de pipeline MVP com execução determinística.
- Primitivas de auditoria e hashing para reprodutibilidade.
- Integração Python controlada e auditável (PyTorch/TensorFlow).
- Formatador oficial e linter mínimo.
- Language server básico.

## Longo prazo

- FFI com C/Rust.
- ABI documentada.
- Benchmarks públicos.
- Ferramentas de nível enterprise e fluxos de compliance.
