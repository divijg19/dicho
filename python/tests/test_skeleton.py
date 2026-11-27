"""Dependency-free smoke test for the Python package skeleton."""

import unittest

from dicho_data import __version__


class SkeletonTest(unittest.TestCase):
    def test_version_matches_foundation_release(self):
        self.assertEqual(__version__, "0.3.0")


if __name__ == "__main__":
    unittest.main()
