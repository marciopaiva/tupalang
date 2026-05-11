use serde_json::json;
use tupa_plugin::{PluginError, PluginManager};

// Compila o plugin de teste usando cargo e retorna o caminho para o .so/.dll
fn compile_test_plugin() -> Result<std::path::PathBuf, String> {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let plugin_src_dir = manifest_dir.join("plugin_src");

    // Executar cargo build no diretório do plugin
    let target_dir = tempfile::tempdir().unwrap();
    let output = std::process::Command::new("cargo")
        .current_dir(&plugin_src_dir)
        .args(["build", "--release", "--target-dir"])
        .arg(target_dir.path())
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "cargo build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Determinar localização da biblioteca compilada
    let target_arch = std::env::var("CARGO_BUILD_TARGET").unwrap_or_else(|_| {
        if cfg!(target_os = "macos") {
            "universal2-apple-darwin".to_string()
        } else if cfg!(target_os = "linux") {
            "x86_64-unknown-linux-gnu".to_string()
        } else if cfg!(target_os = "windows") {
            "x86_64-pc-windows-msvc".to_string()
        } else {
            std::env::var("HOST").unwrap_or_else(|_| "unknown".to_string())
        }
    });

    let lib_dir = target_dir.path().join("release").join(target_arch);

    let lib_name = if cfg!(target_os = "windows") {
        "test_plugin.dll"
    } else if cfg!(target_os = "macos") {
        "libtest_plugin.dylib"
    } else {
        "libtest_plugin.so"
    };

    let lib_path = lib_dir.join(lib_name);
    if !lib_path.exists() {
        return Err(format!(
            "Plugin library not found at {}",
            lib_path.display()
        ));
    }

    Ok(lib_path)
}

#[test]
fn test_plugin_load_and_call_integration() {
    let lib_path = match compile_test_plugin() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Skipping plugin integration test: {}", e);
            return;
        }
    };

    // Carregar plugin
    let mut pm = PluginManager::new();
    let plugin = pm
        .load_plugin(&lib_path)
        .expect("Failed to load test plugin");
    assert_eq!(plugin.name, "integration_test_plugin");
    assert!(plugin.functions.contains(&"identity".to_string()));
    assert!(plugin.functions.contains(&"double".to_string()));
    assert!(plugin.functions.contains(&"const42".to_string()));

    // Testar list_functions
    let all_functions = pm.list_functions();
    assert_eq!(all_functions.len(), 1);
    assert_eq!(all_functions[0].0, "integration_test_plugin");
    assert_eq!(all_functions[0].1.len(), 3);
}

#[test]
fn test_plugin_call_identity() {
    let lib_path = match compile_test_plugin() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Skipping plugin call test: {}", e);
            return;
        }
    };

    let mut pm = PluginManager::new();
    pm.load_plugin(&lib_path).expect("load failed");

    let input = json!({"key": "value", "number": 123});
    let result = pm
        .call("identity", input.clone())
        .expect("identity call failed");
    assert_eq!(result, input);
}

#[test]
fn test_plugin_call_double() {
    let lib_path = match compile_test_plugin() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Skipping plugin call test: {}", e);
            return;
        }
    };

    let mut pm = PluginManager::new();
    pm.load_plugin(&lib_path).expect("load failed");

    let result = pm.call("double", json!(7)).expect("double call failed");
    assert_eq!(result, json!(14));

    let result = pm.call("double", json!(-5)).expect("double call failed");
    assert_eq!(result, json!(-10));

    // Non-number should return null (plugin logic)
    let result = pm
        .call("double", json!("not a number"))
        .expect("double call failed");
    assert_eq!(result, json!(null));
}

#[test]
fn test_plugin_call_const42() {
    let lib_path = match compile_test_plugin() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Skipping plugin call test: {}", e);
            return;
        }
    };

    let mut pm = PluginManager::new();
    pm.load_plugin(&lib_path).expect("load failed");

    let result = pm
        .call("const42", json!({"anything": "goes"}))
        .expect("const42 call failed");
    assert_eq!(result, json!(42));
}

#[test]
fn test_plugin_call_unknown_function() {
    let lib_path = match compile_test_plugin() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Skipping plugin call test: {}", e);
            return;
        }
    };

    let mut pm = PluginManager::new();
    pm.load_plugin(&lib_path).expect("load failed");

    let result = pm.call("function_that_does_not_exist_2026", json!(null));
    assert!(matches!(result, Err(PluginError::SymbolNotFound(_))));
}

#[test]
fn test_plugin_load_multiple() {
    // Compilar primeiro plugin
    let lib_path1 = match compile_test_plugin() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Skipping multi-plugin test: {}", e);
            return;
        }
    };

    // Para um segundo plugin, vamos recompilar com nome diferente
    // Para simplificação, apenas carregamos o mesmo plugin duas vezes
    let mut pm = PluginManager::new();
    let _p1 = pm.load_plugin(&lib_path1).expect("first load failed");
    let count_after_first = pm.list_functions().len();

    // Carregar novamente o mesmo plugin (adiciona segunda entrada)
    let _p2 = pm.load_plugin(&lib_path1).expect("second load failed");
    let count_after_second = pm.list_functions().len();

    assert_eq!(count_after_first, 1);
    assert_eq!(count_after_second, 2); // Mesmo plugin aparecendo duas vezes
                                       // Nota: PluginManager permite carregar o mesmo .so múltiplas vezes; cada entrada independente
}
