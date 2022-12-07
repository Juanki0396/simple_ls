pub enum HiddenItems {
    All,
    Files,
    Directories,
    Any,
}

pub enum PrintMode {
    Complete,
    BasicInfo,
    Line,
}

trait FromArg {
    type Output;

    fn from_arg(arg: &String) -> Result<Self::Output, &'static str>;
}

impl FromArg for HiddenItems {
    type Output = HiddenItems;

    fn from_arg(arg: &String) -> Result<Self::Output, &'static str> {
        if !arg.starts_with("-") {
            return Err("Not an argument");
        } else if arg.contains("--All") || arg.contains("A") {
            return Ok(HiddenItems::All);
        } else if arg.contains("--Directories") || arg.contains("d") {
            return Ok(HiddenItems::Directories);
        } else {
            return Ok(HiddenItems::Any);
        }
    }
}

impl FromArg for PrintMode {
    type Output = PrintMode;

    fn from_arg(arg: &String) -> Result<Self::Output, &'static str> {
        if !arg.starts_with("-") {
            return Err("Not an argument");
        } else if arg.contains("--Complete") || arg.contains("-l") {
            return Ok(PrintMode::Complete);
        } else if arg.contains("--BasicInfo") || arg.contains("-b") {
            return Ok(PrintMode::BasicInfo);
        } else {
            return Ok(PrintMode::Line);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_all_hiddens() {
        let short_arg = String::from("-a");
        let long_arg = String::from("--All");

        assert!(match HiddenItems::from_arg(&short_arg).unwrap() {
            HiddenItems::All => true,
            _ => false,
        });

        assert!(match HiddenItems::from_arg(&long_arg).unwrap() {
            HiddenItems::All => true,
            _ => false,
        });
    }
}
