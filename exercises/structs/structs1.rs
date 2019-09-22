// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String /* TODO: Something goes here */);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            name: "green".to_string(),
            hex: "#00FF00".to_string(),
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // For more fun, use the field initialization shorthand.
        let green = ColorTupleStruct("green".to_string(), "#00FF00".to_string());

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        struct UnitStruct;
        impl ToString for UnitStruct {
            fn to_string(&self) -> String {
                "UnitStruct".to_string()
            }
        }
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct.to_string());

        assert_eq!(message, "\"UnitStruct\"s are fun!");
    }
}
