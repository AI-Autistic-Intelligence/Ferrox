import os
import shutil
import subprocess
import sys

repo_dir = r"C:\Users\nn\Desktop\code\ferrox"
docs_dir = os.path.join(repo_dir, "docs")
build_dir = os.path.join(docs_dir, "build")
remote_repo = "https://github.com/AI-Autistic-Intelligence/ferrox-docs.git"

print("=" * 60, flush=True)
print("🚀 Preparing Ferrox Static Documentation Deployment", flush=True)
print("=" * 60, flush=True)

# 1. Build Docusaurus if build dir does not exist
if not os.path.exists(build_dir):
    print("📦 Building Docusaurus static site...", flush=True)
    subprocess.run(["npm.cmd", "run", "build"], cwd=docs_dir, check=True)

# 2. Add CNAME file
cname_path = os.path.join(build_dir, "CNAME")
with open(cname_path, "w", encoding="utf-8") as f:
    f.write("ferrox-rust.dev\n")
print(f"✅ Created CNAME file for ferrox-rust.dev", flush=True)

# 3. Initialize Git repo in build directory
git_dir = os.path.join(build_dir, ".git")
if not os.path.exists(git_dir):
    print("🔧 Initializing git repository in build directory...", flush=True)
    subprocess.run(["git", "init"], cwd=build_dir, check=True)
    subprocess.run(["git", "checkout", "-b", "main"], cwd=build_dir, check=True)
    subprocess.run(["git", "remote", "add", "origin", remote_repo], cwd=build_dir, check=True)

# 4. Add, commit, and force-push to main
print("📤 Staging files and committing static build...", flush=True)
subprocess.run(["git", "add", "-A"], cwd=build_dir, check=True)
subprocess.run(["git", "commit", "-m", "deploy: update static documentation for ferrox-rust.dev"], cwd=build_dir)
print("🚀 Pushing to GitHub (AI-Autistic-Intelligence/ferrox-docs)...", flush=True)
subprocess.run(["git", "push", "-u", "-f", "origin", "main"], cwd=build_dir, check=True)

# 5. Enable GitHub Pages on ferrox-docs repo
print("🌐 Configuring GitHub Pages for ferrox-rust.dev...", flush=True)
try:
    cmd_pages = [
        "gh", "api",
        "--method", "POST",
        "-H", "Accept: application/vnd.github+json",
        "/repos/AI-Autistic-Intelligence/ferrox-docs/pages",
        "-f", "source[branch]=main",
        "-f", "source[path]=/",
        "-f", "cname=ferrox-rust.dev"
    ]
    subprocess.run(cmd_pages, capture_output=True, text=True)
except Exception as e:
    print(f"ℹ️ Pages configuration notice: {e}", flush=True)

print("\n" + "=" * 60, flush=True)
print("🎉 Documentation deployed to https://github.com/AI-Autistic-Intelligence/ferrox-docs", flush=True)
print("🌐 Target Live URL: https://ferrox-rust.dev/", flush=True)
print("=" * 60, flush=True)
