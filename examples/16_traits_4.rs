// ============================================================================
// TRAIT DEFINITION: Serializable
// ============================================================================
//
// We define a trait that combines three capabilities:
// 1. Display - for human-readable string formatting (used in println!("{}", x))
// 2. Debug   - for debug formatting (used in println!("{:?}", x))
// 3. Clone   - for creating copies of the value
//
// The `: Display + Debug + Clone` syntax is called "supertraits" - any type
// implementing Serializable MUST also implement these three traits.
//
// This is a common pattern when you want to bundle multiple behaviors together.
// By requiring Display, we can use the formatted string output as a basis for
// serialization in the default to_bytes() implementation.

use std::fmt::{Debug, Display};

trait Serializable: Display + Debug + Clone {
    // This method converts the implementing type to a byte vector.
    //
    // We provide a default implementation that:
    // 1. Uses the Display trait's to_string() method to get a String
    // 2. Converts that String into owned bytes via into_bytes()
    //
    // Types can override this if they need custom byte serialization.
    // For example, Json<T> overrides this to produce JSON-formatted bytes.
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

// ============================================================================
// WRAPPER STRUCT: Json<T>
// ============================================================================
//
// A "newtype" pattern wrapper around any type T.
//
// - `pub T` means the inner value is publicly accessible (data.0)
// - This is a tuple struct with a single unnamed field
// - The generic <T> allows wrapping any type
//
// Why use a wrapper?
// It allows us to implement traits (like Serializable) for types we don't own.
// We can't impl Serializable for Vec<i32> directly (orphan rules), but we CAN
// impl it for our own Json<Vec<i32>> type.

#[derive(Clone)] // Automatically implements Clone if T: Clone
struct Json<T>(pub T);

// ============================================================================
// DISPLAY IMPLEMENTATION FOR Json<T>
// ============================================================================
//
// Required because Serializable has Display as a supertrait.
//
// We constrain T: serde::Serialize so we can convert it to JSON.
// The Display output will be the JSON string representation.
//
// fmt::Result is an alias for Result<(), fmt::Error> - we just need to
// indicate success/failure of the formatting operation.

impl<T: serde::Serialize> Display for Json<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // serde_json::to_string() returns Result<String, Error>
        // We use map_err to convert serde's error to fmt::Error
        // The `?` operator propagates any error, or unwraps the Ok value
        let json_string = serde_json::to_string(&self.0).map_err(|_| std::fmt::Error)?;

        // write! macro writes the formatted string to the formatter
        // This is the standard way to implement Display
        write!(f, "{}", json_string)
    }
}

// ============================================================================
// DEBUG IMPLEMENTATION FOR Json<T>
// ============================================================================
//
// Required because Serializable has Debug as a supertrait.
//
// Debug is typically used for developer-facing output (e.g., {:?} format).
// Here we just delegate to Display for simplicity, but you could provide
// more detailed debug output if needed.

impl<T: serde::Serialize> Debug for Json<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We reuse the Display implementation for Debug output.
        // In practice, Debug might show more details like "Json([1, 2, 3])"
        // but for this example, JSON output is sufficient.
        Display::fmt(self, f)
    }
}

// ============================================================================
// SERIALIZABLE IMPLEMENTATION FOR Json<T>
// ============================================================================
//
// This is where we connect everything together.
//
// Trait bounds explanation:
// - T: serde::Serialize  -> needed to convert T to JSON bytes
// - T: Clone             -> needed because Serializable requires Clone,
//                           and Json<T> derives Clone which requires T: Clone
//
// By implementing Serializable, Json<T> now has the to_bytes() method
// which we override to provide JSON-specific serialization.

impl<T: serde::Serialize + Clone> Serializable for Json<T> {
    fn to_bytes(&self) -> Vec<u8> {
        // serde_json::to_vec() serializes directly to Vec<u8>
        // This is more efficient than to_string().into_bytes()
        // because it avoids creating an intermediate String.
        //
        // unwrap_or_default() handles errors by returning an empty Vec
        // In production code, you might want proper error handling instead.
        serde_json::to_vec(&self.0).unwrap_or_default()
    }
}

// ============================================================================
// MAIN FUNCTION - DEMONSTRATION
// ============================================================================

fn main() {
    // Create a Json wrapper around a Vec<i32>
    // Type inference figures out Json<Vec<i32>> from the argument
    let data = Json(vec![1, 2, 3]);

    // Call to_bytes() which uses our Serializable implementation
    // Returns JSON-formatted bytes: [1,2,3]
    let bytes = data.to_bytes();

    // Uses Display impl - prints the JSON string representation
    // Output: [1,2,3]
    println!("{data}");

    // Bonus: show the actual bytes and convert back to string to verify
    println!("Bytes: {:?}", bytes);
    println!("As string: {}", String::from_utf8_lossy(&bytes));

    // Demonstrate Debug output (uses {:?} formatter)
    println!("Debug: {:?}", data);

    // Demonstrate Clone (required by Serializable supertrait)
    let cloned = data.clone();
    println!("Cloned: {}", cloned);
}
