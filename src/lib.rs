extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod tokenizer;

// Import 'window.alert'
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export generate_truth_table
#[wasm_bindgen]
pub fn generate_truth_table(expr: String) -> String {
    let mut input = "P and not Q";
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
}
