"""Render the embedded Rusk API at MkDocs build time."""
import os
from pathlib import Path
import subprocess

from mkdocs.exceptions import PluginError
from mkdocs.structure.files import File


ROOT = Path(__file__).resolve().parents[1]


def on_files(files, config):
    revision = os.environ.get('GITHUB_SHA', 'main')
    source_url = f"{config.repo_url.rstrip('/')}/blob/{revision}/lib"
    try:
        api = subprocess.run(
            ['cargo', 'run', '--quiet', '--locked', '--', 'doc', '--source-url', source_url],
            cwd=ROOT, check=True, stdout=subprocess.PIPE,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise PluginError(f"Rusk API reference generation failed: {error}") from error
    files.append(File.generated(config, 'library/api.md', content=api.stdout))
    return files
