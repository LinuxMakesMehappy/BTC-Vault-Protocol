use anchor_lang::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use crate::errors::VaultError;
use crate::security::cve_scanner::{CveScanner, CrateDependency, ScanResult, Advisory, Severity};

/// Automated dependency management with security validation
#[derive(Debug, Clone)]
pub struct DependencyManager {
    cve_scanner: CveScanner,
    policy: SecurityPolicy,
    dependency_cache: DependencyCache,
    alternatives_db: AlternativesDatabase,
}

/// Security policy for dependency management
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub zero_cve_mode: bool,
    pub max_vulnerability_age_days: u32,
    pub auto_reject_critical: bool,
    pub auto_reject_high: bool,
    pub require_manual_approval: HashSet<String>, // Crate names requiring manual approval
    pub blocked_crates: HashSet<String>,
    pub trusted_publishers: HashSet<String>,
}

/// Cache for dependency information and scan results
#[derive(Debug, Clone, Default)]
pub struct DependencyCache {
    scan_results: HashMap<String, CachedScanResult>, // crate_name:version -> result
    last_global_scan: Option<DateTime<Utc>>,
    cache_ttl_hours: u32,
}

/// Cached scan result with timestamp
#[derive(Debug, Clone)]
pub struct CachedScanResult {
    pub result: ScanResult,
    pub timestamp: DateTime<Utc>,
    pub is_valid: bool,
}

/// Database of alternative crates for vulnerable dependencies
#[derive(Debug, Clone, Default)]
pub struct AlternativesDatabase {
    alternatives: HashMap<String, Vec<Alternative>>, // vulnerable_crate -> alternatives
}

/// Alternative crate suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub name: String,
    pub version: String,
    pub compatibility_score: f32, // 0.0 to 1.0
    pub security_score: f32,      // 0.0 to 1.0
    pub maintenance_score: f32,   // 0.0 to 1.0
    pub migration_effort: MigrationEffort,
    pub description: String,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationEffort {
    Trivial,    // Drop-in replacement
    Easy,       // Minor API changes
    Moderate,   // Significant refactoring
    Difficult,  // Major rewrite required
}

/// Dependency validation result
#[derive(Debug, Clone)]
pub struct DependencyValidationResult {
    pub is_approved: bool,
    pub crate_name: String,
    pub version: String,
    pub vulnerabilities: Vec<Advisory>,
    pub policy_violations: Vec<String>,
    pub alternatives: Vec<Alternative>,
    pub recommendation: DependencyRecommendation,
}

#[derive(Debug, Clone)]
pub enum DependencyRecommendation {
    Approve,
    ApproveWithWarning(String),
    Reject(String),
    RequiresManualReview(String),
    SuggestAlternative(Vec<Alternative>),
}

/// Update report for dependency changes
#[derive(Debug, Clone)]
pub struct UpdateReport {
    pub updated_crates: Vec<CrateUpdate>,
    pub rejected_updates: Vec<RejectedUpdate>,
    pub security_improvements: Vec<SecurityImprovement>,
    pub total_vulnerabilities_fixed: usize,
}

#[derive(Debug, Clone)]
pub struct CrateUpdate {
    pub name: String,
    pub from_version: String,
    pub to_version: String,
    pub update_reason: UpdateReason,
}

#[derive(Debug, Clone)]
pub enum UpdateReason {
    SecurityFix,
    PolicyCompliance,
    DependencyRequirement,
    MaintenanceUpdate,
}

#[derive(Debug, Clone)]
pub struct RejectedUpdate {
    pub name: String,
    pub version: String,
    pub reason: String,
    pub alternatives: Vec<Alternative>,
}

#[derive(Debug, Clone)]
pub struct SecurityImprovement {
    pub cve_id: String,
    pub severity: Severity,
    pub fixed_by_update: String,
    pub impact_description: String,
}

impl DependencyManager {
    /// Create a new dependency manager with default security policy
    pub fn new() -> Self {
        Self {
            cve_scanner: CveScanner::new(),
            policy: SecurityPolicy::default(),
            dependency_cache: DependencyCache::default(),
            alternatives_db: AlternativesDatabase::default(),
        }
    }

