use anyhow::Result;
use prompted::input;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Diagnostics, Source, Sources, Vm};
use std::sync::Arc;

fn main() -> Result<()> {
    let context = rune_modules::default_context()?;
    let runtime = Arc::new(context.runtime()?);
    let mut src = "".to_string();
    let mut src_old;

    loop {
        src_old = src.clone();
        let src_add = input!("> ");
        if !src.is_empty() && !src.trim().ends_with(';') {
            src = format!("{src};");
        }
        src = format!("{src}\n{src_add}");
        let mut sources = Sources::new();
        sources.insert(Source::memory(format!("pub fn repl() {{ \n{}\n }}", src))?)?;

        let mut diagnostics = Diagnostics::new();

        let result = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build();

        if result.is_err() {
            if !diagnostics.is_empty() {
                let mut writer = StandardStream::stderr(ColorChoice::Always);
                diagnostics.emit(&mut writer, &sources)?;
            }
            src = src_old;
            continue;
        }

        let unit = result?;

        let mut vm = Vm::new(runtime.clone(), Arc::new(unit));

        let output = vm.call(["repl"], ())?;

        println!("{:?}", output);
    }
}
