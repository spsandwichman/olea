//#![allow(dead_code)]

mod ast;
mod ir;
mod ir_builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src = std::fs::read_to_string("example.yafl")?;
    let st: ast::Block = serde_lexpr::from_str(&src)?;
    println!("{st:?}");
    let mut builder = ir_builder::IrBuilder::new();
    builder.build_block(&st);
    let ir = builder.finish();
    println!("{ir:?}");
    Ok(())
}
