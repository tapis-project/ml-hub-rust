from pathlib import Path
import subprocess
import unittest


GATUS_ROOT = Path(__file__).resolve().parents[1]
TRAEFIK_ROOT = GATUS_ROOT.parent / "traefik" / "overlays"


class GatusConfigurationTest(unittest.TestCase):
    def test_public_configuration_is_sanitized_and_api_only(self):
        configuration = (GATUS_ROOT / "base" / "public-config.yaml").read_text()

        self.assertIn("metrics: false", configuration)
        self.assertEqual(5, configuration.count("group: MLHub APIs"))
        self.assertNotIn("Model Deployment Controller", configuration)
        self.assertNotIn("Infrastructure Reachability", configuration)
        self.assertIn("hide-url: true", configuration)
        self.assertIn("hide-hostname: true", configuration)
        self.assertIn("hide-port: true", configuration)
        self.assertIn("hide-conditions: true", configuration)
        self.assertIn("hide-errors: true", configuration)

    def test_internal_configuration_contains_every_required_check(self):
        configuration = (GATUS_ROOT / "base" / "internal-config.yaml").read_text()

        expected_checks = [
            "Models API",
            "Datasets API",
            "Deployments API",
            "Agents API",
            "Endpoints API",
            "Model Deployment Controller",
            "MongoDB Reachability",
            "RabbitMQ Reachability",
            "NFS Reachability",
            "Traefik Reachability",
            "Envoy Reachability",
            "Endpoint Router Reachability",
        ]

        self.assertIn("metrics: true", configuration)

        for check in expected_checks:
            self.assertIn(f"name: {check}", configuration)

    def test_every_overlay_renders_two_pinned_persistent_instances(self):
        for environment in ["minikube", "tapisdev", "tapisstage", "tapisprod"]:
            result = subprocess.run(
                ["kubectl", "kustomize", str(GATUS_ROOT / "overlays" / environment)],
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertEqual(0, result.returncode, result.stderr)
            self.assertEqual(2, result.stdout.count("kind: Deployment"))
            self.assertEqual(2, result.stdout.count("kind: PersistentVolumeClaim"))
            self.assertEqual(2, result.stdout.count("image: ghcr.io/twin/gatus:v5.37.0"))
            self.assertEqual(2, result.stdout.count("storage: 1Gi"))

    def test_only_public_gatus_is_routed_by_traefik(self):
        hosts = {
            "minikube": "mlhub-status.local.develop.tapis.io",
            "tapisdev": "mlhub-status.develop.tapis.io",
            "tapisstage": "mlhub-status.staging.tapis.io",
            "tapisprod": "mlhub-status.tapis.io",
        }

        for environment, hostname in hosts.items():
            configuration = (TRAEFIK_ROOT / environment / "dynamic-config.yaml").read_text()

            self.assertIn(f"Host(`{hostname}`)", configuration)
            self.assertIn("url: http://mlhub-gatus-public-service:8080", configuration)
            self.assertNotIn("mlhub-gatus-internal-service", configuration)


if __name__ == "__main__":
    unittest.main()
