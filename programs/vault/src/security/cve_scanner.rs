use anchor_lang::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use crate::errors::VaultError;

/// CVE Scanner for automated vulnerability detection
#[derive(Debug, Clone)]
pub struct CveScanner {
    advisory_db: AdvisoryDatabase,
    scan_config: ScanConfig,
    last_update: DateTime<Utc>,
}

/// Advisory database containing vulnerability information
#[derive(Debug, Clone, Default)]
pub struct AdvisoryDatabase {
    advisories: HashMap<String, Advisory>,
    crate_advisories: HashMap<String, Vec<String>>, // crate_name -> advisory_ids
}

/// Individual security advisory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Advisory {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub affected_crates: Vec<AffectedCrate>,
    pub date: DateTime<Utc>,
    pub url: String,
    pub solution: Option<String>,
}

/// Information about affected crate versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedCrate {
    pub name: String,
    pub affected_versions: VersionRange,
    pub patched_versions: Vec<String>,
}

/// Version range specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRange {
    pub min_version: Option<String>,
    pub max_version: Option<String>,
    pub excluded_versions: Vec<String>,
}

/// CVE severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Scanner configuration
#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub max_allowed_severity: Severity,
    pub auto_update_db: bool,
    pub fail_on_vulnerability: bool,
    pub allowed_vulnerabilities: HashSet<String>, // Advisory IDs that are temporarily allowed
}

/// Scan result containing vulnerability information
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub vulnerabilities: Vec<VulnerabilityMatch>,
    pub total_crates_scanned: usize,
    pub scan_duration_ms: u64,
    pub database_version: String,
}

/// Individual vulnerability match
#[derive(Debug, Clone)]
pub struct VulnerabilityMatch {
    pub advisory: Advisory,
    pub affected_crate: String,
    pub current_version: String,
    pub severity_score: f32,
    pub is_allowed: bool,
}

impl CveScanner {
    /// Create a new CVE scanner with default configuration
    pub fn new() -> Self {
        Self {
            advisory_db: AdvisoryDatabase::default(),
            scan_config: ScanConfig::default(),
            last_update: Utc::now(),
        }
    }

    /// Create scanner with custom configuration
    pub fn with_config(config: ScanConfig) -> Self {
        Self {
            advisory_db: AdvisoryDatabase::default(),
            scan_config: config,
            last_update: Utc::now(),
        }
    }

    /// Update the advisory database from RustSec
    pub async fn update_database(&mut self) -> Result<()> {
        msg!("Updating CVE advisory database...");
        
        // In a real implementation, this would fetch from:
        // https://github.com/RustSec/advisory-db.git
        // For now, we'll simulate with known vulnerabilities
        self.load_known_vulnerabilities()?;
        self.last_update = Utc::now();
        
        msg!("Advisory database updated successfully");
        Ok(())
    }

    /// Scan dependencies for vulnerabilities
    pub fn scan_dependencies(&self, dependencies: &[CrateDependency]) -> Result<ScanResult> {
        let start_time = std::time::Instant::now();
        let mut vulnerabilities = Vec::new();

        msg!("Scanning {} dependencies for vulnerabilities...", dependencies.len());

        for dep in dependencies {
            if let Some(advisory_ids) = self.advisory_db.crate_advisories.get(&dep.name) {
                for advisory_id in advisory_ids {
                    if let Some(advisory) = self.advisory_db.advisories.get(advisory_id) {
                        if self.is_version_affected(&dep.version, advisory)? {
                            let is_allowed = self.scan_config.allowed_vulnerabilities.contains(advisory_id);
                            
                            vulnerabilities.push(VulnerabilityMatch {
                                advisory: advisory.clone(),
                                affected_crate: dep.name.clone(),
                                current_version: dep.version.clone(),
                                severity_score: self.calculate_severity_score(advisory.severity),
                                is_allowed,
                            });
                        }
                    }
                }
            }
        }

        let scan_duration = start_time.elapsed();
        
        let result = ScanResult {
            vulnerabilities,
            total_crates_scanned: dependencies.len(),
            scan_duration_ms: scan_duration.as_millis() as u64,
            database_version: self.get_database_version(),
        };

        self.log_scan_results(&result)?;
        Ok(result)
    }

