trait Licensed {
    // TODO: Add a default implementation for `licensing_info` so that
    // implementors like the two structs below can share that default behavior
    // without repeating the function.
    // The default license information should be the string "Default license".
    fn licensing_info(&self) -> String{
        String::from("Default license")
    }
    fn return_damn(&self) -> String{
        String::from("Damn")
    }
    fn return_damned(&self) -> String;
    
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {
    fn return_damned(&self) -> String{
        String::from("Damned")
    }
} // Don't edit this line.
impl Licensed for OtherSoftware {
    fn return_damned(&self) -> String{
        String::from("Damned")
    }
} // Don't edit this line.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
    #[test]
    fn is_damn(){
        let damn_info = "Damn";
        let some_software = SomeSoftware { version_number: 1};
        assert_eq!(some_software.return_damn(), damn_info);
    }
}
