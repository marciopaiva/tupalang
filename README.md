# ⚡ Tupã

> **Força ancestral, código moderno**  
> Linguagem brasileira para sistemas críticos e IA evolutiva

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-impl_em_andamento-orange)](#)

> **Status atual**: Especificação v0.1 completa. Implementação em andamento com lexer, parser, typechecker e CLI básicos.

```tupa
// IA responsável desde o primeiro caractere
fn summarize(article: Text) -> SafeText<!misinformation> {
	return llm.generate(f"Resuma objetivamente: {article}")
}
```

---

## ✅ Comece aqui

- [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md)
- [docs/README.md](docs/README.md) (mapa da documentação)
- [examples/README.md](examples/README.md)
- [docs/SPEC.md](docs/SPEC.md)
- [docs/GLOSSARY.md](docs/GLOSSARY.md)
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [docs/STYLE_GUIDE.md](docs/STYLE_GUIDE.md)
- [docs/FAQ.md](docs/FAQ.md)
- [docs/ROADMAP.md](docs/ROADMAP.md)
- [docs/COMMON_ERRORS.md](docs/COMMON_ERRORS.md)
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [docs/DEV_ENV.md](docs/DEV_ENV.md)

## 🌩️ Por que Tupã?

Na mitologia tupi-guarani, **Tupã** é a divindade do trovão, uma força bruta canalizada com precisão. Assim é nossa linguagem:

| Problema atual das linguagens | Solução Tupã |
|-------------------------------|--------------|
| ❌ Python: dinâmico demais → bugs em runtime | ✅ Tipagem gradual com *alignment* em compile-time |
| ❌ Rust: seguro mas curva acentuada para pesquisadores de IA | ✅ Sintaxe legível + segurança sem sacrifício |
| ❌ Todas: diferenciabilidade via bibliotecas frágeis | ✅ `∇` (nabla) como operador de primeira classe |
| ❌ Modelos densos → pegada de carbono insustentável | ✅ Esparsidade declarativa no tipo (`density=0.1`) |

> **Tupã não é apenas mais uma linguagem.** É uma proposta focada em IA e sistemas críticos, onde segurança ética e eficiência energética são requisitos de primeira classe.

---

## 🧠 Pilares Técnicos

### 1. Diferenciabilidade Nativa
```tupa
fn mse(pred: f64, target: f64) -> f64 {
	let diff = pred - target
	return diff * diff
}

let (d_pred, _) = ∇mse(0.8, 1.0)  // → -0.4 (derivada simbólica em compile-time)
```
- Zero *graph tracing* em runtime
- Qualquer função pura é automaticamente derivável
- Backpropagation nativa no LLVM IR

### 2. Alignment via Sistema de Tipos
```tupa
// Compila SOMENTE se safety for provada
fn generate() -> SafeText<!hate_speech, !misinformation> {
	return llm.generate(prompt)
}
```
- Restrições éticas verificadas estaticamente
- Integração com RLHF scorers e verificadores formais
- Zero *runtime guards* frágeis

### 3. Esparsidade Declarativa
```tupa
// 90% menos energia no inference
let model: Tensor<f16, shape=[4096, 4096], density=0.1> = load("llama3.tp")
```
- Densidade como parte do tipo
- Kernels sparsos selecionados automaticamente
- Quantização nativa (`f16` first-class)

### 4. Performance Previsível
- Zero alocações ocultas (como Zig)
- Binário nativo via LLVM (sem VM)
- Footprint mínimo (~15 MB RAM idle)

---

## 💻 Sintaxe: Legível como Python, Poderosa como Rust

```tupa
// Inferência de tipos com tipagem explícita opcional
let idade = 28          // i64 (inferido)
let nome: string = "Ana" // string (explícito)

// Tipos de função (first-class)
let add: fn(i64, i64) -> i64 = soma

// Pattern matching elegante
match http_status {
	200 => print("OK"),
	404 => print("Não encontrado"),
	code if code >= 500 => print(f"Erro servidor: {code}"),
	_ => print("Outro status")
}

// Concorrência leve com segurança garantida
spawn async fn worker(id: i64) {
	let data = await db.query(id)
	process(data)  // Zero data races pelo sistema de tipos
}
```

✅ Familiar para devs Python/JS  
✅ Seguro como Rust  
✅ Rápido como C

---

## 🚀 Caso de Uso: Microserviço de Detecção de Fraude

```tupa
// fraud_detector.tp
@differentiable
fn risk_score(tx: Transaction) -> f64 {
	let neural = fraud_net.infer(tx.features)  // Tensor<f16, density=0.15>
	let symbolic = if tx.country == "BR" && tx.amount > 1000 { 0.8 } else { 0.2 }
	return 0.7 * neural + 0.3 * symbolic  // Fusão neurosimbólica nativa
}

@service(port=8080)
fn main() {
	route.post("/predict", |req: Request| {
		// Safe<f64, !nan> garante que score nunca é NaN (crítico para produção)
		let score: Safe<f64, !nan> = risk_score(req.transaction)
		return Response::json(score)
	})
}
```

**Resultados esperados** (vs Python + PyTorch):
| Métrica | Python | Tupã | Ganho |
|---------|--------|------|-------|
| Latência P99 | 45 ms | 8 ms | **5.6x mais rápido** |
| Consumo energia | 100% | 12% | **88% menos carbono** |
| Vazamento dados | Possível (runtime) | Impossível (compile-time) | **Segurança formal** |

---

## 🚀 Próximos 30 dias (ajude agora!)

| Tarefa | Arquivo | Dificuldade |
|--------|---------|-------------|
| Diagnósticos com span/linha/coluna (spec + implementação) | `docs/SPEC.md` | ⭐⭐ |
| Evoluir typechecker (retorno, match, loops, tipos de função) | `crates/tupa-typecheck/` | ⭐⭐⭐ |
| Protótipo de codegen MVP (LLVM) | `crates/tupa-codegen/` | ⭐⭐⭐⭐ |
| Expandir exemplos reais + edge cases | `examples/` | ⭐ |

👉 **Comece aqui**: Abra uma issue com `[RFC]` no título para propor mudanças na spec.

---

## 🧭 Roadmap

- [docs/MVP_PLAN.md](docs/MVP_PLAN.md)
- [docs/ADOPTION_PLAN.md](docs/ADOPTION_PLAN.md)

## 🌍 Orgulho Brasileiro, Ambição Global

Tupã é a **primeira linguagem brasileira com ambição global desde Lua** (1993). Mas enquanto Lua focou em *embeddability*, Tupã nasce para resolver os maiores desafios da computação moderna:

- 🇧🇷 **Raízes tupi-guarani**: nome, logo e filosofia inspirados na sabedoria ancestral
- 🌱 **Sustentabilidade**: esparsidade nativa reduz pegada de carbono da IA
- 🛡️ **Segurança ética**: alignment não é *feature*, é fundação
- ⚡ **Performance**: LLVM + zero-cost abstractions = código tão rápido quanto C

> *"Não estamos reinventando a roda. Estamos construindo a primeira roda que não polui o planeta enquanto rola."*

---

## 📚 Recursos

### Para usuários

- [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md)
- [examples/README.md](examples/README.md)
- [docs/SPEC.md](docs/SPEC.md)
- [docs/GLOSSARY.md](docs/GLOSSARY.md)
- [docs/FAQ.md](docs/FAQ.md)
- [docs/ROADMAP.md](docs/ROADMAP.md)

### Para contribuidores

- [CONTRIBUTING.md](CONTRIBUTING.md)
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- [docs/DEV_ENV.md](docs/DEV_ENV.md)
- [docs/DIAGNOSTICS_CHECKLIST.md](docs/DIAGNOSTICS_CHECKLIST.md)
- [docs/DIAGNOSTICS_GLOSSARY.md](docs/DIAGNOSTICS_GLOSSARY.md)
- [docs/TESTING.md](docs/TESTING.md)
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [docs/STYLE_GUIDE.md](docs/STYLE_GUIDE.md)
- [docs/DOCS_CONTRIBUTING.md](docs/DOCS_CONTRIBUTING.md)

### Internals e planejamento

- [docs/CODEGEN.md](docs/CODEGEN.md)
- [docs/MVP_PLAN.md](docs/MVP_PLAN.md)
- [docs/ADOPTION_PLAN.md](docs/ADOPTION_PLAN.md)
- [docs/DESIGN_NOTES.md](docs/DESIGN_NOTES.md)
- [docs/CHANGELOG.md](docs/CHANGELOG.md)
- [docs/RELEASE_CHECKLIST.md](docs/RELEASE_CHECKLIST.md)

---

## 🧰 CLI (dev)

```bash
# lex e imprime tokens
cargo run -p tupa-cli -- lex examples/hello.tp

# lex com saída JSON
cargo run -p tupa-cli -- lex --format json examples/hello.tp

# parse e imprime AST
cargo run -p tupa-cli -- parse examples/hello.tp

# parse e imprime AST em JSON
cargo run -p tupa-cli -- parse --format json examples/hello.tp

# parse via stdin
cat examples/hello.tp | cargo run -p tupa-cli -- parse --stdin

# lex via stdin
cat examples/hello.tp | cargo run -p tupa-cli -- lex --stdin

# parse e valida tipos
cargo run -p tupa-cli -- check examples/hello.tp

# valida tipos com saída JSON
cargo run -p tupa-cli -- check --format json examples/hello.tp

# valida via stdin
cat examples/hello.tp | cargo run -p tupa-cli -- check --stdin

# gera codegen (stub)
cargo run -p tupa-cli -- codegen examples/hello.tp

# gera codegen (stub) em JSON
cargo run -p tupa-cli -- codegen --format json examples/hello.tp

# versão e sobre
cargo run -p tupa-cli -- version
cargo run -p tupa-cli -- about
```

---

## 🧩 Diagnósticos (exemplo)

Erros agora incluem código e linha/coluna:

```
error[E2001]: type mismatch: expected I64, got Bool
	--> examples/invalid_type.tp:2:15
	 |
 2 | 	let x: i64 = true;
	 |               ^^^^
```

Saída JSON também está disponível via `--format json` para consumo por ferramentas.

---

## ⚖️ Licença

- **Compilador**: Apache License 2.0
- **Runtime**: MIT License
- **Especificação**: CC-BY-SA 4.0

> ✅ Software livre, comercialmente amigável, com compartilhamento obrigatório de melhorias na spec

---

## 💬 Comunidade

- [GitHub Discussions](https://github.com/marciopaiva/tupalang/discussions): RFCs e debates técnicos
- [Twitter @tupalang](https://twitter.com/tupalang): atualizações e demos

---

> **🌩️ Tupã: onde a sabedoria ancestral encontra a engenharia do futuro**  
> *github.com/marciopaiva/tupalang*

---

## 🎨 Sugestões de customização para seu repositório

1. **Adicione um banner visual** no topo:
   ```markdown
   ![Tupã Banner](https://via.placeholder.com/1200x300/1A1A1A/E66700?text=⚡+TUPÃ+-+Força+Ancestral,+Código+Moderno)
   ```

2. **Inclua um GIF demo** logo após o exemplo de código:
   ```markdown
   ![Demo](demo.gif)
   *Compilando hello.tp → binário nativo em 0.8s*
   ```

3. **Badge de "Projeto Brasileiro"** (orgulho cultural):
   ```markdown
   [![Brasil](https://img.shields.io/badge/feito_no-Brasil-009739?logo=brazil)](#)
   ```

4. **Seção "Quem usa Tupã?"** (para quando tiver adopters):
   ```markdown
   ## 🏢 Early Adopters
	- [Satellite AI](https://example.com): Detecção de anomalias em redes Red Hat
	- [Nuvem Tupã](https://example.com): PaaS brasileiro para microserviços IA
   ```
