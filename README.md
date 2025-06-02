<div align="center">
  <img src="logo2.png" alt="Banking & Fintech Prompt Engineering Library" width="400">
</div>

# Banking & Fintech Prompt Engineering Library 🦀🏦

A simplified, educational Rust library for learning prompt engineering concepts and Rust programming patterns, specialized for banking and financial services applications.

## 🎯 Learning Objectives

This library is designed to teach:
- **Builder Pattern**: Fluent API design for constructing complex objects
- **Trait-based Abstraction**: Creating interfaces for different implementations
- **Async Programming**: Non-blocking operations with async/await
- **Structured Code**: Clean, readable, and well-documented Rust code
- **Prompt Engineering**: Organizing and structuring prompts for LLMs

## 🚀 Quick Start

### Run the Demo
```bash
cargo run
```

### Run Tests
```bash
cargo test
```

## 📚 Core Concepts

### 1. Prompt Structure
Prompts are organized into logical sections:
- **Goal**: What you want to achieve
- **Context**: Background information
- **Role**: The persona for the LLM to adopt
- **Steps**: Specific instructions or actions
- **Examples**: Input/output demonstrations
- **Output**: Desired response format

### 2. Builder Pattern
```rust
let prompt = PromptBuilder::new()
    .goal("Analyze customer feedback")
    .role("Data analyst")
    .step("Load the data")
    .step("Perform analysis")
    .output("Summary report")
    .build();
```

### 3. Template System
Pre-built templates for banking and fintech use cases:
```rust
let template = PromptTemplate::CreditRiskAssessment {
    credit_type: "personal loans".to_string(),
    risk_focus: "default probability assessment".to_string(),
};
let prompt = template.to_builder().build();
```

### 4. Available Banking & Fintech Templates

- **CreditRiskAssessment**: For loan evaluation and credit risk analysis
- **FraudDetection**: For transaction monitoring and fraud prevention
- **RegulatoryCompliance**: For compliance assessment and audit preparation
- **MarketAnalysis**: For financial market analysis and investment insights
- **FinancialAdvisory**: For wealth management and investment recommendations

### 5. LLM Client Abstraction
```rust
#[async_trait]
pub trait SimpleLLMClient: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String>;
}
```

## Crafting High-Performance LLM Agents with **ThoughtStage**-Style Prompts

*A synthesis of the latest research & concrete engineering advice*

---

### 1 | Why Structured Prompts Matter

Large-language-model agents don't just need *words*—they need a **scaffold** that makes their reasoning explicit, constrains their output and bakes in safety.
Our internal `ThoughtStage` enum captures that scaffold:

```rust
enum ThoughtStage {
    Goal(StageInput),
    Context(StageInput),
    Assumption(StageInput),
    Step(StageInput),
    Role(StageInput),
    Output(StageInput),
    TokenLimit(StageInput),
    WordLimit(StageInput),
    Reasoning(StageInput),
    Guardrail(StageInput),
    Rubric(StageInput),
    Example { user: StageInput, assistant: StageInput },
    Style(StageInput),
    Meta(StageInput),
}
```

Recent literature shows that **systems which approximate this structure consistently outperform ad-hoc prompting.** Below we merge four key papers—and one especially relevant survey—into practical guidance for each stage.

---

### 2 | The Core Survey: *Towards Goal-oriented LLM Prompting*

Li et al.'s survey (Jan 2024) proposes five iterative stages—**Goal Decomposition → Action Selection → Action Execution → Sub-goal Evaluation → Valuable-Sub-goal Selection**—and reviews 50 studies that implement them.  Performance gains are substantial:

| Technique                  | Task                 | Reported Boost                                    |
| -------------------------- | -------------------- | ------------------------------------------------- |
| **Chain-of-Thought (CoT)** | arithmetic reasoning | **+22.6 pp** accuracy                             |
| **Self-consistency**       | arithmetic reasoning | **+32.5 pp** accuracy                             |
| **Self-refine**            | code Q\&A            | **+21.1 pp** F1                                   |
| **SayPlan**                | embodied planning    | **+73 pp** *"excitability"* over baseline planner |

