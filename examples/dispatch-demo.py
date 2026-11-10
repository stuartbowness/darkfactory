"""Exercise the real dispatcher with a local fake Codex process, without credentials."""
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "tools/darkfactory-dispatch"


def main():
    subprocess.run(["cargo", "build", "--locked", "--manifest-path", str(CRATE / "Cargo.toml")], check=True)
    # Cargo metadata respects CARGO_TARGET_DIR if the operator has configured it.
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version", "1",
         "--manifest-path", str(CRATE / "Cargo.toml")], text=True))
    binary = Path(metadata["target_directory"]) / "debug/darkfactory-dispatch"
    with tempfile.TemporaryDirectory(prefix="darkfactory-demo-") as folder:
        work = Path(folder)
        mock = work / "bin"
        mock.mkdir()
        (mock / "codex").symlink_to(CRATE / "tests/test_codex_mock.sh")
        environment = dict(os.environ)
        environment["PATH"] = str(mock) + os.pathsep + environment.get("PATH", "")
        # Do not inherit a test runner's fixture output or nested-dispatch settings.
        for name in ("CODEX_MOCK_PID_FILE", "DARKFACTORY_DISPATCH_DEPTH"):
            environment.pop(name, None)
        print("Offline fixture demo: no model calls; usage values are synthetic.", flush=True)
        for mode, expected, code in [("success", "completed", 0),
                                     ("failed", "failed", 1), ("sleep", "timed_out", 1)]:
            environment["CODEX_MOCK_MODE"] = mode
            result = subprocess.run(
                [str(binary), "--model", "offline-fixture", "--prompt", "Run the fixture",
                 "--cwd", str(work), "--output", str(work / (mode + ".md")),
                 "--timeout", "1" if mode == "sleep" else "10", "--grace", "1",
                 "--json"], env=environment, capture_output=True, text=True, timeout=20,
            )
            report = json.loads(result.stdout)
            if report["status"] != expected or result.returncode != code:
                raise RuntimeError(f"{mode}: unexpected result: {report}")
            print(f"{mode}: status={report['status']}, exit={result.returncode}, "
                  f"output_exists={str(report['output_exists']).lower()}")
    print("All three process-lifecycle checks passed.")


if __name__ == "__main__":
    main()