    /// Check if a specific crate version has known vulnerabilities
    pub fn check_crate_vulnerability(&self, name: &str, version: &str) -> Result<Vec<Advisory>> {
        let mut vulnerabilities = Vec::new();

        if let Some(advisory_ids) = self.advisory_db.crate_advisories.get(name) {
            for advisory_id in advisory_ids {
                if let Some(advisory) = self.advisory_db.advisories.get(advisory_id) {
                    if self.is_version_affected(version, advisory)? {
                        vulnerabilities.push(advisory.clone());
                    }
                }
            }
        }

        Ok(vulnerabilities)
    }

    /// Get security recommendations for vulnerable dependencies
    pub fn get_security_recommendations(&self, scan_result: &ScanResult) -> Vec<SecurityRecommendation> {
        let mut recommendations = Vec::new();

        for vuln in &scan_result.vulnerabilities {
            if !vuln.is_allowed {
                let recommendation = match vuln.advisory.severity {
                    Severity::Critical => SecurityRecommendation {
                        crate_name: vuln.affected_crate.clone(),
                        current_version: vuln.current_version.clone(),
                        recommendation_type: RecommendationType::ImmediateUpdate,
                        suggested_version: self.get_safe_version(&vuln.advisory),
                        rationale: format!("Critical vulnerability: {}", vuln.advisory.title),
                        urgency: Urgency::Immediate,
                    },
                    Severity::High => SecurityRecommendation {
                        crate_name: vuln.affected_crate.clone(),
                        current_version: vuln.current_version.clone(),
                        recommendation_type: RecommendationType::ScheduledUpdate,
                        suggested_version: self.get_safe_version(&vuln.advisory),
                        rationale: format!("High severity vulnerability: {}", vuln.advisory.title),
                        urgency: Urgency::High,
                    },
                    Severity::Medium | Severity::Low => SecurityRecommendation {
                        crate_name: vuln.affected_crate.clone(),
                        current_version: vuln.current_version.clone(),
                        recommendation_type: RecommendationType::ConsiderUpdate,
                        suggested_version: self.get_safe_version(&vuln.advisory),
                        rationale: format!("Moderate vulnerability: {}", vuln.advisory.title),
                        urgency: Urgency::Medium,
                    },
                };
                recommendations.push(recommendation);
            }
        }

        recommendations
    }

    /// Validate that scan results meet security policy
    pub fn validate_security_policy(&self, scan_result: &ScanResult) -> Result<PolicyValidationResult> {
        let mut violations = Vec::new();
        let mut critical_count = 0;
        let mut high_count = 0;

        for vuln in &scan_result.vulnerabilities {
            if !vuln.is_allowed {
                match vuln.advisory.severity {
                    Severity::Critical => {
                        critical_count += 1;
                        violations.push(PolicyViolation {
                            rule: "No critical vulnerabilities allowed".to_string(),
                            crate_name: vuln.affected_crate.clone(),
                            advisory_id: vuln.advisory.id.clone(),
                            severity: vuln.advisory.severity,
                        });
                    },
                    Severity::High => {
                        high_count += 1;
                        if vuln.advisory.severity > self.scan_config.max_allowed_severity {
                            violations.push(PolicyViolation {
                                rule: format!("Severity exceeds maximum allowed: {:?}", self.scan_config.max_allowed_severity),
                                crate_name: vuln.affected_crate.clone(),
                                advisory_id: vuln.advisory.id.clone(),
                                severity: vuln.advisory.severity,
                            });
                        }
                    },
                    _ => {}
                }
            }
        }

        let is_compliant = violations.is_empty();
        
        Ok(PolicyValidationResult {
            is_compliant,
            violations,
            critical_vulnerabilities: critical_count,
            high_vulnerabilities: high_count,
            total_vulnerabilities: scan_result.vulnerabilities.len(),
        })
    }

