import sys, os

def root_cwd() -> str:
    return os.path.abspath(os.getcwd()).replace("\\", "/") + "/"

def publish(path: str, dry_run: bool) -> None:
    print(f"Publishing {path}...")
    cmd = f"cd {path} && cargo publish --allow-dirty"
    if dry_run: cmd += " --dry-run"
    os.system(cmd)

def main():
    dry_run = True

    args = sys.argv[1:]

    if len(args) > 0:
        if args[0] == "--publish":
            dry_run = False

    cwd = root_cwd()
    utils_dir = cwd + "catalyzer-utils/"
    macros_dir = cwd + "catalyzer-macros/"
    core_dir = cwd + "catalyzer-core/"

    publish(utils_dir, dry_run)
    publish(macros_dir, dry_run)
    publish(core_dir, dry_run)
    publish(cwd, dry_run)

if __name__ == '__main__':
    try: 
        main()
    except KeyboardInterrupt:
        print("Exiting...")
        exit(0)
    except Exception as e:
        print(f"An error occurred: {e}")
        exit(1)
