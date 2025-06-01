// ╭─────────────────────────────────────╮
// │   Simple Banking Prompt Library    │
// │        Educational Version         │
// ╰─────────────────────────────────────╯

//! # Simple Banking Prompt Library for Learning
//!
//! A minimal prompt engineering library focused on banking use cases.
//! Demonstrates core Rust patterns: builder pattern, traits, and async programming.

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;

// ═══════════════════════════════════════════════════════════════════════════════════
// SECTION: Core Data Structures
// ═══════════════════════════════════════════════════════════════════════════════════

/// Different types of content that can be in a prompt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptSection {
    /// The main goal or objective
    Goal(String),
    /// The role or persona for the AI
    Role(String),
    /// Specific instructions
    Step(String),
    /// Desired output format
    Output(String),
}

/// A prompt containing multiple sections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    sections: Vec<PromptSection>,
}

impl Default for Prompt {
    fn default() -> Self {
        Self::new()
    }
}

impl Prompt {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    fn add_section(&mut self, section: PromptSection) {
        self.sections.push(section);
    }
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        for section in &self.sections {
            match section {
                PromptSection::Goal(content) => {
                    result.push(format!("Goal: {content}"));
                }
                PromptSection::Role(content) => {
                    result.push(format!("Role: {content}"));
                }
                PromptSection::Step(content) => {
                    result.push(format!("Step: {content}"));
                }
                PromptSection::Output(content) => {
                    result.push(format!("Output: {content}"));
                }
            }
        }
        write!(f, "{}", result.join("\n"))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SECTION: Builder Pattern
// ═══════════════════════════════════════════════════════════════════════════════════

/// Builder for creating prompts using a fluent API.
#[derive(Default)]
pub struct PromptBuilder {
    prompt: Prompt,
}

impl PromptBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            prompt: Prompt::new(),
        }
    }

    /// Adds a goal section
    #[must_use]
    pub fn goal(mut self, goal: impl Into<String>) -> Self {
        self.prompt.add_section(PromptSection::Goal(goal.into()));
        self
    }

    /// Adds a role section
    #[must_use]
    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.prompt.add_section(PromptSection::Role(role.into()));
        self
    }

    /// Adds a step section
    #[must_use]
    pub fn step(mut self, step: impl Into<String>) -> Self {
        self.prompt.add_section(PromptSection::Step(step.into()));
        self
    }

    /// Adds an output format section
    #[must_use]
    pub fn output(mut self, output: impl Into<String>) -> Self {
        self.prompt
            .add_section(PromptSection::Output(output.into()));
        self
    }

    /// Finishes building and returns the prompt
    #[must_use]
    pub fn build(self) -> Prompt {
        self.prompt
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SECTION: LLM Client Interface
// ═══════════════════════════════════════════════════════════════════════════════════

/// Simple interface for communicating with LLMs.
#[async_trait]
pub trait SimpleLLMClient: Send + Sync {
    /// Sends a prompt to the LLM and gets a response.
    async fn generate(&self, prompt: &str) -> Result<String>;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SECTION: Mock LLM Client
// ═══════════════════════════════════════════════════════════════════════════════════

/// Mock LLM client for demonstration and testing.
pub struct MockLLMClient;

#[async_trait]
impl SimpleLLMClient for MockLLMClient {
    /// Returns a mock response based on prompt content.
    async fn generate(&self, prompt: &str) -> Result<String> {
        // Simulate network delay
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Simple responses based on banking prompt content
        if prompt.contains("credit risk") || prompt.contains("Credit Risk") {
            Ok("CREDIT ANALYSIS COMPLETE\n\nApplicant Profile: FICO 720, DTI 28%, Stable Employment\nRisk Assessment: LOW RISK (2.1% default probability)\nRecommendation: APPROVED at Prime + 1.25%\nRequired: Income verification, property appraisal".to_string())
        } else if prompt.contains("fraud") || prompt.contains("Fraud") {
            Ok("FRAUD ALERT ISSUED\n\nTransaction Pattern: Multiple ATM withdrawals detected\nRisk Level: HIGH (Score 85/100)\nGeographic Anomaly: 500+ miles from normal location\nAction Required: FREEZE card, contact customer immediately".to_string())
        } else {
            Ok("Analysis complete. Banking task processed according to regulatory guidelines and best practices.".to_string())
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SECTION: Banking Templates
// ═══════════════════════════════════════════════════════════════════════════════════

/// Pre-built templates for common banking use cases.
#[derive(Debug, Clone)]
pub enum BankingTemplate {
    /// Credit risk assessment and loan evaluation
    CreditRisk { loan_type: String, focus: String },
    /// Fraud detection and prevention
    FraudDetection { channel: String, scope: String },
}

impl BankingTemplate {
    /// Creates a pre-configured prompt builder.
    #[must_use]
    pub fn to_builder(&self) -> PromptBuilder {
        match self {
            Self::CreditRisk { loan_type, focus } => PromptBuilder::new()
                .goal(format!(
                    "Assess credit risk for {loan_type} focusing on {focus}"
                ))
                .role("Senior Credit Risk Analyst")
                .step("Analyze credit history and payment patterns")
                .step("Evaluate income stability and debt ratios")
                .step("Calculate default probability and risk rating")
                .step("Determine loan terms and interest rates")
                .output("Risk assessment with approval recommendation"),
            Self::FraudDetection { channel, scope } => PromptBuilder::new()
                .goal(format!("Detect fraud in {channel} using {scope}"))
                .role("Fraud Detection Specialist")
                .step("Analyze transaction patterns and anomalies")
                .step("Apply fraud scoring models")
                .step("Check against known risk indicators")
                .step("Generate alerts and recommended actions")
                .output("Fraud risk assessment with action plan"),
        }
    }

    /// Gets a description of what this template does.
    #[must_use]
    pub fn description(&self) -> String {
        match self {
            Self::CreditRisk { loan_type, focus } => {
                format!("Assesses credit risk for {loan_type} focusing on {focus}")
            }
            Self::FraudDetection { channel, scope } => {
                format!("Detects fraud in {channel} using {scope}")
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SECTION: Demo Function
// ═══════════════════════════════════════════════════════════════════════════════════

/// Demonstrates the banking prompt library.
async fn demo_banking_prompts() -> Result<()> {
    println!("🏦 Simple Banking Prompt Library Demo");
    println!("=====================================");
    println!();

    // Manual prompt building
    println!("📝 Manual Prompt Building:");
    let manual_prompt = PromptBuilder::new()
        .goal("Evaluate loan application")
        .role("Credit Analyst")
        .step("Review credit score and history")
        .step("Analyze income and debt ratios")
        .output("Approval recommendation with terms")
        .build();

    println!(
        "✅ Built manually: {} sections",
        manual_prompt.sections.len()
    );
    println!();

    // Template-based building
    println!("🎯 Template-Based Building:");
    let template = BankingTemplate::CreditRisk {
        loan_type: "mortgage".to_string(),
        focus: "default risk".to_string(),
    };

    let template_prompt = template.to_builder().build();
    println!("✅ {}", template.description());
    println!(
        "✅ Built from template: {} sections",
        template_prompt.sections.len()
    );
    println!();

    // Test with LLM client
    println!("🤖 Testing with LLM:");
    let llm_client = MockLLMClient;

    let response = llm_client.generate(&template_prompt.to_string()).await?;
    println!("💬 Response:");
    println!("{response}");
    println!();

    println!("🎉 Demo completed!");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SECTION: Main Function
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::main]
async fn main() -> Result<()> {
    demo_banking_prompts().await?;

    println!();
    println!("📚 Key Learning Points:");
    println!("   ✅ Builder pattern for fluent APIs");
    println!("   ✅ Trait abstraction for LLM clients");
    println!("   ✅ Template system for reusable prompts");
    println!("   ✅ Async programming with Rust");
    println!("   ✅ Clean, readable code structure");

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SECTION: Tests
// ═══════════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_builder() {
        let prompt = PromptBuilder::new()
            .goal("Test goal")
            .role("Test role")
            .step("Test step")
            .build();

        let text = prompt.to_string();
        assert!(text.contains("Goal: Test goal"));
        assert!(text.contains("Role: Test role"));
        assert!(text.contains("Step: Test step"));
    }

    #[tokio::test]
    async fn test_mock_llm_client() {
        let client = MockLLMClient;
        let response = client.generate("credit risk assessment").await.unwrap();
        assert!(response.contains("CREDIT") || response.contains("credit"));
        assert!(!response.is_empty());
    }

    #[test]
    fn test_credit_risk_template() {
        let template = BankingTemplate::CreditRisk {
            loan_type: "personal loan".to_string(),
            focus: "default probability".to_string(),
        };

        let prompt = template.to_builder().build();
        let text = prompt.to_string();

        assert!(text.contains("personal loan"));
        assert!(text.contains("Credit Risk Analyst"));
        assert!(text.contains("default probability"));
    }

    #[test]
    fn test_fraud_detection_template() {
        let template = BankingTemplate::FraudDetection {
            channel: "online banking".to_string(),
            scope: "real-time".to_string(),
        };

        let prompt = template.to_builder().build();
        let text = prompt.to_string();

        assert!(text.contains("online banking"));
        assert!(text.contains("Fraud Detection"));
        assert!(text.contains("real-time"));
    }

    #[test]
    fn test_template_descriptions() {
        let credit_template = BankingTemplate::CreditRisk {
            loan_type: "mortgage".to_string(),
            focus: "risk assessment".to_string(),
        };

        let fraud_template = BankingTemplate::FraudDetection {
            channel: "credit cards".to_string(),
            scope: "pattern analysis".to_string(),
        };

        assert!(credit_template.description().contains("mortgage"));
        assert!(fraud_template.description().contains("credit cards"));
    }
}
