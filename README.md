# ⚡ Tupã

> **Força ancestral, código moderno**  
> Linguagem brasileira para sistemas críticos e IA evolutiva

[![Build Status](https://img.shields.io/github/actions/workflow/status/marciopaiva/tupalang/ci.yml?branch=main&logo=github)](https://github.com/marciopaiva/tupalang/actions)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![MVP](https://img.shields.io/badge/status-MVP%20em%20constru%C3%A7%C3%A3o-orange)](#)

> **Status atual**: este projeto é apenas uma ideia. Ainda não há implementação iniciada nem previsão de início.

```tupa
// IA responsável desde o primeiro caractere
fn summarize(article: Text) -> SafeText<!misinformation> {
	return llm.generate(f"Resuma objetivamente: {article}")
}
```

---

## 🌩️ Por que Tupã?

Na mitologia tupi-guarani, **Tupã** é a divindade do trovão — força bruta canalizada com precisão. Assim é nossa linguagem:

| Problema atual das linguagens | Solução Tupã |
|-------------------------------|--------------|
| ❌ Python: dinâmico demais → bugs em runtime | ✅ Tipagem gradual com *alignment* em compile-time |
| ❌ Rust: seguro mas curva acentuada para pesquisadores de IA | ✅ Sintaxe legível + segurança sem sacrifício |
| ❌ Todas: diferenciabilidade via bibliotecas frágeis | ✅ `∇` (nabla) como operador de primeira classe |
| ❌ Modelos densos → pegada de carbono insustentável | ✅ Esparsidade declarativa no tipo (`density=0.1`) |

> **Tupã não é "mais uma linguagem"** — é a **primeira linguagem projetada desde o solo para a era pós-LLM**, onde segurança ética e eficiência energética são tão críticas quanto performance.

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

## 🤝 Como Contribuir

### Níveis de envolvimento

| Perfil | Como ajudar | Issue label |
|--------|-------------|-------------|
| **Curioso** | Teste o MVP, reporte bugs | `good first issue` |
| **Dev Rust** | Implemente parser/lexer | `help wanted` |
| **Pesquisador IA** | Projete `@differentiable` semantics | `research` |
| **Designer** | Crie logo/branding Tupã | `design` |
| **Escritor** | Documentação em português/inglês | `docs` |

### Primeiros passos

```bash
# 1. Clone o repositório
git clone https://github.com/marciopaiva/tupalang
cd tupa

# 2. Setup Rust (nightly requerido)
rustup install nightly-2025-01-15
rustup override set nightly-2025-01-15

# 3. Rode testes do lexer
cargo test -p tupa-lexer

# 4. Compile o "Hello World"
cargo run --bin tupa-cli -- examples/hello.tp
```

> 💡 **Não sabe Rust?** Comece com:
> - `docs/SPEC.md` → sugira melhorias na especificação
> - `examples/` → crie exemplos de uso para IA
> - Issues → triagem de bugs relatados

---

## 🌍 Orgulho Brasileiro, Ambição Global

Tupã é a **primeira linguagem brasileira com ambição global desde Lua** (1993). Mas enquanto Lua focou em *embeddability*, Tupã nasce para resolver os maiores desafios da computação moderna:

- 🇧🇷 **Raízes tupi-guarani** — nome, logo e filosofia inspirados na sabedoria ancestral
- 🌱 **Sustentabilidade** — esparsidade nativa reduz pegada de carbono da IA
- 🛡️ **Segurança ética** — alignment não é *feature*, é fundação
- ⚡ **Performance** — LLVM + zero-cost abstractions = código tão rápido quanto C

> *"Não estamos reinventando a roda — estamos construindo a primeira roda que não polui o planeta enquanto rola."*

---

## 📚 Recursos

| Documento | Descrição |
|-----------|-----------|
| [docs/SPEC.md](docs/SPEC.md) | Especificação técnica completa (gramática EBNF) |
| [docs/MVP_PLAN.md](docs/MVP_PLAN.md) | Plano objetivo do MVP |
| [docs/ISSUES.md](docs/ISSUES.md) | Lista de issues iniciais sugeridas |
| [docs/ADOPTION_PLAN.md](docs/ADOPTION_PLAN.md) | Plano técnico mínimo de adoção |
| [docs/AI_SUPPORT_SUGGESTIONS.md](docs/AI_SUPPORT_SUGGESTIONS.md) | Sugestões adicionais para apoio por IA |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Guia para novos contribuidores |
| [examples/README.md](examples/README.md) | Casos de uso reais (IA, microserviços, sistemas críticos) |

---

## ⚖️ Licença

- **Compilador**: Apache License 2.0
- **Runtime**: MIT License
- **Especificação**: CC-BY-SA 4.0

> ✅ Software livre, comercialmente amigável, com compartilhamento obrigatório de melhorias na spec

---

## 💬 Comunidade

- [GitHub Discussions](https://github.com/marciopaiva/tupalang/discussions) — RFCs e debates técnicos
- [Twitter @tupalang](https://twitter.com/tupalang) — atualizações e demos

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
   - [Satellite AI](https://example.com) — Detecção de anomalias em redes Red Hat
   - [Nuvem Tupã](https://example.com) — PaaS brasileiro para microserviços IA
   ```
