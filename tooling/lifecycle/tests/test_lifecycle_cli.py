import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from tooling.lifecycle import lifecycle_cli


class ComponentAliasTests(unittest.TestCase):
    def setUp(self):
        self.components = [
            {
                "name": "models",
                "aliases": ["model", "mdl"],
                "labels": ["api"],
                "commands": {"test": "echo {{ self.name }}"},
            },
            {
                "name": "deployments",
                "aliases": ["deploy", "deps"],
                "labels": ["api", "local"],
                "commands": {"test": "echo {{ self.name }}"},
            },
            {
                "name": "agents",
                "labels": ["local"],
                "commands": {"test": "echo {{ self.name }}"},
            },
        ]

    def test_resolves_canonical_name(self):
        resolved = lifecycle_cli.resolve_components(self.components, ["models"])

        self.assertEqual([component["name"] for component in resolved], ["models"])

    def test_resolves_each_alias(self):
        for alias in ["model", "mdl"]:
            with self.subTest(alias=alias):
                resolved = lifecycle_cli.resolve_components(self.components, [alias])

                self.assertEqual([component["name"] for component in resolved], ["models"])

    def test_deduplicates_name_and_aliases(self):
        resolved = lifecycle_cli.resolve_components(
            self.components,
            ["models", "model", "mdl"],
        )

        self.assertEqual([component["name"] for component in resolved], ["models"])

    def test_no_identifiers_selects_all_components(self):
        resolved = lifecycle_cli.resolve_components(self.components, [])

        self.assertEqual(resolved, self.components)

    def test_preserves_configuration_order(self):
        resolved = lifecycle_cli.resolve_components(
            self.components,
            ["agents", "model", "deploy"],
        )

        self.assertEqual(
            [component["name"] for component in resolved],
            ["models", "deployments", "agents"],
        )

    def test_unknown_identifier_lists_names_and_aliases(self):
        with self.assertRaisesRegex(ValueError, "model.*deploy.*Received: 'unknown'"):
            lifecycle_cli.resolve_components(self.components, ["unknown"])

    def test_rejects_non_array_aliases(self):
        self.components[0]["aliases"] = "model"

        with self.assertRaisesRegex(ValueError, "must be an array"):
            lifecycle_cli.build_component_identifier_map(self.components)

    def test_rejects_non_string_and_empty_aliases(self):
        for alias in [None, 1, "", " "]:
            with self.subTest(alias=alias):
                self.components[0]["aliases"] = [alias]

                with self.assertRaisesRegex(ValueError, "must be non-empty strings"):
                    lifecycle_cli.build_component_identifier_map(self.components)

    def test_rejects_duplicate_alias_on_one_component(self):
        self.components[0]["aliases"] = ["model", "model"]

        with self.assertRaisesRegex(ValueError, "identifier 'model' is duplicated"):
            lifecycle_cli.build_component_identifier_map(self.components)

    def test_rejects_alias_to_alias_collision(self):
        self.components[1]["aliases"] = ["model"]

        with self.assertRaisesRegex(ValueError, "identifier 'model' is duplicated"):
            lifecycle_cli.build_component_identifier_map(self.components)

    def test_rejects_alias_to_name_collision(self):
        self.components[0]["aliases"] = ["deployments"]

        with self.assertRaisesRegex(ValueError, "identifier 'deployments' is duplicated"):
            lifecycle_cli.build_component_identifier_map(self.components)

    def test_rejects_duplicate_component_names(self):
        self.components[1]["name"] = "models"

        with self.assertRaisesRegex(ValueError, "identifier 'models' is duplicated"):
            lifecycle_cli.build_component_identifier_map(self.components)

    def test_cli_dry_run_accepts_an_alias(self):
        output = self.run_cli("test", "deploy", "--dry-run", "--verbose")

        self.assertIn("echo deployments", output)

    def test_cli_requires_an_explicit_selection(self):
        with self.assertRaises(SystemExit) as error:
            self.run_cli("test", "--dry-run")

        self.assertEqual(error.exception.code, 2)

    def test_cli_accepts_long_and_short_all_flags(self):
        for all_flag in ["--all", "-A"]:
            with self.subTest(all_flag=all_flag):
                output = self.run_cli(
                    "test",
                    all_flag,
                    "--dry-run",
                    "--verbose",
                )

                self.assertIn("echo models", output)
                self.assertIn("echo deployments", output)
                self.assertIn("echo agents", output)

    def test_cli_rejects_all_with_a_component_name_or_alias(self):
        for component_identifier in ["models", "model"]:
            with self.subTest(component_identifier=component_identifier):
                with self.assertRaises(SystemExit) as error:
                    self.run_cli(
                        "test",
                        component_identifier,
                        "--all",
                        "--dry-run",
                    )

                self.assertEqual(error.exception.code, 2)

    def test_cli_labels_without_components_select_matching_components(self):
        output = self.run_cli(
            "test",
            "--labels",
            "api",
            "--dry-run",
            "--verbose",
        )

        self.assertIn("echo models", output)
        self.assertIn("echo deployments", output)
        self.assertNotIn("echo agents", output)

    def test_cli_multiple_labels_require_every_label(self):
        output = self.run_cli(
            "test",
            "--labels",
            "api",
            "local",
            "--dry-run",
            "--verbose",
        )

        self.assertNotIn("echo models", output)
        self.assertIn("echo deployments", output)
        self.assertNotIn("echo agents", output)

    def test_cli_all_with_labels_selects_matching_components(self):
        output = self.run_cli(
            "test",
            "--all",
            "--labels",
            "api",
            "--dry-run",
            "--verbose",
        )

        self.assertIn("echo models", output)
        self.assertIn("echo deployments", output)
        self.assertNotIn("echo agents", output)

    def test_cli_explicit_components_are_filtered_by_labels(self):
        output = self.run_cli(
            "test",
            "models",
            "deployments",
            "--labels",
            "local",
            "--dry-run",
            "--verbose",
        )

        self.assertNotIn("echo models", output)
        self.assertIn("echo deployments", output)

    def test_cli_rejects_removed_initialization_flags(self):
        for initialization_flag in [
            "-i",
            "--initialize",
            "-s",
            "--skip-initialization",
        ]:
            with self.subTest(initialization_flag=initialization_flag):
                with self.assertRaises(SystemExit) as error:
                    self.run_cli("test", "models", initialization_flag)

                self.assertEqual(error.exception.code, 2)

    def test_cli_executes_command_without_initialization_state(self):
        config = {"components": self.components}

        with tempfile.TemporaryDirectory() as directory:
            config_path = Path(directory) / "components.json"
            config_path.write_text(json.dumps(config))

            with (
                patch.object(lifecycle_cli, "get_config_path", return_value=str(config_path)),
                patch.object(sys, "argv", ["lifecycle_cli.py", "test", "models"]),
                patch("os.system") as system_mock,
            ):
                lifecycle_cli.main()

        system_mock.assert_called_once_with("set -e; echo models")

    def run_cli(self, *arguments):
        config = {"components": self.components}

        with tempfile.TemporaryDirectory() as directory:
            config_path = Path(directory) / "components.json"
            config_path.write_text(json.dumps(config))

            with (
                patch.object(lifecycle_cli, "get_config_path", return_value=str(config_path)),
                patch.object(
                    sys,
                    "argv",
                    ["lifecycle_cli.py", *arguments],
                ),
                patch("builtins.print") as print_mock,
            ):
                lifecycle_cli.main()

        return "\n".join(str(call.args[0]) for call in print_mock.call_args_list)

    def test_configuration_path_resolves_from_repository_root(self):
        repository_root = Path(lifecycle_cli.__file__).resolve().parents[2]

        self.assertEqual(
            Path(lifecycle_cli.get_config_path()),
            repository_root / "components.json",
        )


if __name__ == "__main__":
    unittest.main()
