"""Installer checks use a disposable prefix, never the real home directory."""
import pathlib
import shutil
import subprocess
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]


class InstallTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="darkfactory-install-")
        self.addCleanup(self.temp.cleanup)
        self.base = pathlib.Path(self.temp.name).resolve()
        self.repo = self.base / "checkout with spaces"
        (self.repo / "skills/first").mkdir(parents=True)
        (self.repo / "skills/last").mkdir()
        (self.repo / "agents").mkdir()
        for name in ("first", "last"):
            (self.repo / f"skills/{name}/SKILL.md").write_text("# Fixture\n")
        (self.repo / "agents/worker.md").write_text("# Fixture\n")
        shutil.copy2(ROOT / "install.sh", self.repo / "install.sh")
        self.prefix = self.base / "test home"

    def run_install(self, *args):
        return subprocess.run(
            ["sh", str(self.repo / "install.sh"), "--prefix", str(self.prefix), *args],
            text=True, capture_output=True, check=False,
        )

    def test_install_and_repeat_preserve_link_targets(self):
        first = self.run_install()
        self.assertEqual(first.returncode, 0, first.stderr)
        for client in (".claude", ".codex", ".agents"):
            link = self.prefix / client / "skills/first"
            self.assertTrue(link.is_symlink())
            self.assertEqual(link.resolve(), self.repo / "skills/first")
        self.assertEqual((self.prefix / ".claude/agents/worker.md").resolve(),
                         self.repo / "agents/worker.md")
        second = self.run_install()
        self.assertEqual(second.returncode, 0, second.stderr)
        self.assertNotIn("linked:", second.stdout)

    def test_dry_run_does_not_create_prefix(self):
        result = self.run_install("--dry-run")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse(self.prefix.exists())

    def test_legacy_migration_uses_safe_installer(self):
        shutil.copy2(ROOT / "migrate.sh", self.repo / "migrate.sh")
        result = subprocess.run(
            ["sh", str(self.repo / "migrate.sh"), "--prefix", str(self.prefix), "--dry-run"],
            text=True, capture_output=True, check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse(self.prefix.exists())
        self.assertIn("would link:", result.stdout)

    def test_conflicts_leave_all_other_destinations_untouched(self):
        for kind in ("file", "directory", "foreign_link", "broken_link"):
            with self.subTest(kind=kind):
                if self.prefix.exists():
                    shutil.rmtree(self.prefix)
                conflict = self.prefix / ".agents/skills/last"
                conflict.parent.mkdir(parents=True)
                if kind == "file":
                    conflict.write_text("user content")
                elif kind == "directory":
                    conflict.mkdir()
                    (conflict / "keep").write_text("user content")
                else:
                    target = self.base / ("missing" if kind == "broken_link" else "foreign")
                    if kind == "foreign_link":
                        target.mkdir(exist_ok=True)
                    conflict.symlink_to(target)
                result = self.run_install()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("Conflict:", result.stderr)
                self.assertFalse((self.prefix / ".claude").exists())
                if kind == "file":
                    self.assertEqual(conflict.read_text(), "user content")
                elif kind == "directory":
                    self.assertEqual((conflict / "keep").read_text(), "user content")
                else:
                    self.assertEqual(conflict.readlink(), target)

    def test_legacy_link_chain_is_idempotent(self):
        for client in (".claude", ".codex"):
            (self.prefix / client / "skills").mkdir(parents=True)
        original = self.prefix / ".claude/skills/first"
        original.symlink_to(self.repo / "skills/first")
        chained = self.prefix / ".codex/skills/first"
        chained.symlink_to(original)
        result = self.run_install()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(chained.readlink(), original)

    def test_missing_source_fails_before_installing(self):
        (self.repo / "skills/last/SKILL.md").unlink()
        result = self.run_install()
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(self.prefix.exists())

    def test_parent_conflict_fails_before_installing(self):
        self.prefix.mkdir()
        (self.prefix / ".codex").write_text("user content")
        result = self.run_install()
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse((self.prefix / ".claude").exists())

    def test_agent_conflict_is_detected_before_skill_changes(self):
        destination = self.prefix / ".claude/agents/worker.md"
        destination.parent.mkdir(parents=True)
        destination.write_text("my agent")
        result = self.run_install()
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(destination.read_text(), "my agent")
        self.assertFalse((self.prefix / ".claude/skills").exists())


if __name__ == "__main__":
    unittest.main()
