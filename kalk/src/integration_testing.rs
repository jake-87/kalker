#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read, path::PathBuf};

    use crate::kalk_num::KalkNum;

    fn eval_file(name: &str) -> KalkNum {
        let mut path = PathBuf::new();
        path.push(env!("CARGO_MANIFEST_DIR"));
        path.push("..");
        path.push("tests");
        path.push(name);
        path.set_extension("kalker");

        let mut file_content = String::new();
        File::open(path)
            .unwrap()
            .read_to_string(&mut file_content)
            .unwrap();
        let mut context = crate::parser::Context::new();

        crate::parser::eval(&mut context, &file_content, 58)
            .unwrap()
            .unwrap()
    }

    #[test]
    fn test_basics() {
        let result = eval_file("basics");
        assert_eq!(result.to_string_real(10), "3400");
        assert!(!result.has_imaginary());
    }

    #[test]
    fn test_radix() {
        let result = eval_file("radix");
        assert_eq!(result.to_string_real(10), "62");
        assert_eq!(result.to_string_imaginary(10, false), "11.3125");
    }
}
