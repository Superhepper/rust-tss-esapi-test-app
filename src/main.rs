use tss_esapi::Context;
use tss_esapi::TctiNameConf;

fn main() -> tss_esapi::Result<()> {
    let mut context = Context::new(TctiNameConf::Tbs)?;

    println!(
        "Hello, world: {:?}",
        context.get_tpm_property(tss_esapi::constants::PropertyTag::DayOfYear)?
    );
    // also get_random
    Ok(())
}