    // Private helper methods
    fn load_known_vulnerabilities(&mut self) -> Result<()> {
        // Load current known vulnerabilities that we need to address
        let advisories = vec![
            Advisory {
                id: "RUSTSEC-2024-0344".to_string(),
                title: "Timing variability in `curve25519-dalek`'s `Scalar29::sub`/`Scalar52::sub`".to_string(),
                description: "Timing side-channel vulnerability in scalar subtraction".to_string(),
                severity: Severity::Medium,
                affected_crates: vec![AffectedCrate {
                    name: "curve25519-dalek".to_string(),
                    affected_versions: VersionRange {
                        min_version: None,
                        max_version: Some("4.1.2".to_string()),
                        excluded_versions: vec![],
                    },
                    patched_versions: vec!["4.1.3".to_string()],
                }],
                date: chrono::DateTime::parse_from_rfc3339("2024-06-18T00:00:00Z").unwrap().with_timezone(&Utc),
                url: "https://rustsec.org/advisories/RUSTSEC-2024-0344".to_string(),
                solution: Some("Upgrade to >=4.1.3".to_string()),
            },
            Advisory {
                id: "RUSTSEC-2024-0388".to_string(),
                title: "`derivative` is unmaintained; consider using an alternative".to_string(),
                description: "The derivative crate is no longer maintained".to_string(),
                severity: Severity::Low,
                affected_crates: vec![AffectedCrate {
                    name: "derivative".to_string(),
                    affected_versions: VersionRange {
                        min_version: None,
                        max_version: None,
                        excluded_versions: vec![],
                    },
                    patched_versions: vec![],
                }],
                date: chrono::DateTime::parse_from_rfc3339("2024-06-26T00:00:00Z").unwrap().with_timezone(&Utc),
                url: "https://rustsec.org/advisories/RUSTSEC-2024-0388".to_string(),
                solution: Some("Consider using derive_more or similar alternatives".to_string()),
            },
            Advisory {
                id: "RUSTSEC-2024-0436".to_string(),
                title: "paste - no longer maintained".to_string(),
                description: "The paste crate is no longer maintained".to_string(),
                severity: Severity::Low,
                affected_crates: vec![AffectedCrate {
                    name: "paste".to_string(),
                    affected_versions: VersionRange {
                        min_version: None,
                        max_version: None,
                        excluded_versions: vec![],
                    },
                    patched_versions: vec![],
                }],
                date: chrono::DateTime::parse_from_rfc3339("2024-10-07T00:00:00Z").unwrap().with_timezone(&Utc),
                url: "https://rustsec.org/advisories/RUSTSEC-2024-0436".to_string(),
                solution: Some("Consider alternatives or vendor the code".to_string()),
            },
            Advisory {
                id: "RUSTSEC-2023-0033".to_string(),
                title: "Parsing borsh messages with ZST which are not-copy/clone is unsound".to_string(),
                description: "Unsound parsing of zero-sized types in borsh".to_string(),
                severity: Severity::High,
                affected_crates: vec![AffectedCrate {
                    name: "borsh".to_string(),
                    affected_versions: VersionRange {
                        min_version: None,
                        max_version: Some("0.9.3".to_string()),
                        excluded_versions: vec![],
                    },
                    patched_versions: vec!["0.10.0".to_string()],
                }],
                date: chrono::DateTime::parse_from_rfc3339("2023-04-12T00:00:00Z").unwrap().with_timezone(&Utc),
                url: "https://rustsec.org/advisories/RUSTSEC-2023-0033".to_string(),
                solution: Some("Upgrade to >=0.10.0".to_string()),
            },
        ];

        for advisory in advisories {
            self.advisory_db.advisories.insert(advisory.id.clone(), advisory.clone());
            
            for affected_crate in &advisory.affected_crates {
                self.advisory_db.crate_advisories
                    .entry(affected_crate.name.clone())
                    .or_insert_with(Vec::new)
                    .push(advisory.id.clone());
            }
        }

        Ok(())
    }

