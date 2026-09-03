import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import lifecycle_cli


class ComponentAliasTests(unittest.TestCase):
    def setUp(self):
        self.components = [
            {
                "name": "models",
                "aliases": ["model", "mdl"],
                "commands": {"test": "echo {{ self.name }}"},
            },
            {
                "name": "deployments",
                "aliases": ["deploy", "deps"],
                "commands": {"test": "echo {{ self.name }}"},
            },
            {
                "name": "agents",
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
        config = {"components": self.components}

        with tempfile.TemporaryDirectory() as directory:
            config_path = Path(directory) / "components.json"
            config_path.write_text(json.dumps(config))

            with (
                patch.object(lifecycle_cli, "get_config_path", return_value=str(config_path)),
                patch.object(
                    sys,
                    "argv",
                    ["lifecycle_cli.py", "test", "deploy", "--dry-run", "--verbose"],
                ),
                patch("builtins.print") as print_mock,
            ):
                lifecycle_cli.main()

        output = "\n".join(str(call.args[0]) for call in print_mock.call_args_list)
        self.assertIn("echo deployments", output)


if __name__ == "__main__":
    unittest.main()
