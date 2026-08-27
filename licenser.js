#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const COMMENT_STYLES = {
    block: ['.js', '.ts', '.jsx', '.tsx', '.css', '.c', '.cpp', '.cs', '.go', '.java', '.php'],
    hash: ['.py', '.sh', '.rb', '.pl', '.yaml', '.yml', '.dockerfile', '.toml'],
    xml: ['.html', '.xml', '.svg',],
    sql: ['.sql']
};

function wrapInComment(text, ext) {
    const lines = text.trim().split('\n');
    
    if (COMMENT_STYLES.block.includes(ext)) {
        return `/**\n${lines.map(l => ` * ${l}`).join('\n')}\n */\n\n`;
    } else if (COMMENT_STYLES.hash.includes(ext)) {
        return `${lines.map(l => `# ${l}`).join('\n')}\n\n`;
    } else if (COMMENT_STYLES.xml.includes(ext)) {
        return `<!--\n${lines.join('\n')}\n-->\n\n`;
    } else if (COMMENT_STYLES.sql.includes(ext)) {
        return `${lines.map(l => `-- ${l}`).join('\n')}\n\n`;
    }
    return `${lines.map(l => `// ${l}`).join('\n')}\n\n`;
}

async function run() {
    const args = process.argv.slice(2);
    
    let configPath = 'license.json';
    let templatePath = 'license-header.txt';
    let targetExts = [];

    for (let i = 0; i < args.length; i++) {
        if (args[i] === '--config') configPath = args[++i];
        else if (args[i] === '--template') templatePath = args[++i];
        else if (args[i].startsWith('.')) targetExts.push(args[i]);
    }

    if (targetExts.length === 0) {
        console.log("Usage: node licenser.js .js .py [--config path] [--template path]");
        process.exit(1);
    }

    if (!fs.existsSync(configPath) || !fs.existsSync(templatePath)) {
        console.error("Error: license.json or license-header.txt not found.");
        process.exit(1);
    }

    const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
    let template = fs.readFileSync(templatePath, 'utf8');

    Object.keys(config).forEach(key => {
        const regex = new RegExp(`\\[${key}\\]`, 'g');
        template = template.replace(regex, config[key]);
    });

    function processDir(dir) {
        const files = fs.readdirSync(dir);

        files.forEach(file => {
            const fullPath = path.join(dir, file);
            const stat = fs.statSync(fullPath);

            if (stat.isDirectory() && file !== 'node_modules' && file !== '.git') {
                processDir(fullPath);
            } else {
                const ext = path.extname(file);
                if (targetExts.includes(ext)) {
                    applyLicense(fullPath, ext);
                }
            }
        });
    }

    function applyLicense(filePath, ext) {
        const content = fs.readFileSync(filePath, 'utf8');
        const header = wrapInComment(template, ext);
        const identifier = template.trim().substring(0, 20);
        
        if (content.includes(identifier)) {
            console.log(`skipping: ${filePath} (Header already exists)`);
            return;
        }

        try {
            fs.writeFileSync(filePath, header + content);
            console.log(`licensed: ${filePath}`);
        } catch (err) {
            console.error(`error: could not write to ${filePath}`, err);
        }
    }

    console.log("Starting licensing process...");
    processDir(process.cwd());
    console.log("Done!");
}

run();