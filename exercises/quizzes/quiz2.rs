// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {


    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        
        let mut return_vector: Vec<String> = Vec::new();
        for (s, command) in input{
            match command{
                Command::Uppercase => {
                    return_vector.push(s.to_uppercase());
                }
                Command::Append(bar_multiplier) => {
                    let mut s_temp = s.clone();
                    for _ in 0..bar_multiplier{
                        // println!("fuck off"); //trust me for debugging purposes
                        s_temp.push('b');
                        s_temp.push('a');
                        s_temp.push('r');
                        
                    }
                    //alternative
                    // s_temp = s_temp+&"bar".repeat(bar_multiplier);
                    return_vector.push(s_temp);
                }
                Command::Trim => {
                    return_vector.push(s.trim().to_string());
                }
            }
        }
        
        return_vector
        
    }
    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