    /// Create dependency manager with custom policy
    pub fn with_policy(policy: SecurityPolicy) -> Self {
        let mut manager = Self::new();
        manager.policy = policy;
        manager.load_alternatives_database().unwrap_or_else(|e| {
            msg!("Warning: Failed to load alternatives database: {}", e);
        });
        manager
    }

    /// Initialize the dependency manager
    pub async fn initialize(&mut self) -> Result<()> {
        msg!("Initializing dependency manager...");
        
        // Update CVE database
        self.cve_scanner.update_database().await?;
        
        // Load alternatives database
        self.load_alternatives_database()?;
        
        // Set cache TTL
        self.dependency_cache.cache_ttl_hours = 24;
        
        msg!("Dependency manager initialized successfully");
        Ok(())
    }

    /// Validate a dependency against security policy
    pub async fn validate_dependency(&mut self, name: &str, version: &str) -> Result<DependencyValidationResult> {
        msg!("Validating dependency: {} v{}", name, version);

        // Check cache first
        let cache_key = format!("{}:{}", name, version);
        if let Some(cached) = self.get_cached_result(&cache_key) {
            if cached.is_valid && self.is_cache_fresh(&cached.timestamp) {
                return Ok(self.build_validation_result_from_cache(name, version, &cached));
            }
        }

        // Check if crate is blocked
        if self.policy.blocked_crates.contains(name) {
            return Ok(DependencyValidationResult {
                is_approved: false,
                crate_name: name.to_string(),
                version: version.to_string(),
                vulnerabilities: vec![],
                policy_violations: vec![format!("Crate '{}' is explicitly blocked", name)],
                alternatives: self.get_alternatives(name),
                recommendation: DependencyRecommendation::Reject(
                    format!("Crate '{}' is on the blocked list", name)
                ),
            });
        }

        // Scan for vulnerabilities
        let dependency = CrateDependency {
            name: name.to_string(),
            version: version.to_string(),
            source: "crates.io".to_string(),
        };

        let vulnerabilities = self.cve_scanner.check_crate_vulnerability(name, version)?;
        
        // Apply security policy
        let mut policy_violations = Vec::new();
        let mut is_approved = true;

        // Zero-CVE mode check
        if self.policy.zero_cve_mode && !vulnerabilities.is_empty() {
            is_approved = false;
            policy_violations.push("Zero-CVE policy violated: vulnerabilities found".to_string());
        }

        // Severity-based checks
        for vuln in &vulnerabilities {
            match vuln.severity {
                Severity::Critical if self.policy.auto_reject_critical => {
                    is_approved = false;
                    policy_violations.push(format!("Critical vulnerability rejected: {}", vuln.id));
                },
                Severity::High if self.policy.auto_reject_high => {
                    is_approved = false;
                    policy_violations.push(format!("High severity vulnerability rejected: {}", vuln.id));
                },
                _ => {}
            }

            // Check vulnerability age
            let age_days = (Utc::now() - vuln.date).num_days() as u32;
            if age_days > self.policy.max_vulnerability_age_days {
                policy_violations.push(format!(
                    "Vulnerability {} is {} days old (max allowed: {})", 
                    vuln.id, age_days, self.policy.max_vulnerability_age_days
                ));
            }
        }

        // Manual approval check
        if self.policy.require_manual_approval.contains(name) {
            return Ok(DependencyValidationResult {
                is_approved: false,
                crate_name: name.to_string(),
                version: version.to_string(),
                vulnerabilities,
                policy_violations,
                alternatives: self.get_alternatives(name),
                recommendation: DependencyRecommendation::RequiresManualReview(
                    format!("Crate '{}' requires manual security review", name)
                ),
            });
        }

        // Determine recommendation
        let recommendation = if !is_approved {
            if vulnerabilities.is_empty() {
                DependencyRecommendation::Reject(policy_violations.join("; "))
            } else {
                let alternatives = self.get_alternatives(name);
                if alternatives.is_empty() {
                    DependencyRecommendation::Reject(format!(
                        "Vulnerabilities found and no alternatives available: {}", 
                        vulnerabilities.iter().map(|v| v.id.as_str()).collect::<Vec<_>>().join(", ")
                    ))
                } else {
                    DependencyRecommendation::SuggestAlternative(alternatives.clone())
                }
            }
        } else if !vulnerabilities.is_empty() {
            DependencyRecommendation::ApproveWithWarning(format!(
                "Approved with {} known vulnerabilities", vulnerabilities.len()
            ))
        } else {
            DependencyRecommendation::Approve
        };

        let result = DependencyValidationResult {
            is_approved,
            crate_name: name.to_string(),
            version: version.to_string(),
            vulnerabilities,
            policy_violations,
            alternatives: self.get_alternatives(name),
            recommendation,
        };

        // Cache the result
        self.cache_validation_result(&cache_key, &result);

        Ok(result)
    }