Those five survey stages map neatly onto *ThoughtStage*:

| Survey Stage                | ThoughtStage analogue    |
| --------------------------- | ------------------------ |
| Goal Decomposition          | `Goal`, `Step`           |
| Action Selection            | `Role`, `Assumption`     |
| Action Execution            | (implicit in LLM call)   |
| Sub-goal Evaluation         | `Reasoning`, `Guardrail` |
| Valuable-Sub-goal Selection | `Rubric`, `Example`      |

([arXiv][1])

---

### 3 | Complementary Findings & How They Map

| Paper                                                                       | Core Idea                                                                | Relevant ThoughtStages                                        |
| --------------------------------------------------------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------- |
| **Tree of Thoughts** (Yao et al., May 2023)                                 | Explore many reasoning paths; backtrack if needed → +70 pp on Game-of-24 | `Step`, `Reasoning`, `Rubric` (select best path) ([arXiv][2]) |
| **Prompt Pattern Catalog** (White et al., Feb 2023)                         | 16 reusable patterns (Persona, Template…)                                | `Role`, `Output`, `Style`, `Meta` ([arXiv][3])                |
| **Prompt Canvas** (Hewing & Leinhos, 2024)                                  | Worksheet with Goal, Context, Role, Style, Guardrails, Examples          | Directly mirrors most enum fields                             |
| **Content–Format-Integrated Prompt Optimisation (CFPO)** (Liu et al., 2025) | Jointly tune wording *and* formatting → +3-7 pp on reasoning tasks       | `Output`, `TokenLimit`, `Style`, `Meta`                       |
| **Prompt Formatting Impact Study** (He et al., 2024)                        | Markdown sections & bullet lists swing accuracy by up to **40 %**        | `Output`, `Style`, `TokenLimit`                               |
| **Efficient Prompting Survey** (Chang et al., Apr 2024)                     | Prompt compression & length caps cut latency/\$\$                        | `TokenLimit`, `WordLimit` ([arXiv][4])                        |

---

### 4 | Stage-by-Stage Best-Practice Cheatsheet

| ThoughtStage          | What the papers recommend                                                     |
| --------------------- | ----------------------------------------------------------------------------- |
| **Goal**              | State desired end-state in ≤ 20 words (Li et al.).                            |
| **Context**           | Separate paragraph labelled *Context:* (Prompt Canvas).                       |
| **Assumption**        | Bullet-list hidden premises; keep each under 80 chars (CFPO).                 |
| **Step**              | Numbered sub-tasks *1., 2., 3.*; pairs well with Tree-of-Thought sampling.    |
| **Role**              | "You are a *{domain-expert}* …" boosts BLEU +6 % on translation (Canvas).     |
| **Output**            | Specify **format + tone** (e.g. "Markdown table, concise").                   |
| **Token/Word Limits** | Always add a hard cap—smaller models otherwise over-run (He et al.).          |
| **Reasoning**         | Enable CoT or ToT only for models ≥ GPT-4 quality; else skip to save tokens.  |
| **Guardrail**         | Use a negative→positive pattern: "Do **not** reveal keys. Instead…"           |
| **Rubric**            | Put acceptance criteria after guardrails; self-check loops use it.            |
| **Example**           | One or two few-shot pairs still yield +12 pp EM on QA (CFPO data).            |
| **Style**             | Limit to one adjective pair ("concise & persuasive"); more dilutes adherence. |
| **Meta**              | End with parsing instructions ("Return JSON only, no commentary").            |

---

### 5 | Putting It All Together

Below is a *minimal but research-backed* prompt template (Markdown). Swap `[ … ]` for your content and you have an agent-ready script fully aligned with the literature.

