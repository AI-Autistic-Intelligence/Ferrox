import os

def rename_all(root_dir, old_str, new_str):
    print(f"Renaming {old_str} to {new_str}...")
    
    # 1. Rename contents in all files
    for dirpath, dirnames, filenames in os.walk(root_dir):
        if '.git' in dirpath or 'target' in dirpath:
            continue
        for file in filenames:
            if file == 'rename.py': continue
            filepath = os.path.join(dirpath, file)
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                if old_str in content or old_str.capitalize() in content or old_str.upper() in content:
                    content = content.replace(old_str, new_str)
                    content = content.replace(old_str.capitalize(), new_str.capitalize())
                    content = content.replace(old_str.upper(), new_str.upper())
                    
                    with open(filepath, 'w', encoding='utf-8') as f:
                        f.write(content)
                    print(f"Updated content in {filepath}")
            except Exception as e:
                pass # Skip binary files

    # 2. Collect renames bottom up
    renames = []
    for dirpath, dirnames, filenames in os.walk(root_dir, topdown=False):
        if '.git' in dirpath or 'target' in dirpath:
            continue
            
        for name in filenames:
            if old_str in name:
                renames.append((os.path.join(dirpath, name), os.path.join(dirpath, name.replace(old_str, new_str))))
                
        for name in dirnames:
            if old_str in name:
                renames.append((os.path.join(dirpath, name), os.path.join(dirpath, name.replace(old_str, new_str))))

    # 3. Apply renames
    for old, new in renames:
        print(f"Renaming {old} -> {new}")
        os.rename(old, new)
        
    print("Done!")

if __name__ == '__main__':
    rename_all('.', 'yalc', 'ferrox')
