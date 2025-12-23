import re
import json
import os

def parse_emulated_devices(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Find the start of the emulatedDevices array
    start_marker = "const emulatedDevices = ["
    start_idx = content.find(start_marker)
    if start_idx == -1:
        print("Could not find start of emulatedDevices array")
        return

    # Extract the array content roughly
    # We'll just grab everything from the start marker to the end of the file or a reasonably safe point
    # Since it's TypeScript/JS object literals, we need to massage it into JSON.
    
    # Simple regex approach to find objects inside the array:
    # { ... }
    # This is fragile but might work if the formatting is consistent.
    
    # Better approach: The file format is very regular.
    # It looks like:
    # {
    #   'order': 10,
    #   'show-by-default': true,
    #   'title': 'iPhone SE',
    #   ...
    # },
    
    # We can try to use standard python eval/exec if we sanitize the input slightly,
    # by replacing single quotes with double quotes and handling comments.
    
    # Let's extract the list block
    list_content = content[start_idx + len(start_marker):]
    
    # Truncate at the end of the array definition "];"
    end_marker = "];"
    end_idx = list_content.find(end_marker)
    if end_idx != -1:
        list_content = list_content[:end_idx + 1] # include the closing bracket

    # Remove JS comments
    list_content = re.sub(r'//.*', '', list_content)
    
    # Replace single quotes with double quotes for keys/strings
    # This is risky for nested quotes, but let's try a regex for keys first
    # Key regex: 'key': -> "key":
    list_content = re.sub(r"'([^']+)':", r'"\1":', list_content)
    
    # Value strings regex: 'value' -> "value", avoiding already double-quoted
    # This matches 'string'
    list_content = re.sub(r":\s*'([^']*)'", r': "\1"', list_content)
    
    # Handle array strings: ['a', 'b'] -> ["a", "b"]
    # This is getting complicated. Let's try a simpler approach since we see how embed_mobile_devices_in_cpp.py did it.
    # It used ast.literal_eval but on a localized version.
    
    # Let's verify if we can just use ast.literal_eval on the raw content after stripping comments.
    
    import ast
    
    # Function to extract objects
    devices = []
    
    # Split by "}," to separate items roughly
    items = list_content.split('},')
    
    clean_items = []
    current_item = ""
    
    for line in list_content.split('\n'):
        line = line.strip()
        if not line: continue
        if line.startswith('//'): continue
        
        # Manually parsing is tough.
        # Let's go back to the python script "embed_mobile_devices_in_cpp.py" approach
        # It relies on specific markers "DEVICE-LIST-BEGIN".
        
    start_marker = "// DEVICE-LIST-BEGIN"
    start_idx = content.find(start_marker)
    if (start_idx != -1):
        # We can use the logic from the existing script!
        # ... except we are in python, not TS.
        pass
        
    # Let's try to just use regex to extract titles and user agents as a robust fallback
    # The user wants "all devices" and "config".
    
    # Let's use a smarter regex to extract the JSON-like objects.
    
    # Extract entries enclosed in { ... } inside the array.
    # We'll use a stack counter for braces.
    
    objects = []
    open_braces = 0
    current_obj_str = []
    in_object = False
    
    # Start scanning after "const emulatedDevices = ["
    scan_start = content.find("const emulatedDevices = [") + len("const emulatedDevices = [")
    
    chars = content[scan_start:]
    
    # Only process untill the closing ];
    # But wait, we might iterate char by char.
    
    i = 0
    while i < len(chars):
        c = chars[i]
        
        # Check for end of list
        if open_braces == 0 and c == ']' and chars[i+1] == ';':
            break
            
        if c == '{':
            if open_braces == 0:
                in_object = True
            open_braces += 1
        
        if in_object:
            current_obj_str.append(c)
            
        if c == '}':
            open_braces -= 1
            if open_braces == 0:
                in_object = False
                obj_text = "".join(current_obj_str)
                objects.append(obj_text)
                current_obj_str = []
        
        i += 1
        
    final_devices = []
    
    for obj_str in objects:
        try:
            # Cleanup to make it valid JSON or python dict
            # remove comments
            obj_str = re.sub(r'//.*', '', obj_str)
            # 'key': -> "key":
            obj_str = re.sub(r"'([\w-]+)':", r'"\1":', obj_str)
            # : 'value' -> : "value"
            obj_str = re.sub(r":\s*'([^']*)'", r': "\1"', obj_str)
            # : true/false -> : true/false (JSON is lower, Python is Title)
            # We want JSON output so lowercase is good.
            
            # Arrays: ['a', 'b'] -> ["a", "b"]
            obj_str = re.sub(r"\['([^']*)', '([^']*)'\]", r'["\1", "\2"]', obj_str) # 2 items
            obj_str = re.sub(r"\['([^']*)'\]", r'["\1"]', obj_str) # 1 item
            
            # Trailing commas
            obj_str = re.sub(r",\s*}", "}", obj_str)
            obj_str = re.sub(r",\s*]", "]", obj_str)
            
            # Fix keys that might not be quoted if they are simple identifiers? 
            # The file seems to quote most keys e.g. 'title'.
            
            # Handle image urls which have parentheses and might break simple regexes if we aren't careful?
            # They are inside strings, so our regex ": '...'" should catch them IF they don't have escaped quotes.
            
            # EVAL STRATEGY
            # It's safer to convert to Python dict syntax and eval.
            py_str = obj_str.replace("true", "True").replace("false", "False").replace("undefined", "None")
            # Convert "key": to "key":
            # Convert 'key': to "key":
            # Re-convert keys to strings if they are bare?
            
            # Actually, `content` has 'title': '...'.
            # Python accepts 'title': '...'.
            # So just removing comments and handling true/false/undefined might be enough for eval().
            
            clean_py = obj_str.replace("true", "True").replace("false", "False").replace("undefined", "None")
            clean_py = re.sub(r'//.*', '', clean_py)
            
            # The file uses single quotes for keys and values generally. Python likes that.
            device_dict = eval(clean_py)
            final_devices.append(device_dict)
        except Exception as e:
            # print(f"Skipping object due to parse error: {e}\nObj: {obj_str[:50]}...")
            pass

    return final_devices

if __name__ == "__main__":
    devices = parse_emulated_devices("third_party/devtools-frontend/src/front_end/models/emulation/EmulatedDevices.ts")
    if devices:
        print(f"Extracted {len(devices)} devices.")
        with open("chromium_analysis/all_devices.json", "w") as f:
            json.dump(devices, f, indent=2)
    else:
        print("No devices extracted.")
