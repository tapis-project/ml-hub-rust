import os
from pathlib import Path
import subprocess
import tempfile
import unittest


DEPLOYER = Path(__file__).resolve().parents[1] / "deployer"


class DeployerTest(unittest.TestCase):
    def setUp(self):
        self.temp_dir = tempfile.TemporaryDirectory()
        self.root = Path(self.temp_dir.name)
        self.bin_dir = self.root / "bin"
        self.bin_dir.mkdir()
        self.log = self.root / "commands.log"

        self._write_executable(
            self.root / "dev",
            """#!/usr/bin/env bash
echo "dev $*" >> "$DEPLOYER_TEST_LOG"
if [[ -n ${DEPLOYER_FAIL_DEV_MATCH:-} && "$*" == *"$DEPLOYER_FAIL_DEV_MATCH"* ]]; then
    exit 1
fi
""",
        )
        self._write_executable(
            self.bin_dir / "kubectl",
            """#!/usr/bin/env bash
echo "kubectl $*" >> "$DEPLOYER_TEST_LOG"

if [[ "$1 $2" == "get job" ]]; then
    job=$3
    if [[ "$*" == *"--ignore-not-found"* ]]; then
        if [[ ",${DEPLOYER_EXISTING_JOBS:-}," == *",$job,"* ]]; then
            echo "job.batch/$job"
        fi
        exit 0
    fi

    if [[ "$*" == *'type==\"Failed\"'* ]]; then
        if [[ ",${DEPLOYER_FAILED_JOBS:-}," == *",$job,"* ]]; then
            printf True
        fi
    else
        if [[ ",${DEPLOYER_FAILED_JOBS:-}," != *",$job,"* ]]; then
            printf True
        fi
    fi
fi
""",
        )

    def tearDown(self):
        self.temp_dir.cleanup()

    def _write_executable(self, path, content):
        path.write_text(content)
        path.chmod(0o755)

    def _run(self, stage, **environment):
        env = {
            **os.environ,
            "PATH": f"{self.bin_dir}:{os.environ['PATH']}",
            "DEPLOYER_TEST_LOG": str(self.log),
            "DEPLOYER_POLL_INTERVAL_SECONDS": "0",
            **environment,
        }

        return subprocess.run(
            [str(DEPLOYER), stage, str(self.root), "minikube"],
            capture_output=True,
            text=True,
            env=env,
            check=False,
        )

    def _commands(self):
        if not self.log.exists():
            return []

        return self.log.read_text().splitlines()

    def test_build_runs_every_build_in_order(self):
        result = self._run("build")

        self.assertEqual(0, result.returncode, result.stderr)
        self.assertEqual(
            [
                "dev build migrator",
                "dev build hpc-cluster-seeder",
                "dev build models",
                "dev build datasets",
                "dev build deployments",
                "dev build agents",
                "dev build endpoints",
                "dev build endpoint-router",
                "dev build artifact-ingester",
                "dev build artifact-publisher",
                "dev build model-deployment-controller",
                "dev build-extract hf-model-etl",
                "dev build-transform-load hf-model-etl",
                "dev build-extract hf-dataset-etl",
                "dev build-transform-load hf-dataset-etl",
            ],
            self._commands(),
        )

    def test_makes_dev_executable_before_running_stage(self):
        dev = self.root / "dev"
        dev.chmod(0o644)

        result = self._run("build-migrations")

        self.assertEqual(0, result.returncode, result.stderr)
        self.assertTrue(os.access(dev, os.X_OK))
        self.assertEqual("dev build migrator", self._commands()[0])

    def test_missing_dev_fails_during_chmod_before_running_stage(self):
        (self.root / "dev").unlink()

        result = self._run("build")

        self.assertNotEqual(0, result.returncode)
        self.assertIn("chmod", result.stderr)
        self.assertEqual([], self._commands())

    def test_start_runs_stages_and_readiness_checks_in_order(self):
        result = self._run("start")

        self.assertEqual(0, result.returncode, result.stderr)
        commands = self._commands()
        self.assertLess(commands.index("dev start nfs -t overlay minikube"), commands.index("dev start rabbit -t overlay minikube"))
        self.assertLess(commands.index("dev start traefik -t overlay minikube"), commands.index("dev run migrator -t overlay minikube"))
        self.assertLess(commands.index("dev run hpc-cluster-seeder -t overlay minikube"), commands.index("dev run migrator -t overlay minikube"))
        self.assertLess(commands.index("dev run migrator -t overlay minikube"), commands.index("dev start deployments -t overlay minikube"))
        self.assertLess(commands.index("dev run migrator -t overlay minikube"), commands.index("dev start endpoints -t overlay minikube"))
        self.assertLess(commands.index("dev run migrator -t overlay minikube"), commands.index("dev start endpoint-router -t overlay minikube"))
        self.assertLess(commands.index("dev start endpoints -t overlay minikube"), commands.index("dev start endpoint-router -t overlay minikube"))
        self.assertLess(commands.index("dev start endpoint-router -t overlay minikube"), commands.index("dev start envoy -t overlay minikube"))
        self.assertLess(commands.index("dev run migrator -t overlay minikube"), commands.index("dev start models -t overlay minikube"))
        self.assertLess(commands.index("dev start model-deployment-controller -t overlay minikube"), commands.index("dev run hf-model-etl -t overlay minikube"))
        self.assertLess(commands.index("dev start model-deployment-controller -t overlay minikube"), commands.index("dev start gatus -t overlay minikube"))
        self.assertLess(commands.index("dev start gatus -t overlay minikube"), commands.index("dev run hf-model-etl -t overlay minikube"))
        self.assertLess(commands.index("dev run hf-model-etl -t overlay minikube"), commands.index("dev run hf-dataset-etl -t overlay minikube"))
        self.assertIn("kubectl rollout status statefulset/mlhub-mongo-stateful-set --timeout=0s", commands)

        expected_components = [
            "models",
            "datasets",
            "deployments",
            "agents",
            "endpoints",
            "endpoint-router",
            "envoy",
            "gatus",
        ]

        for component in expected_components:
            self.assertEqual(
                1,
                commands.count(f"dev start {component} -t overlay minikube"),
            )

        self.assertEqual(
            1,
            commands.count(
                "kubectl rollout status deployment/mlhub-gatus-public-deployment --timeout=0s"
            ),
        )
        self.assertEqual(
            1,
            commands.count(
                "kubectl rollout status deployment/mlhub-gatus-internal-deployment --timeout=0s"
            ),
        )

        self.assertEqual(
            1,
            commands.count("dev run migrator -t overlay minikube"),
        )

    def test_existing_job_fails_before_aggregate_start(self):
        result = self._run("start", DEPLOYER_EXISTING_JOBS="mlhub-migrator")

        self.assertNotEqual(0, result.returncode)
        self.assertIn("already exists", result.stderr)
        self.assertFalse(any(command.startswith("dev ") for command in self._commands()))

    def test_stop_services_stops_gatus_before_monitored_services(self):
        result = self._run("stop-services")

        self.assertEqual(0, result.returncode, result.stderr)

        commands = self._commands()

        self.assertEqual("dev stop gatus -t overlay minikube", commands[0])
        self.assertLess(
            commands.index("dev stop gatus -t overlay minikube"),
            commands.index("dev stop model-deployment-controller -t overlay minikube"),
        )
        self.assertLess(
            commands.index("dev stop gatus -t overlay minikube"),
            commands.index("dev stop models -t overlay minikube"),
        )

    def test_existing_job_fails_before_grouped_job_stage(self):
        result = self._run("start-jobs", DEPLOYER_EXISTING_JOBS="mlhub-hf-model-etl")

        self.assertNotEqual(0, result.returncode)
        self.assertFalse(any(command.startswith("dev ") for command in self._commands()))

    def test_failed_migrator_stops_service_startup(self):
        result = self._run("start", DEPLOYER_FAILED_JOBS="mlhub-migrator")

        self.assertNotEqual(0, result.returncode)
        commands = self._commands()
        self.assertIn("dev run migrator -t overlay minikube", commands)
        self.assertNotIn("dev start models -t overlay minikube", commands)

    def test_existing_seed_job_fails_before_migrations(self):
        result = self._run(
            "start-migrations",
            DEPLOYER_EXISTING_JOBS="mlhub-hpc-cluster-seeder",
        )

        self.assertNotEqual(0, result.returncode)
        self.assertFalse(any(command.startswith("dev ") for command in self._commands()))

    def test_failed_build_stops_following_builds(self):
        result = self._run("build-services", DEPLOYER_FAIL_DEV_MATCH="build deployments")

        self.assertNotEqual(0, result.returncode)
        commands = self._commands()
        self.assertIn("dev build deployments", commands)
        self.assertNotIn("dev build agents", commands)


if __name__ == "__main__":
    unittest.main()
