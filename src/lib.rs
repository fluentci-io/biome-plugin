use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let path = dag().get_env("PATH")?;
    let home = dag().get_env("HOME")?;
    dag().set_envs(vec![("PATH".into(), format!("{}/.bun/bin:{}", home, path))])?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "bun",
            "install",
            "-g",
            &format!("@biomejs/biome@{}", version),
        ])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn format(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["bunx", "@biomejs/biome", "format", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn lint(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["bunx", "@biomejs/biome", "lint", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn ci(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["bunx", "@biomejs/biome", "ci", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn check(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["bunx", "@biomejs/biome", "check", &args])?
        .stdout()?;
    Ok(stdout)
}