    /// Get alternative crates for a vulnerable dependency
    pub fn get_alternatives(&self, crate_name: &str) -> Vec<Alternative> {
        self.alternatives_db.alternatives
            .get(crate_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Suggest secure alternatives for a vulnerable crate
    pub fn suggest_secure_alternatives(&self, crate_name: &str) -> Result<Vec<Alternative>> {
        let alternatives = self.get_alternatives(crate_name);
        
        // Filter and sort by security score
        let mut secure_alternatives: Vec<_> = alternatives
            .into_iter()
            .filter(|alt| alt.security_score >= 0.8) // Only high-security alternatives
            .collect();
        
        secure_alternatives.sort_by(|a, b| {
            b.security_score.partial_cmp(&a.security_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(secure_alternatives)
    }

    /// Perform automated security updates
    pub async fn auto_update_secure(&mut self, current_dependencies: &[CrateDependency]) -> Result<UpdateReport> {
        msg!("Performing automated security updates...");
        
        let mut updated_crates = Vec::new();
        let mut rejected_updates = Vec::new();
        let mut security_improvements = Vec::new();

        for dep in current_dependencies {
            // Check if current version has vulnerabilities
            let vulnerabilities = self.cve_scanner.check_crate_vulnerability(&dep.name, &dep.version)?;
            
            if !vulnerabilities.is_empty() {
                // Try to find a secure version
                if let Some(secure_version) = self.find_secure_version(&dep.name, &vulnerabilities) {
                    // Validate the secure version
                    let validation = self.validate_dependency(&dep.name, &secure_version).await?;
                    
                    if validation.is_approved {
                        updated_crates.push(CrateUpdate {
                            name: dep.name.clone(),
                            from_version: dep.version.clone(),
                            to_version: secure_version,
                            update_reason: UpdateReason::SecurityFix,
                        });

                        // Record security improvements
                        for vuln in vulnerabilities {
                            security_improvements.push(SecurityImprovement {
                                cve_id: vuln.id,
                                severity: vuln.severity,
                                fixed_by_update: dep.name.clone(),
                                impact_description: vuln.title,
                            });
                        }
                    } else {
                        rejected_updates.push(RejectedUpdate {
                            name: dep.name.clone(),
                            version: secure_version,
                            reason: validation.policy_violations.join("; "),
                            alternatives: validation.alternatives,
                        });
                    }
                } else {
                    // No secure version available, suggest alternatives
                    let alternatives = self.suggest_secure_alternatives(&dep.name)?;
                    rejected_updates.push(RejectedUpdate {
                        name: dep.name.clone(),
                        version: dep.version.clone(),
                        reason: "No secure version available".to_string(),
                        alternatives,
                    });
                }
            }
        }

        let total_vulnerabilities_fixed = security_improvements.len();
        let report = UpdateReport {
            updated_crates,
            rejected_updates,
            security_improvements,
            total_vulnerabilities_fixed,
        };

        self.log_update_report(&report)?;
        Ok(report)
    }

    /// Check if the dependency manager is in zero-CVE compliance
    pub async fn verify_zero_cve_compliance(&mut self, dependencies: &[CrateDependency]) -> Result<ComplianceReport> {
        msg!("Verifying zero-CVE compliance...");
        
        let scan_result = self.cve_scanner.scan_dependencies(dependencies)?;
        let policy_validation = self.cve_scanner.validate_security_policy(&scan_result)?;
        
        let is_compliant = policy_validation.is_compliant && scan_result.vulnerabilities.is_empty();
        
        Ok(ComplianceReport {
            is_zero_cve_compliant: is_compliant,
            total_vulnerabilities: scan_result.vulnerabilities.len(),
            critical_vulnerabilities: policy_validation.critical_vulnerabilities,
            high_vulnerabilities: policy_validation.high_vulnerabilities,
            policy_violations: policy_validation.violations,
            scan_result,
            recommendations: if !is_compliant {
                self.generate_compliance_recommendations(dependencies).await?
            } else {
                vec![]
            },
        })
    }

    // Private helper methods
    fn load_alternatives_database(&mut self) -> Result<()> {
        // Load known alternatives for vulnerable crates
        let alternatives = vec![
            ("curve25519-dalek".to_string(), vec![
                Alternative {
                    name: "ed25519-dalek".to_string(),
                    version: "2.0.0".to_string(),
                    compatibility_score: 0.8,
                    security_score: 0.95,
                    maintenance_score: 0.9,
                    migration_effort: MigrationEffort::Moderate,
                    description: "EdDSA signature library with better security guarantees".to_string(),
                    pros: vec![
                        "No known timing vulnerabilities".to_string(),
                        "Actively maintained".to_string(),
                        "Formal verification available".to_string(),
                    ],
                    cons: vec![
                        "Different API".to_string(),
                        "May require cryptographic protocol changes".to_string(),
                    ],
                },
            ]),
            ("derivative".to_string(), vec![
                Alternative {
                    name: "derive_more".to_string(),
                    version: "0.99.17".to_string(),
                    compatibility_score: 0.9,
                    security_score: 0.85,
                    maintenance_score: 0.95,
                    migration_effort: MigrationEffort::Easy,
                    description: "Actively maintained derive macro library".to_string(),
                    pros: vec![
                        "Drop-in replacement for most use cases".to_string(),
                        "Actively maintained".to_string(),
                        "More features".to_string(),
                    ],
                    cons: vec![
                        "Slightly different syntax for some derives".to_string(),
                    ],
                },
            ]),
            ("paste".to_string(), vec![
                Alternative {
                    name: "proc-macro2".to_string(),
                    version: "1.0.70".to_string(),
                    compatibility_score: 0.6,
                    security_score: 0.9,
                    maintenance_score: 0.95,
                    migration_effort: MigrationEffort::Difficult,
                    description: "Low-level proc-macro implementation".to_string(),
                    pros: vec![
                        "Core infrastructure crate".to_string(),
                        "Highly maintained".to_string(),
                        "Better performance".to_string(),
                    ],
                    cons: vec![
                        "Requires significant refactoring".to_string(),
                        "More complex API".to_string(),
                    ],
                },
            ]),
            ("borsh".to_string(), vec![
                Alternative {
                    name: "bincode".to_string(),
                    version: "1.3.3".to_string(),
                    compatibility_score: 0.7,
                    security_score: 0.9,
                    maintenance_score: 0.85,
                    migration_effort: MigrationEffort::Moderate,
                    description: "Binary serialization with better security".to_string(),
                    pros: vec![
                        "No known parsing vulnerabilities".to_string(),
                        "Good performance".to_string(),
                        "Serde integration".to_string(),
                    ],
                    cons: vec![
                        "Different serialization format".to_string(),
                        "May break compatibility".to_string(),
                    ],
                },
            ]),
        ];

        for (crate_name, alts) in alternatives {
            self.alternatives_db.alternatives.insert(crate_name, alts);
        }

        Ok(())
    }

    fn get_cached_result(&self, cache_key: &str) -> Option<&CachedScanResult> {
        self.dependency_cache.scan_results.get(cache_key)
    }

    fn is_cache_fresh(&self, timestamp: &DateTime<Utc>) -> bool {
        let age_hours = (Utc::now() - *timestamp).num_hours() as u32;
        age_hours < self.dependency_cache.cache_ttl_hours
    }

    fn build_validation_result_from_cache(&self, name: &str, version: &str, cached: &CachedScanResult) -> DependencyValidationResult {
        // Simplified - in production would properly reconstruct from cache
        DependencyValidationResult {
            is_approved: cached.result.vulnerabilities.is_empty(),
            crate_name: name.to_string(),
            version: version.to_string(),
            vulnerabilities: vec![], // Would extract from cached result
            policy_violations: vec![],
            alternatives: self.get_alternatives(name),
            recommendation: DependencyRecommendation::Approve,
        }
    }

    fn cache_validation_result(&mut self, cache_key: &str, result: &DependencyValidationResult) {
        // Simplified caching - in production would store full scan result
        let cached = CachedScanResult {
            result: ScanResult {
                vulnerabilities: vec![],
                total_crates_scanned: 1,
                scan_duration_ms: 0,
                database_version: "cached".to_string(),
            },
            timestamp: Utc::now(),
            is_valid: true,
        };
        
        self.dependency_cache.scan_results.insert(cache_key.to_string(), cached);
    }

    fn find_secure_version(&self, crate_name: &str, vulnerabilities: &[Advisory]) -> Option<String> {
        // Find the highest patched version across all vulnerabilities
        let mut patched_versions = Vec::new();
        
        for vuln in vulnerabilities {
            for affected_crate in &vuln.affected_crates {
                if affected_crate.name == crate_name {
                    patched_versions.extend(affected_crate.patched_versions.iter().cloned());
                }
            }
        }
        
        // Return the highest version (simplified - would use proper semver comparison)
        patched_versions.into_iter().max()
    }

    fn log_update_report(&self, report: &UpdateReport) -> Result<()> {
        msg!("Dependency Update Report:");
        msg!("  - Updated crates: {}", report.updated_crates.len());
        msg!("  - Rejected updates: {}", report.rejected_updates.len());
        msg!("  - Vulnerabilities fixed: {}", report.total_vulnerabilities_fixed);
        
        for update in &report.updated_crates {
            msg!("  ✅ {} {} -> {} ({})", 
                 update.name, update.from_version, update.to_version,
                 match update.update_reason {
                     UpdateReason::SecurityFix => "security fix",
                     UpdateReason::PolicyCompliance => "policy compliance",
                     UpdateReason::DependencyRequirement => "dependency requirement",
                     UpdateReason::MaintenanceUpdate => "maintenance",
                 });
        }
        
        for rejection in &report.rejected_updates {
            msg!("  ❌ {} v{}: {}", rejection.name, rejection.version, rejection.reason);
        }
        
        Ok(())
    }

    async fn generate_compliance_recommendations(&self, dependencies: &[CrateDependency]) -> Result<Vec<ComplianceRecommendation>> {
        let mut recommendations = Vec::new();
        
        for dep in dependencies {
            let vulnerabilities = self.cve_scanner.check_crate_vulnerability(&dep.name, &dep.version)?;
            
            if !vulnerabilities.is_empty() {
                let alternatives = self.suggest_secure_alternatives(&dep.name)?;
                
                recommendations.push(ComplianceRecommendation {
                    crate_name: dep.name.clone(),
                    current_version: dep.version.clone(),
                    issue: format!("{} vulnerabilities found", vulnerabilities.len()),
                    action: if alternatives.is_empty() {
                        "Remove dependency or find secure alternative".to_string()
                    } else {
                        format!("Replace with: {}", alternatives[0].name)
                    },
                    priority: if vulnerabilities.iter().any(|v| v.severity >= Severity::High) {
                        Priority::High
                    } else {
                        Priority::Medium
                    },
                });
            }
        }
        
        Ok(recommendations)
    }
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            zero_cve_mode: true,
            max_vulnerability_age_days: 90,
            auto_reject_critical: true,
            auto_reject_high: true,
            require_manual_approval: HashSet::new(),
            blocked_crates: HashSet::new(),
            trusted_publishers: HashSet::new(),
        }
    }
}

/// Compliance report for zero-CVE verification
#[derive(Debug, Clone)]
pub struct ComplianceReport {
    pub is_zero_cve_compliant: bool,
    pub total_vulnerabilities: usize,
    pub critical_vulnerabilities: usize,
    pub high_vulnerabilities: usize,
    pub policy_violations: Vec<crate::security::cve_scanner::PolicyViolation>,
    pub scan_result: ScanResult,
    pub recommendations: Vec<ComplianceRecommendation>,
}

#[derive(Debug, Clone)]
pub struct ComplianceRecommendation {
    pub crate_name: String,
    pub current_version: String,
    pub issue: String,
    pub action: String,
    pub priority: Priority,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}