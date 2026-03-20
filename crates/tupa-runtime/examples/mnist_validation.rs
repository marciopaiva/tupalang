use serde_json::{json, Value};
use tupa_codegen::execution_plan::{ConstraintPlan, ExecutionPlan, StepPlan, TypeSchema};
use tupa_runtime::{evaluate_constraints, register_step, run_pipeline_async};

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("MNIST Validation Example (Simulated)");

    // 1. Load Image (Simulated as JSON Array)
    register_step("mnist::load_image", |_input: Value| {
        println!("Loading MNIST image...");
        // Create a 28x28 array (simulated as nested vectors)
        let row = vec![0.0; 28];
        let image = vec![row; 28]; // 28x28

        Ok(json!({
            "image": image,
            "label": 5
        }))
    });

    // 2. Validate Shape (Manually)
    register_step("mnist::validate_shape", |input: Value| {
        println!("Validating tensor shape...");
        // Access image from previous step
        let image = input
            .get("mnist::load_image")
            .and_then(|data| data.get("image"))
            .ok_or("Missing image")?;

        let rows = image.as_array().ok_or("Image is not an array")?;
        if rows.len() != 28 {
            return Err(format!("Invalid rows: {}, expected 28", rows.len()));
        }

        for (i, row) in rows.iter().enumerate() {
            let cols = row.as_array().ok_or(format!("Row {} is not an array", i))?;
            if cols.len() != 28 {
                return Err(format!(
                    "Invalid cols at row {}: {}, expected 28",
                    i,
                    cols.len()
                ));
            }
        }

        println!("Shape [28, 28] validated.");
        Ok(json!({
            "valid": 1.0,
            "shape": [28, 28]
        }))
    });

    // Create Execution Plan
    let plan = ExecutionPlan {
        name: "mnist_validation_pipeline".to_string(),
        version: "1.0.0".to_string(),
        seed: None,
        input_schema: TypeSchema {
            kind: "any".to_string(),
            elem: None,
            fields: None,
            len: None,
            name: None,
            tensor_shape: None,
            tensor_dtype: None,
        },
        output_schema: None,
        steps: vec![
            StepPlan {
                name: "mnist::load_image".to_string(),
                function_ref: "mnist::load_image".to_string(),
                effects: vec![],
            },
            StepPlan {
                name: "mnist::validate_shape".to_string(),
                function_ref: "mnist::validate_shape".to_string(),
                effects: vec![],
            },
        ],
        constraints: vec![
            ConstraintPlan {
                metric: "mnist::validate_shape.valid".to_string(),
                comparator: "eq".to_string(),
                threshold: 1.0,
            }, // true -> 1.0
        ],
        metrics: std::collections::HashMap::new(),
        metric_plans: vec![],
    };

    println!("Running pipeline...");
    let result = run_pipeline_async(&plan, json!({})).await;

    match result {
        Ok(output) => {
            println!("Pipeline Output: {}", output);
            let report = evaluate_constraints(&plan, &output);
            println!("Validation result: {}", report);
        }
        Err(e) => {
            println!("Pipeline Failed: {:?}", e);
            return Err(e.to_string());
        }
    }

    Ok(())
}
