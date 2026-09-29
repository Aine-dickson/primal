# Prismal Design Documents

> **Prismal: One model. Many representations.**

These documents record how Prismal was derived. The method: list every case the language
must cover before choosing any syntax, then work out the semantics from those cases.

Source: design conversation, exported 2026-09-29. Text is preserved as written; the 09.x
documents include their original "where we are" tracking notes.

## Foundations

| # | Document | Question it answers |
|---|---|---|
| 01 | [Test Case Catalogue](01-test-case-catalogue.md) | What should the system eventually be able to represent? |
| 02 | [Capability / Case Matrix](02-capability-case-matrix.md) | What does each case require? |
| 03 | [Capability Taxonomy](03-capability-taxonomy.md) | Which capabilities keep recurring across cases? |
| 04 | [Generalized Model](04-generalized-model.md) | What is the smallest coherent system that expresses them all? |
| 05 | [Model Validation](05-model-validation.md) | Does the model survive the test cases? |
| 05b | [Model Gap Analysis](05b-model-gap-analysis.md) | Where is the model strained or insufficient? |
| 05c | [Refined Core Model](05c-refined-core-model.md) | The consolidated semantic core |

## Semantics

| # | Document | Question it answers |
|---|---|---|
| 06 | [Runtime Semantics](06-runtime-semantics.md) | How does a model execute and evolve? |
| 07 | [Projection & Representation Model](07-projection-representation-model.md) | How does meaning become a representation? |
| 08 | [Interactive Model](08-interactive-model.md) | How do user actions flow back into the model? |

## 09 DSL Design

| # | Document |
|---|---|
| 09.1 | [Language Requirements](09.01-language-requirements.md) |
| 09.2 | [Construct Inventory](09.02-construct-inventory.md) |
| 09.3 | [Syntax Options & Construct Minimization](09.03-syntax-options.md) |
| 09.4 | [Semantic Mapping](09.04-semantic-mapping.md) |
| 09.5 | [Type System](09.05-type-system.md) |
| 09.5b | [Type-System Revalidation Against the Extension Model](09.05b-type-system-revalidation.md) |
| 09.6 | [Scoping / Binding](09.06-scoping-binding.md) |
| 09.7 | [Composition](09.07-composition.md) |
| 09.8 | [Error Model](09.08-error-model.md) |
| 09.9 | [Extension Model](09.09-extension-model.md) |
| 09.10 | Example Programs: [Plan](09.10a-example-programs-plan.md) · [Batch 1](09.10b-example-programs-batch-1.md) · [Batch 1 Refinement + Batch 2](09.10c-example-programs-batch-2.md) |
| 09.11 | [DSL Validation](09.11-dsl-validation.md) |

## Identity

- [Name & Identity](10-branding.md): why the name is Prismal, the taglines, and the social description

## Status

**Frozen exploration record. Not normative.**

These documents were audited on 2026-09-29 (`docs/audit/2026-09-29-design-audit.md`). They remain
as the historical record; their text is not rewritten, but marked correction notes point to the
current position where the audit found errors. Departures and decisions are tracked in
`docs/decisions/divergence-register.md`, which takes precedence. No DSL has been adopted.
Current project state: `docs/PROJECT-STATE.md`.
