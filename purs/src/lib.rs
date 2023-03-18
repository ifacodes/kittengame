mod gen;
mod info;
use anyhow::Result;
pub use info::*;

pub fn parse() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::query_types;

    use super::query_attachments;

    #[test]
    fn attachment() {
        let module =
            naga::front::wgsl::parse_str(include_str!("../../internal/main.wgsl")).unwrap();
        query_types(&module);
        println!("{module:#?}");
        assert_eq!(1, query_attachments(&module).unwrap())
    }
}
