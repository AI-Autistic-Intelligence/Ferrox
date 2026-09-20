const fs = require('fs');
const path = require('path');

const rules = [
    { title: "Philosophy / Purpose", content: "*Document the philosophy and core purpose of this component here.*" },
    { title: "Architectural Layering", content: "*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* " },
    { title: "How it Works (Under the hood)", content: "*Detail the internal mechanics, memory model, and execution flow.*" },
    { title: "Why it was designed this way", content: "*Explain the historical context, trade-offs, and design rationale.*" },
    { title: "Usage Guide & Code Examples", content: "*Provide integration examples, setup guides, and typical use cases.*" },
    { title: "Anti-Patterns", content: "*List common mistakes, misconfigurations, and patterns to avoid when using this component.*" },
    { title: "Pro-Tips / Best Practices", content: "*Provide advanced tips, performance optimizations, and recommended patterns.*" }
];

function processDirectory(dir) {
    const entries = fs.readdirSync(dir, { withFileTypes: true });
    for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);
        if (entry.isDirectory()) {
            processDirectory(fullPath);
        } else if (entry.isFile() && fullPath.endsWith('.md')) {
            let content = fs.readFileSync(fullPath, 'utf8');
            let updated = false;
            
            // Check for existing rules or equivalent titles heuristically
            const hasSection = (titleKeywords) => {
                const regex = new RegExp(`^#+\\s+.*(${titleKeywords.join('|')}).*`, 'im');
                return regex.test(content);
            };

            const checks = [
                hasSection(['Philosophy', 'Purpose', 'Overview']),
                hasSection(['Architectural', 'Architecture', 'Layering', 'Layer']),
                hasSection(['How it Works', 'Under the hood', 'Internal', 'Mechanics']),
                hasSection(['Why', 'Rationale', 'Design']),
                hasSection(['Usage', 'Example', 'Guide']),
                hasSection(['Anti-Patterns', 'Avoid', 'Anti Pattern']),
                hasSection(['Best Practices', 'Pro-Tips', 'Tips'])
            ];

            let appendContent = "\\n\\n---";
            checks.forEach((hasIt, index) => {
                if (!hasIt) {
                    appendContent += `\\n\\n## ${index + 1}. ${rules[index].title}\\n\\n${rules[index].content}`;
                    updated = true;
                }
            });

            if (updated) {
                fs.writeFileSync(fullPath, content + appendContent);
                console.log(`Updated: ${fullPath}`);
            }
        }
    }
}

const docsDir = path.join('C:', 'Users', 'nn', 'Desktop', 'code', 'ferrox', 'docs', 'docs');
processDirectory(docsDir);
console.log('Documentation augmentation complete.');