    fn is_version_affected(&self, version: &str, advisory: &Advisory) -> Result<bool> {
        for affected_crate in &advisory.affected_crates {
            if self.version_in_range(version, &affected_crate.affected_versions)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn version_in_range(&self, version: &str, range: &VersionRange) -> Result<bool> {
        // Simplified version comparison - in production would use semver crate
        if let Some(min) = &range.min_version {
            if version < min.as_str() {
                return Ok(false);
            }
        }
        
        if let Some(max) = &range.max_version {
            if version > max.as_str() {
                return Ok(false);
            }
        }
        
        if range.excluded_versions.contains(&version.to_string()) {
            return Ok(false);
        }
        
        Ok(true)
    }

    fn calculate_severity_score(&self, severity: Severity) -> f32 {
        match severity {
            Severity::Low => 2.5,
            Severity::Medium => 5.0,
            Severity::High => 7.5,
            Severity::Critical => 10.0,
        }
    }

    fn get_database_version(&self) -> String {
        format!("local-{}", self.last_update.format("%Y%m%d"))
    }

    fn get_safe_version(&self, advisory: &Advisory) -> Option<String> {
        advisory.affected_crates.first()
            .and_then(|crate_info| crate_info.patched_versions.first())
            .cloned()
    }

    fn log_scan_results(&self, result: &ScanResult) -> Result<()> {
        msg!("CVE Scan completed:");
        msg!("  - Crates scanned: {}", result.total_crates_scanned);
        msg!("  - Vulnerabilities found: {}", result.vulnerabilities.len());
        msg!("  - Scan duration: {}ms", result.scan_duration_ms);
        
        for vuln in &result.vulnerabilities {
            let status = if vuln.is_allowed { "ALLOWED" } else { "BLOCKED" };
            msg!("  - {} [{}]: {} v{} - {} ({})", 
                 status, vuln.advisory.severity as u8, 
                 vuln.affected_crate, vuln.current_version,
                 vuln.advisory.title, vuln.advisory.id);
        }
        
        Ok(())
    }
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            max_allowed_severity: Severity::Medium,
            auto_update_db: true,
            fail_on_vulnerability: true,
            allowed_vulnerabilities: HashSet::new(),
        }
    }
}

/// Dependency information for scanning
#[derive(Debug, Clone)]
pub struct CrateDependency {
    pub name: String,
    pub version: String,
    pub source: String,
}

/// Security recommendation for vulnerable dependencies
#[derive(Debug, Clone)]
pub struct SecurityRecommendation {
    pub crate_name: String,
    pub current_version: String,
    pub recommendation_type: RecommendationType,
    pub suggested_version: Option<String>,
    pub rationale: String,
    pub urgency: Urgency,
}

#[derive(Debug, Clone)]
pub enum RecommendationType {
    ImmediateUpdate,
    ScheduledUpdate,
    ConsiderUpdate,
    FindAlternative,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Urgency {
    Low,
    Medium,
    High,
    Immediate,
}

/// Policy validation result
#[derive(Debug, Clone)]
pub struct PolicyValidationResult {
    pub is_compliant: bool,
    pub violations: Vec<PolicyViolation>,
    pub critical_vulnerabilities: usize,
    pub high_vulnerabilities: usize,
    pub total_vulnerabilities: usize,
}

/// Individual policy violation
#[derive(Debug, Clone)]
pub struct PolicyViolation {
    pub rule: String,
    pub crate_name: String,
    pub advisory_id: String,
    pub severity: Severity,
}