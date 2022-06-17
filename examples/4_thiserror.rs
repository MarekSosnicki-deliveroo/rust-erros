use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
enum MyParseError {
    #[error("Input string is too long {0}")]
    InputTooLong(usize),
    #[error("Failed to parse {0}")]
    ParseFail(#[from] ParseIntError),
    #[error("Value is 0")]
    ValueIs0,
}

fn parse_item(value: &str) -> Result<i32, MyParseError> {
    if value.len() > 4 {
        return Err(MyParseError::InputTooLong(value.len()));
    }

    let result = value.parse()?;
    if result == 0 {
        Err(MyParseError::OtherFailure("Cannot parse 0".to_string()))
    } else {
        Ok(result)
    }
}

#[derive(Debug, thiserror::Error)]
enum MyAnalyseError {}
// fn analyse_items(items: &[(bool, i32)]) -> anyhow::Result<()> {
//     for (attr, value) in items {
//         if *attr == (value % 2 == 0) {
//             bail!("Analysis failed for {}, {}", attr, value)
//         }
//     }
//     Ok(())
// }

#[derive(Debug, thiserror::Error)]
enum MainAppError {}

// fn parse_items(items: &[(bool, &str)]) -> anyhow::Result<Vec<(bool, i32)>> {
//     items
//         .iter()
//         .cloned()
//         .map(|(attr, value)| Ok((attr, parse_item(value)?)))
//         .collect::<anyhow::Result<Vec<(bool, i32)>>>()
//         .context("Failed to parse items")
// }
//
//
// fn long_example() -> anyhow::Result<()> {
//     let items = vec![(true, "1"), (true, "2"), (true, "3")];
//     let parsed = parse_items(items.as_slice()).context("Failed to get parsed items")?;
//     analyse_items(&parsed).context("Items analysis failed")
// }
//

fn main() {}