```markdown
## Goal
[Write a weekly engineering digest]

## Context
[Audience: Staff engineers & managers in fintech]

## Assumptions
- Readers understand basic security jargon
- No confidential data may be disclosed

## Steps
1. Summarise top merged PRs
2. Highlight risk items
3. Provide next-week plan

## Role
You are the company's Senior Staff Engineer.

## Guardrails
Do **not** reveal internal repository URLs. Instead, refer to modules abstractly.

## Rubric
✓ Covers at least 3 PRs  
✓ Includes a bullet-point action list  
✓ ≤ 600 tokens

## Example
**User:** What is least-privilege?  
**Assistant:** Least-privilege means each identity has only the permissions essential to perform its tasks.

## Output
Markdown with h2 sections; TL;DR at top.

## Style
Concise & authoritative.

## Meta
Return Markdown only, no extra commentary outside sections.
```

---

### 6 | Limitations & Future Work

* **Guardrail + Rubric research is thin.** Current papers treat them as sub-steps; explicit stage-wise studies are needed.
* **Efficiency trade-offs.** Compression methods (Chang et al.) are promising but can harm interpretability—ideal for background tasks, not user-visible answers.
* **Hierarchical prompts.** Li et al. flag multi-level decomposition (scripts, planning) as the next frontier—perfect for extending `Meta` or adding new enum variants.

---

### 7 | Take-home for builders

1. **Adopt the full ThoughtStage schema**—it subsumes best practices across all surveyed work.
2. **Log A/B experiments** on *format* as vigorously as on *content*—40 % swings are common.
3. **Automate evaluation**: feed your `Rubric` back through self-critique loops (e.g., *SelfCheck*, *CRITIC*).
4. **Cache + reuse prompts**—efficient prompting cuts both tokens *and* latency.

---

### 8 | Key References

1. Li H. et al., **"Towards Goal-oriented Prompt Engineering for Large Language Models: A Survey."** arXiv, 2024. ([arXiv][1])
2. Yao S. et al., **"Tree of Thoughts: Deliberate Problem Solving with LLMs."** arXiv, 2023. ([arXiv][2])
3. White J. et al., **"A Prompt Pattern Catalog to Enhance Prompt Engineering with ChatGPT."** arXiv, 2023. ([arXiv][3])
4. Liu Y. et al., **"Content-Format Integrated Prompt Optimisation (CFPO)."** arXiv, 2025.
5. He H. et al., **"Does Prompt Formatting Have Any Impact on LLM Performance?"** arXiv, 2024.
6. Chang K. et al., **"Efficient Prompting Methods for Large Language Models: A Survey."** arXiv, 2024. ([arXiv][4])

Use this blueprint as your next blog post—or the backbone of a talk—and equip your agents with a research-grade prompt architecture. Happy building!

[1]: https://arxiv.org/abs/2401.14043?utm_source=chatgpt.com "Towards Goal-oriented Prompt Engineering for Large Language Models: A Survey"
[2]: https://arxiv.org/abs/2305.10601?utm_source=chatgpt.com "Tree of Thoughts: Deliberate Problem Solving with Large Language Models"
[3]: https://arxiv.org/abs/2302.11382?utm_source=chatgpt.com "A Prompt Pattern Catalog to Enhance Prompt Engineering with ChatGPT"
[4]: https://arxiv.org/abs/2404.01077?utm_source=chatgpt.com "Efficient Prompting Methods for Large Language Models: A Survey"

## Crafting High-Performance LLM Agents with **ThoughtStage**-Style Prompts

*A synthesis of the latest research & concrete engineering advice*

---

### 1 | Why Structured Prompts Matter

Large-language-model agents don't just need *words*—they need a **scaffold** that makes their reasoning explicit, constrains their output and bakes in safety.

### 8 | Key References

