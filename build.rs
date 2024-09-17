use vergen_git2::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    === ENV VARIABLES: ===
    VERGEN_BUILD_TIMESTAMP
    VERGEN_CARGO_TARGET_TRIPLE
    VERGEN_GIT_BRANCH
    VERGEN_GIT_COMMIT_TIMESTAMP
    VERGEN_GIT_SHA
    VERGEN_RUSTC_CHANNEL
    VERGEN_RUSTC_COMMIT_DATE
    VERGEN_RUSTC_COMMIT_HASH
    VERGEN_RUSTC_SEMVER
    VERGEN_SYSINFO_OS_VERSION
    VERGEN_SYSINFO_TOTAL_MEMORY
    VERGEN_SYSINFO_CPU_CORE_COUNT
    VERGEN_SYSINFO_CPU_BRAND
    VERGEN_SYSINFO_CPU_FREQUENCY
    */

    let build = BuildBuilder::default().build_timestamp(true).build()?;
    let cargo = CargoBuilder::default().target_triple(true).build()?;
    let git2 = Git2Builder::default()
        .branch(true)
        .commit_timestamp(true)
        .sha(true)
        .build()?;
    let rustc = RustcBuilder::default()
        .channel(true)
        .commit_date(true)
        .commit_hash(true)
        .semver(true)
        .build()?;
    let si = SysinfoBuilder::default()
        .os_version(true)
        .memory(true)
        .cpu_core_count(true)
        .cpu_brand(true)
        .cpu_frequency(true)
        .build()?;

    Emitter::default()
        // .idempotent()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&git2)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?
        .emit()?;

    // Create `CARGO_PKG_DESCRIPTION_LONG` from `CARGO_PKG_DESCRIPTION`
    // Then override `CARGO_PKG_DESCRIPTION` with truncated version of `CARGO_PKG_DESCRIPTION`
    let mut long_description = env!("CARGO_PKG_DESCRIPTION").to_string();
    let delim_indexes: Vec<_> = long_description.match_indices("\\").collect();

    let mut short_description: String = long_description.clone();
    short_description.replace_range(delim_indexes[0].0..long_description.len(), "");
    long_description.replace_range(delim_indexes[0].0..delim_indexes[0].0 + 1, " ");

    println!(
        "cargo::rustc-env=CARGO_PKG_DESCRIPTION={}",
        short_description
    );
    println!(
        "cargo::rustc-env=CARGO_PKG_DESCRIPTION_LONG={}",
        long_description
    );

    Ok(())
}
