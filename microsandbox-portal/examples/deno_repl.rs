//! Deno REPL example for microsandbox-portal.
//!
//! This example demonstrates how to use the microsandbox-portal to evaluate
//! TypeScript/JavaScript code in a Deno REPL environment. It shows basic
//! usage including:
//!
//! - Starting the Deno engine
//! - Evaluating TypeScript code with type annotations
//! - Working with async/await
//! - Using Deno standard library modules
//! - Handling different types of output (stdout/stderr)
//!
//! # Prerequisites
//!
//! To run this example, you need:
//! - Deno installed and available in PATH
//! - The `deno` feature enabled when building microsandbox-portal
//!
//! # Running the Example
//!
//! ```bash
//! # Build with Deno feature enabled
//! cargo build --features deno
//!
//! # Run the example
//! cargo run --example deno_repl --features deno
//! ```
//!
//! # Example Output
//!
//! The example will output results from Deno REPL execution, prefixed
//! with the output stream (Stdout/Stderr). For instance:
//!
//! ```text
//! ✅ Engines started successfully
//!
//! 🦕 Running Deno TypeScript example in REPL:
//! [Stdout] Hello from Deno TypeScript!
//! [Stdout] Random number: 42
//! [Stdout] Today is: 2025-05-31
//! ...
//! ```

use microsandbox_portal::portal::repl::start_engines;
#[cfg(feature = "deno")]
use microsandbox_portal::portal::repl::Language;
use std::error::Error;

//--------------------------------------------------------------------------------------------------
// Functions: Main
//--------------------------------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start the engines - this initializes all enabled engines
    let _engine_handle = start_engines().await?;
    println!("✅ Engines started successfully");

    // Example 1: Execute TypeScript code in Deno REPL
    #[cfg(feature = "deno")]
    {
        println!("\n🦕 Running Deno TypeScript example in REPL:");
        let deno_code = r#"
// TypeScript with type annotations
interface Person {
    name: string;
    age: number;
    city: string;
}

const person: Person = {
    name: "Alice",
    age: 30,
    city: "San Francisco"
};

console.log("Hello from Deno TypeScript!");
console.log(`Person: ${person.name}, Age: ${person.age}, City: ${person.city}`);

// Working with async/await
async function fetchData(): Promise<string> {
    return new Promise((resolve) => {
        setTimeout(() => resolve("Async data fetched!"), 100);
    });
}

const data = await fetchData();
console.log(data);

// Using Deno standard library (colors)
import { blue, green, red } from "https://deno.land/std@0.224.0/fmt/colors.ts";

console.log(green("This text is green!"));
console.log(blue("This text is blue!"));
console.log(red("This text is red!"));

// Working with dates
const now = new Date();
console.log(`Current time: ${now.toISOString()}`);

// Array manipulation with TypeScript
const numbers: number[] = [1, 2, 3, 4, 5];
const doubled = numbers.map((n: number) => n * 2);
console.log(`Original: [${numbers.join(", ")}]`);
console.log(`Doubled: [${doubled.join(", ")}]`);

// Object destructuring
const { name, age } = person;
console.log(`Extracted: ${name} is ${age} years old`);
        "#;

        let result = _engine_handle
            .eval(deno_code, Language::Deno, "deno_example", None)
            .await?;

        print_output("Deno TypeScript", &result);
    }

    #[cfg(not(feature = "deno"))]
    {
        println!("❌ Deno feature is not enabled");
        println!("To run this example, build with: cargo run --example deno_repl --features deno");
    }

    // Shutdown the engines
    _engine_handle.shutdown().await?;
    println!("\n✅ Engines shutdown successfully");

    Ok(())
}

//--------------------------------------------------------------------------------------------------
// Helper Functions
//--------------------------------------------------------------------------------------------------

/// Print output from REPL evaluation
#[cfg(feature = "deno")]
fn print_output(title: &str, lines: &[microsandbox_portal::portal::repl::Line]) {
    use microsandbox_portal::portal::repl::Stream;

    if lines.is_empty() {
        println!("  No output from {} evaluation", title);
        return;
    }

    for line in lines {
        let stream_indicator = match line.stream {
            Stream::Stdout => "[Stdout]",
            Stream::Stderr => "[Stderr]",
        };
        println!("  {} {}", stream_indicator, line.text);
    }
    println!();
}
