use crate::parser::json::json_value;

mod parser;
fn main() {
    println!(
        "{:?}",
        json_value(
            "    {
            \"description\": \"the description of the test case\",
            \"schema\": {\"the schema that should\" : \"be validated against\"},
            \"tests\": [
                {
                    \"description\": \"a specific test of a valid instance\",
                    \"data\": \"the instance\",
                    \"valid\": true
                },
                {
                    \"description\": \"another specific test this time, invalid\",
                    \"data\": -15.3E2,
                    \"valid\": false
                }
            ]
        }"
        )
    );
}
