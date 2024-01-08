use anyhow::Result;
use tracing_subscriber::{filter::LevelFilter, EnvFilter};
use wasmtime_wasi::preview2::WasiView;

mod runner {

    use crate::A;

    use self::game::plugin::version::V;

    wasmtime::component::bindgen!();
    impl RunnerImports for A {
        fn log(&mut self, msg: String) -> wasmtime::Result<()> {
            tracing::info!(msg);
            Ok(())
        }
    }

    impl game::plugin::version::Host for A {
        fn get_version(&mut self) -> wasmtime::Result<V> {
            Ok(V {
                major: 1,
                minor: 0,
                release: 0,
            })
        }
    }
}

struct State;

pub struct A {
    wasi_table: wasmtime_wasi::preview2::Table,
    wasi_ctx: wasmtime_wasi::preview2::WasiCtx,
    _inner: State,
}

impl WasiView for A {
    fn table(&self) -> &wasmtime_wasi::preview2::Table {
        &self.wasi_table
    }

    fn table_mut(&mut self) -> &mut wasmtime_wasi::preview2::Table {
        &mut self.wasi_table
    }

    fn ctx(&self) -> &wasmtime_wasi::preview2::WasiCtx {
        &self.wasi_ctx
    }

    fn ctx_mut(&mut self) -> &mut wasmtime_wasi::preview2::WasiCtx {
        &mut self.wasi_ctx
    }
}

fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let mut config = wasmtime::Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    let engine = wasmtime::Engine::new(&config)?;
    let mut wasi_table = wasmtime_wasi::preview2::Table::new();
    let wasi_ctx = wasmtime_wasi::preview2::WasiCtxBuilder::new()
        .inherit_stdio()
        .build(&mut wasi_table)?;

    let mut store = wasmtime::Store::new(
        &engine,
        A {
            wasi_table,
            wasi_ctx,
            _inner: State,
        },
    );

    let module = wasmtime::component::Component::from_file(&engine, "dist/wasm-plugin.striped.wasm")?;

    let mut linker = wasmtime::component::Linker::new(&engine);
    wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;
    runner::Runner::add_to_linker(&mut linker, |a| a)?;

    let instance = linker.instantiate(&mut store, &module)?;

    instance
        .get_typed_func::<(), ()>(&mut store, "run")?
        .call(&mut store, ())?;
    Ok(())
}
