use druid::{Selector, AppDelegate, Handled};

use crate::{AppState, parser::Parser, cpu::Cpu};

pub const SHOULD: Selector = Selector::new("cpu.should_rerender");

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        if cmd.is(SHOULD) {
            let mut empty = false;

            if data.input.is_empty() {
                data.cpu.parser = Parser::default();
                empty = true;
            } else {
                data.cpu = Cpu {
                    parser: Parser::new(&data.input),
                    ..Cpu::default()
                };
            }

            match data.cpu.run() {
                _ if empty => {
                    println!("YES");
                    data.output = data.cpu.registers_str();
                }
                Ok(()) => {
                    data.output = data.cpu.registers_str();
                }
                Err(_) => {}
            }

            Handled::Yes
        } else {
            Handled::No
        }
    }    
}
