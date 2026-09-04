const fs = require('fs');
const path = require('path');

const OLD_NAME = 'yalc';
const NEW_NAME = 'ferrox';
const IGNORE_DIRS = ['.git', 'target', 'node_modules'];

function renameContent(dir) {
    const files = fs.readdirSync(dir);
    for (const file of files) {
        if (IGNORE_DIRS.includes(file) || file === 'rename.js' || file === 'rename.py') continue;
        
        const fullPath = path.join(dir, file);
        const stat = fs.statSync(fullPath);
        
        if (stat.isDirectory()) {
            renameContent(fullPath);
        } else {
            try {
                let content = fs.readFileSync(fullPath, 'utf8');
                let changed = false;
                
                // Replace variations
                const variations = [
                    [OLD_NAME, NEW_NAME],
                    [OLD_NAME.toUpperCase(), NEW_NAME.toUpperCase()],
                    [OLD_NAME.charAt(0).toUpperCase() + OLD_NAME.slice(1), NEW_NAME.charAt(0).toUpperCase() + NEW_NAME.slice(1)]
                ];
                
                for (const [o, n] of variations) {
                    if (content.includes(o)) {
                        content = content.split(o).join(n);
                        changed = true;
                    }
                }
                
                if (changed) {
                    fs.writeFileSync(fullPath, content, 'utf8');
                    console.log(`Updated content: ${fullPath}`);
                }
            } catch (e) {
                // Ignore binary files or read errors
            }
        }
    }
}

function renameFilesAndDirs(dir) {
    const items = fs.readdirSync(dir);
    
    // Process subdirectories first (bottom-up)
    for (const item of items) {
        if (IGNORE_DIRS.includes(item)) continue;
        const fullPath = path.join(dir, item);
        if (fs.statSync(fullPath).isDirectory()) {
            renameFilesAndDirs(fullPath);
        }
    }
    
    // Now rename items in current dir
    for (const item of items) {
        if (IGNORE_DIRS.includes(item)) continue;
        
        if (item.toLowerCase().includes(OLD_NAME)) {
            const oldPath = path.join(dir, item);
            
            // Replace maintaining case where possible (simplified for just the exact match)
            const newItem = item.split(OLD_NAME).join(NEW_NAME)
                                .split(OLD_NAME.toUpperCase()).join(NEW_NAME.toUpperCase())
                                .split(OLD_NAME.charAt(0).toUpperCase() + OLD_NAME.slice(1)).join(NEW_NAME.charAt(0).toUpperCase() + NEW_NAME.slice(1));
            
            const newPath = path.join(dir, newItem);
            console.log(`Renaming: ${oldPath} -> ${newPath}`);
            fs.renameSync(oldPath, newPath);
        }
    }
}

console.log(`Starting rename from ${OLD_NAME} to ${NEW_NAME}...`);
renameContent('.');
renameFilesAndDirs('.');
console.log('Done!');
