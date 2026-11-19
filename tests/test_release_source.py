import importlib.util
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("release_source", ROOT / "scripts/check-release-source.py")
release_source = importlib.util.module_from_spec(spec)
spec.loader.exec_module(release_source)
SOURCE = "a" * 40
TAG_OBJECT = "b" * 40


class ReleaseSourceTests(unittest.TestCase):
    def runner(self, lookup_status=0, peeled=SOURCE, malformed=False):
        calls = []

        def run(command, **kwargs):
            calls.append(command)
            if command[1] == "check-ref-format":
                return subprocess.CompletedProcess(command, 0, "", "")
            if command[1] == "rev-parse":
                value = SOURCE if command[-1] == "HEAD^{commit}" else peeled
                return subprocess.CompletedProcess(command, 0, value + "\n", "")
            if command[1] == "ls-remote":
                output = "malformed" if malformed else TAG_OBJECT + "\trefs/tags/v0.1.0\n"
                return subprocess.CompletedProcess(command, lookup_status, output, "")
            if command[1] == "fetch":
                self.assertEqual(command[-1], TAG_OBJECT)
                return subprocess.CompletedProcess(command, 0, "", "")
            self.fail(command)

        return run, calls

    def test_matching_annotated_tag_fetches_observed_object(self):
        run, calls = self.runner()
        result = release_source.check_source("v0.1.0", run)
        self.assertTrue(result["existing_tag"])
        self.assertEqual(result["source_commit"], SOURCE)
        self.assertEqual(sum(command[1] == "fetch" for command in calls), 1)

    def test_absent_tag_allows_new_version_without_fetching(self):
        run, calls = self.runner(lookup_status=2)
        self.assertFalse(release_source.check_source("v0.1.0", run)["existing_tag"])
        self.assertFalse(any(command[1] == "fetch" for command in calls))

    def test_different_source_lookup_error_and_malformed_identity_fail(self):
        for options in ({"peeled": "c" * 40}, {"lookup_status": 128}, {"malformed": True}):
            with self.subTest(options=options), self.assertRaises(ValueError):
                release_source.check_source("v0.1.0", self.runner(**options)[0])


if __name__ == "__main__":
    unittest.main()
