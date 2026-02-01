use cargo_generate::{GenerateArgs, TemplatePath, generate};
use console::style;
use std::env;

pub async fn run(name: String, template_url: String) -> anyhow::Result<()> {
    println!(
        "{} Creating new MontRS project: {}",
        style("🚀").bold(),
        style(&name).cyan().bold()
    );

    // Use local template path
    let template_path = format!("templates/{}", template_url);

    // In a real CLI, we might use include_dir! to embed templates
    // or look relative to the binary path. For now, we look in the CWD
    // assuming the user is in the montrs root.

    let args = GenerateArgs {
        name: Some(name.clone()),
        template_path: TemplatePath {
            path: Some(template_path),
            ..Default::default()
        },
        destination: Some(env::current_dir()?),
        force: false,
        verbose: true,
        ..Default::default()
    };

    generate(args).map_err(|e| anyhow::anyhow!("Scaffolding failed: {}", e))?;

    println!(
        "\n{} Project {} created successfully!",
        style("✨").green().bold(),
        style(&name).cyan().bold()
    );
    println!("Next steps:\n  cd {}\n  montrs build\n  montrs serve", name);

    Ok(())
}