1. Li H. et al., **"Towards Goal-oriented Prompt Engineering for Large Language Models: A Survey."** arXiv, 2024.
2. Yao S. et al., **"Tree of Thoughts: Deliberate Problem Solving with LLMs."** arXiv, 2023.
3. White J. et al., **"A Prompt Pattern Catalog to Enhance Prompt Engineering with ChatGPT."** arXiv, 2023.

Use this blueprint as your next blog post—or the backbone of a talk—and equip your agents with a research-grade prompt architecture. Happy building!

## 🏗️ Architecture

The library consists of:

1. **Core Data Structures** (`PromptSection`, `Prompt`)
2. **Builder Pattern** (`PromptBuilder`)
3. **Template System** (`PromptTemplate` with 5 common patterns)
4. **LLM Interface** (`SimpleLLMClient` trait)
5. **Mock Implementation** (`MockLLMClient`)
6. **Demo & Tests**

## 🔍 Code Walkthrough

### Main Components

```rust
// Define what goes in a prompt
pub enum PromptSection {
    Goal(String),
    Context(String),
    Role(String),
    Step(String),
    Example { user: String, assistant: String },
    Output(String),
}

// Build prompts fluently
pub struct PromptBuilder {
    prompt: Prompt,
}

// Abstract LLM communication
#[async_trait]
pub trait SimpleLLMClient: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String>;
}
```

### Usage Examples

#### Manual Prompt Building
```rust
// Build a structured credit risk assessment prompt manually
let prompt = PromptBuilder::new()
    .goal("Assess credit risk for mortgage application")
    .context("30-year fixed rate mortgage, $400k loan amount")
    .role("Senior Credit Risk Analyst")
    .step("Analyze credit history and FICO score")
    .step("Evaluate debt-to-income ratio")
    .step("Assess employment stability")
    .example(
        "FICO 720, DTI 35%, 3 years employment",
        "Moderate risk - approve with standard terms"
    )
    .output("Risk assessment with approval recommendation")
    .build();
```

#### Template-Based Building
```rust
// Use a pre-built template for fraud detection
let template = PromptTemplate::FraudDetection {
    channel: "online banking".to_string(),
    scope: "real-time monitoring".to_string(),
};
let prompt = template.to_builder().build();

// Or customize a template for regulatory compliance
let compliance_template = PromptTemplate::RegulatoryCompliance {
    regulation: "AML/KYC".to_string(),
    focus: "customer due diligence".to_string(),
};
let custom_prompt = compliance_template.to_builder()
    .step("Review beneficial ownership documentation")
    .build();

// Send to LLM
let llm_client = MockLLMClient;
let response = llm_client.generate(&prompt.to_string()).await?;
```

## 🧪 Testing

The library includes comprehensive tests demonstrating:
- Builder pattern functionality
- Mock LLM client behavior
- Example section formatting
- Error handling

## 🎓 Educational Value

This simplified version focuses on:

✅ **Readability**: Every line is documented and explained  
✅ **Simplicity**: Core concepts without overwhelming complexity  
✅ **Best Practices**: Rust patterns and async programming  
✅ **Practical Examples**: Real-world prompt engineering scenarios  
✅ **Progressive Learning**: Build understanding step by step  

## 🛠️ Dependencies

Minimal dependencies for educational clarity:
- `anyhow`: Error handling
- `async-trait`: Async trait support
- `serde`: Serialization (for structured data)
- `tokio`: Async runtime

## 🚀 Next Steps

After understanding this simple version, you can:
1. Add real LLM client implementations (OpenAI, Anthropic)
2. Implement caching for efficiency
3. Add batch processing capabilities
4. Create a web interface
5. Add more sophisticated prompt templates

## 📝 License

MIT OR Apache-2.0

---

**Happy Learning! 🦀✨**

*Updated with comprehensive ThoughtStage theoretical foundation*

*Last updated: Added comprehensive ThoughtStage theoretical foundation section* 