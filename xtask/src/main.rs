// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod bootstrap;
mod bootstrap_groth16;
mod bootstrap_poseidon;
mod bootstrap_protos;
mod gen_receipt;
mod install;

use clap::{Parser, Subcommand};
use tracing_subscriber::{prelude::*, EnvFilter};

use self::{
    bootstrap::Bootstrap, bootstrap_groth16::BootstrapGroth16,
    bootstrap_poseidon::BootstrapPoseidon, bootstrap_protos::BootstrapProtos,
    gen_receipt::GenReceipt, install::Install,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Bootstrap(Bootstrap),
    BootstrapGroth16(BootstrapGroth16),
    BootstrapPoseidon(BootstrapPoseidon),
    BootstrapProtos(BootstrapProtos),
    GenReceipt(GenReceipt),
    Install(Install),
}

impl Commands {
    fn run(&self) {
        match self {
            Commands::Bootstrap(cmd) => cmd.run(),
            Commands::BootstrapGroth16(cmd) => cmd.run(),
            Commands::BootstrapPoseidon(cmd) => cmd.run(),
            Commands::BootstrapProtos(cmd) => cmd.run(),
            Commands::Install(cmd) => cmd.run(),
            Commands::GenReceipt(cmd) => cmd.run(),
        }
    }
}

fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .init();

    Cli::parse().cmd.run();
}
