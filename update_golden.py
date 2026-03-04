import re
import os
import subprocess
import sys

def get_workspace_root():
    # Assuming the script is run from the project root
    return os.getcwd()

def normalize_output(text, root):
    normalized = text.replace(root, "")
    normalized = normalized.replace("/examples/", "examples/")
    if normalized.startswith('/'):
        normalized = normalized.lstrip('/')
    return normalized

def run_cli(args, root):
    # Use the binary directly to avoid cargo output
    exe = os.path.join(root, "target", "debug", "tupa-cli")
    cmd = [exe] + args
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"Error running command: {cmd}")
        return ""

def run_cli_err(args, root):
    exe = os.path.join(root, "target", "debug", "tupa-cli")
    cmd = [exe] + args
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
        return result.stderr
    except subprocess.CalledProcessError as e:
        print(f"Error running command: {cmd}")
        return ""

def main():
    root = get_workspace_root()
    
    # Build first
    print("Building tupa-cli...")
    subprocess.run(["cargo", "build", "--package", "tupa-cli", "--bin", "tupa-cli"], cwd=root, check=True)

    cli_golden_path = os.path.join(root, "crates", "tupa-cli", "tests", "cli_golden.rs")
    
    with open(cli_golden_path, "r") as f:
        content = f.read()
    
    # Regex to match test functions
    # Matches: fn golden_test_name() { ... run_cli/run_cli_err(args) ... expected_path("filename") ... }
    # This is a bit complex to regex reliably due to multiline.
    # Instead, let's look for blocks.
    
    # Simpler approach: find calls to run_cli/run_cli_err and the associated expected file.
    # We can iterate through the file line by line or use a state machine.
    
    test_pattern = re.compile(r"fn\s+(golden_\w+)\s*\(\)\s*\{")
    run_cli_pattern = re.compile(r"run_cli\s*\(&\[(.*?)\]\)", re.DOTALL)
    run_cli_err_pattern = re.compile(r"run_cli_err\s*\(&\[(.*?)\]\)", re.DOTALL)
    expected_path_pattern = re.compile(r'expected_path\s*\(\s*"(.*?)"\s*\)')
    example_path_pattern = re.compile(r'example_path\s*\(\s*"(.*?)"\s*\)')

    # We will use a simpler extraction logic since we have full file content
    # Split by "fn golden_"
    
    tests = content.split("fn golden_")
    
    for i in range(1, len(tests)):
        test_block = "golden_" + tests[i].split("assert_eq!")[0]
        test_name = test_block.split("(")[0].strip()
        
        print(f"Processing {test_name}...")
        
        is_error = "run_cli_err" in test_block
        
        # Extract args
        args_match = run_cli_err_pattern.search(test_block) if is_error else run_cli_pattern.search(test_block)
        if not args_match:
            print(f"  Skipping {test_name}: could not find run_cli call")
            continue
            
        args_str = args_match.group(1)
        # Parse args from string like: "check", example_path("file.tp").to_str().unwrap(), "--format", "json"
        # We need to reconstruct the args list.
        
        args = []
        parts = args_str.split(',')
        for part in parts:
            part = part.strip()
            if not part: continue
            
            if part.startswith('"') and part.endswith('"'):
                args.append(part.strip('"'))
            elif "example_path" in part:
                # Extract filename from example_path("filename")
                m = example_path_pattern.search(part)
                if m:
                    filename = m.group(1)
                    # Resolve full path
                    full_path = os.path.join(root, "examples", filename)
                    args.append(full_path)
        
        # Extract expected file
        expected_match = expected_path_pattern.search(test_block)
        if not expected_match:
            print(f"  Skipping {test_name}: could not find expected_path")
            continue
            
        expected_filename = expected_match.group(1)
        expected_file_path = os.path.join(root, "examples", "expected", expected_filename)
        
        # Run command
        output = run_cli_err(args, root) if is_error else run_cli(args, root)
        normalized = normalize_output(output, root)
        
        # Write to file
        print(f"  Updating {expected_filename}...")
        with open(expected_file_path, "w") as f:
            f.write(normalized)

if __name__ == "__main__":
    main()
