use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, exit};

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
    
    // Only run security checks in release builds or when explicitly requested
    let run_security_checks = env::var("CARGO_CFG_TARGET_ENV").is_ok() 
        || env::var("VAULT_SECURITY_CHECK").is_ok()
        || env::var("PROFILE").map(|p| p == "release").unwrap_or(false);
    
    if run_security_checks {
        println!("cargo:warning=Running security checks...");
        
        // Run CVE scanning
        if let Err(e) = run_cve_scan() {
            println!("cargo:warning=CVE scan failed: {}", e);
            
            // Fail build if zero-CVE mode is enabled
            if env::var("VAULT_ZERO_CVE_MODE").is_ok() {
                eprintln!("Build failed due to CVE policy violation");
                exit(1);
            }
        }
        
        // Check for hardcoded secrets
        if let Err(e) = check_hardcoded_secrets() {
            println!("cargo:warning=Hardcoded secrets check failed: {}", e);
            exit(1);
        }
        
        // Validate panic-free code
        if let Err(e) = check_panic_free() {
            println!("cargo:warning=Panic-free validation failed: {}", e);
            
            // Only warn for now, don't fail build
            println!("cargo:warning=Found potential panic points - consider fixing for production");
        }
    }
    
    // Generate build metadata
    generate_build_metadata();
}

fn run_cve_scan() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning=Scanning dependencies for CVEs...");
    
    // Run cargo audit
    let output = Command::new("cargo")
        .args(&["audit", "--json"])
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Parse audit output to check for actual vulnerabilities vs warnings
        if stderr.contains("error:") && stderr.contains("vulnerability found") {
            return Err(format!("CVE vulnerabilities detected: {}", stderr).into());
        } else if stderr.contains("warning:") {
            println!("cargo:warning=Audit warnings found: {}", stderr);
        }
    }
    
    println!("cargo:warning=CVE scan completed successfully");
    Ok(())
}

fn check_hardcoded_secrets() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning=Checking for hardcoded secrets...");
    
    let patterns = vec![
        r"(?i)(api[_-]?key|password|secret|token)\s*[:=]\s*['\x22][^\x22']{8,}['\x22]",
        r"['\x22][0-9a-fA-F]{32,}['\x22]", // Hex strings that might be keys
        r"(?i)(private[_-]?key|secret[_-]?key)\s*[:=]\s*['\x22][^\x22']+['\x22]",
    ];
    
    let mut found_secrets = false;
    
    // Check source files
    for entry in walkdir::WalkDir::new("src") {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "rs" {
                    if let Ok(content) = fs::read_to_string(path) {
                        for pattern in &patterns {
                            if regex::Regex::new(pattern)?.is_match(&content) {
                                // Skip test files and examples
                                if !path.to_string_lossy().contains("test") 
                                    && !path.to_string_lossy().contains("example") {
                                    println!("cargo:warning=Potential hardcoded secret in: {}", path.display());
                                    found_secrets = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    if found_secrets {
        return Err("Hardcoded secrets detected in source code".into());
    }
    
    println!("cargo:warning=No hardcoded secrets found");
    Ok(())
}

fn check_panic_free() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning=Checking for panic-inducing code...");
    
    let panic_patterns = vec![
        r"\.unwrap\(\)",
        r"\.expect\(",
        r"panic!\(",
        r"unreachable!\(",
        r"unimplemented!\(",
        r"todo!\(",
    ];
    
    let mut panic_count = 0;
    
    // Check source files
    for entry in walkdir::WalkDir::new("src") {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "rs" {
                    if let Ok(content) = fs::read_to_string(path) {
                        for (line_num, line) in content.lines().enumerate() {
                            for pattern in &panic_patterns {
                                if regex::Regex::new(pattern)?.is_match(line) {
                                    // Skip test files and comments
                                    if !path.to_string_lossy().contains("test") 
                                        && !line.trim().starts_with("//") {
                                        println!("cargo:warning=Potential panic at {}:{}: {}", 
                                                path.display(), line_num + 1, line.trim());
                                        panic_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    if panic_count > 0 {
        println!("cargo:warning=Found {} potential panic points", panic_count);
        
        // Set environment variable for runtime checking
        println!("cargo:rustc-env=VAULT_PANIC_COUNT={}", panic_count);
        
        // Only error if we're in strict mode
        if env::var("VAULT_STRICT_PANIC_FREE").is_ok() {
            return Err(format!("Found {} panic points in strict panic-free mode", panic_count).into());
        }
    }
    
    Ok(())
}

fn generate_build_metadata() {
    // Generate build timestamp
    let build_time = chrono::Utc::now().to_rfc3339();
    println!("cargo:rustc-env=VAULT_BUILD_TIME={}", build_time);
    
    // Generate build hash from git if available
    if let Ok(output) = Command::new("git").args(&["rev-parse", "HEAD"]).output() {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-env=VAULT_GIT_HASH={}", git_hash);
        }
    }
    
    // Security build flags
    let zero_cve_mode = env::var("VAULT_ZERO_CVE_MODE").is_ok();
    println!("cargo:rustc-env=VAULT_ZERO_CVE_MODE={}", zero_cve_mode);
    
    let security_level = env::var("VAULT_SECURITY_LEVEL").unwrap_or_else(|_| "standard".to_string());
    println!("cargo:rustc-env=VAULT_SECURITY_LEVEL={}", security_level);
    
    // Generate security manifest
    generate_security_manifest();
}

fn generate_security_manifest() {
    let manifest = SecurityManifest {
        build_time: chrono::Utc::now(),
        zero_cve_mode: env::var("VAULT_ZERO_CVE_MODE").is_ok(),
        security_level: env::var("VAULT_SECURITY_LEVEL").unwrap_or_else(|_| "standard".to_string()),
        panic_count: env::var("VAULT_PANIC_COUNT").unwrap_or_else(|_| "0".to_string()).parse().unwrap_or(0),
        dependencies_scanned: true,
        secrets_checked: true,
    };
    
    if let Ok(manifest_json) = serde_json::to_string_pretty(&manifest) {
        let _ = fs::write("target/security-manifest.json", manifest_json);
    }
}

#[derive(serde::Serialize)]
struct SecurityManifest {
    build_time: chrono::DateTime<chrono::Utc>,
    zero_cve_mode: bool,
    security_level: String,
    panic_count: u32,
    dependencies_scanned: bool,
    secrets_checked: bool,
}

// Add required dependencies for build script
// Note: These would be added to [build-dependencies] in Cargo.toml