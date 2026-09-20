const fs = require('fs');
const path = require('path');

const srcDir = path.join('C:', 'Users', 'nn', 'Desktop', 'code', 'ferrox', 'docs', 'build');
const destDir = path.join('C:', 'Users', 'nn', 'Desktop', 'code', 'ferrox-docs');

function copyRecursiveSync(src, dest) {
    const exists = fs.existsSync(src);
    const stats = exists && fs.statSync(src);
    const isDirectory = exists && stats.isDirectory();
    
    if (isDirectory) {
        if (!fs.existsSync(dest)) {
            fs.mkdirSync(dest);
        }
        fs.readdirSync(src).forEach(function(childItemName) {
            copyRecursiveSync(path.join(src, childItemName), path.join(dest, childItemName));
        });
    } else {
        fs.copyFileSync(src, dest);
    }
}

if (fs.existsSync(srcDir)) {
    console.log(`Copying files from ${srcDir} to ${destDir}`);
    // We should be careful to only copy into the root, overwriting matching files.
    fs.readdirSync(srcDir).forEach(item => {
        copyRecursiveSync(path.join(srcDir, item), path.join(destDir, item));
    });
    console.log('Done!');
} else {
    console.log('Source directory does not exist yet.');
}
