const { execSync } = require('child_process');
const fs = require('fs');

const cargoToml = fs.readFileSync('Cargo.toml', 'utf8');
const match = cargoToml.match(/members\s*=\s*\[([\s\S]*?)\]/);
if (match) {
    let members = match[1].split('\n')
        .map(l => l.trim().replace(/"/g, '').replace(/,/g, ''))
        .filter(l => l.length > 0 && !l.startsWith('#'));
    
    let madeProgress = true;
    while (madeProgress && members.length > 0) {
        madeProgress = false;
        const nextMembers = [];
        for (const member of members) {
            console.log(`Attempting to publish ${member}...`);
            try {
                // Read package name from its Cargo.toml
                const memberCargo = fs.readFileSync(member + '/Cargo.toml', 'utf8');
                let packageName = member.split('/').pop();
                const nameMatch = memberCargo.match(/name\s*=\s*"([^"]+)"/);
                if (nameMatch) {
                    packageName = nameMatch[1];
                }
                // Skip if publish = false
                if (memberCargo.includes('publish = false')) {
                    console.log(`Skipping ${member} (publish = false)`);
                    continue;
                }
                
                const out = execSync(`cargo publish -p ${packageName}`, { stdio: 'pipe', encoding: 'utf8' });
                console.log(out);
                console.log(`Successfully published ${member}`);
                madeProgress = true;
                // Wait 15s for index propagation
                execSync('powershell -Command "Start-Sleep -Seconds 15"');
            } catch (e) {
                const out = (e.stdout || '') + '\n' + (e.stderr || '') + '\n' + e.message;
                if (out.includes('already exists') || out.includes('is already uploaded')) {
                    console.log(`Already published: ${member}`);
                    madeProgress = true; // Still counts as progressing since it's done
                } else {
                    console.log(`Failed to publish (maybe dependency missing?): ${member}`);
                    nextMembers.push(member);
                }
            }
        }
        members = nextMembers;
        console.log(`Pass complete. ${members.length} remaining.`);
    }
}
