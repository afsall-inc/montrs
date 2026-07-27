use playwright::Playwright;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let playwright = Playwright::initialize().await?;
    playwright.prepare()?;
    
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;

    page.goto_builder("http://localhost:3000").goto().await?;
    println!("Successfully navigated to http://localhost:3000");

    Ok(())
}
