mod terminal_app;
mod tetris_engine;

fn main() -> std::io::Result<()> {
    let mut app = terminal_app::App::new();
    let mut tetris = tetris_engine::TetrisEngine::new();
    app.setup()?;
    app.run(&mut tetris)?;
    app.teardown()?;
    Ok(())
}
