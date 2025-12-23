
import os
import re
from pathlib import Path

ROOT_DIR = "chromium_analysis/url_request_life"

def analyze_dependencies():
    includes = {}
    files_seen = set()

    # Regex to match #include "..." or <...>
    include_pattern = re.compile(r'^\s*#include\s+["<](.+?)[">]')

    for root, dirs, files in os.walk(ROOT_DIR):
        for file in files:
            if not file.endswith(('.h', '.cc')):
                continue
            
            file_path = os.path.join(root, file)
            # relative path from the analysis root, e.g. net/url_request/url_request.h
            rel_path = os.path.relpath(file_path, ROOT_DIR)
            files_seen.add(rel_path)
            
            includes[rel_path] = []
            
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    for line in f:
                        match = include_pattern.match(line)
                        if match:
                            included_file = match.group(1)
                            includes[rel_path].append(included_file)
            except Exception as e:
                print(f"Error reading {file_path}: {e}")

    # Generate Report
    print("# Dependency Analysis Report\n")
    
    print("## Import Graph\n")
    for file in sorted(includes.keys()):
        print(f"### `{file}` imports:")
        internal_deps = []
        external_deps = []
        for dep in includes[file]:
            # Check if this dependency is one of the files we extracted
            # This is a heuristic check (exact match or .h variant)
            is_internal = False
            for seen in files_seen:
                if dep == seen:  # exact match
                    is_internal = True
                    break
            
            if is_internal:
                internal_deps.append(dep)
            else:
                external_deps.append(dep)
        
        if internal_deps:
            print("**Internal (Extracted Files):**")
            for dep in internal_deps:
                print(f"- `{dep}`")
        
        if external_deps:
            # Limit external output to avoid noise, but show interesting ones (net/, services/)
            relevant_external = [d for d in external_deps if d.startswith('net/') or d.startswith('services/')]
            if relevant_external:
                print("**External (Chromium net/services):**")
                for dep in relevant_external:
                    print(f"- `{dep}`")
        print("\n")

if __name__ == "__main__":
    analyze_dependencies()
