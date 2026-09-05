pub struct Dependencies {
    fzf: bool,
    bat: bool,
    rg: bool,
}

impl Dependencies {
    pub fn check() -> Self {
        Self {
            fzf: which::which("fzf").is_ok(),
            bat: which::which("bat").is_ok(),
            rg: which::which("rg").is_ok(),
        }
    }

    pub fn ensure_fzf(&self) -> Result<(), anyhow::Error> {
        if !self.fzf {
            return Err(anyhow::anyhow!("fzf not found. Please install it."))
        }
        Ok(())
    }

    pub fn warn_optional(&self) {
        if !self.bat {
            eprintln!("bat not found. Install for better syntax highlighting.");
        }
        if !self.rg {
            eprintln!("ripgrep not found. Install for content search.");
        }
    }
}
