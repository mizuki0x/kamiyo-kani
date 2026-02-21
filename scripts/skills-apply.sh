#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
skills_root="${repo_root}/skills"

usage() {
  cat <<'USAGE'
Usage:
  ./scripts/skills-apply.sh --list

  ./scripts/skills-apply.sh \
    --skill <skill-id> \
    [--version <version>|--version latest] \
    [--files-modified <csv>] \
    [--target-repo <path>] \
    [--set KEY=VALUE]... \
    [--output <path>] \
    [--print]

  ./scripts/skills-apply.sh \
    --skill <skill-id> \
    [--version <version>] \
    --patch-file <patch-path> \
    [--target-repo <path>]

Examples:
  ./scripts/skills-apply.sh --list

  ./scripts/skills-apply.sh \
    --skill solana-account-generator \
    --version 1.0 \
    --files-modified crates/kamiyo-kani/src/account_info.rs,crates/kamiyo-kani/tests/account_info_verify.rs

  ./scripts/skills-apply.sh \
    --skill add-solana-token-proof \
    --version 1.0 \
    --patch-file /tmp/llm.patch
USAGE
}

list_skills() {
  if [[ ! -d "${skills_root}" ]]; then
    echo "No skills directory found at ${skills_root}" >&2
    exit 1
  fi

  while IFS= read -r -d '' dir; do
    skill_id="$(basename "$(dirname "${dir}")")"
    skill_version="$(basename "${dir}")"
    skill_doc="${dir}/SKILL.md"
    summary=""
    if [[ -f "${skill_doc}" ]]; then
      summary="$(awk 'NF { if ($0 !~ /^#/) { print; exit } }' "${skill_doc}")"
    fi
    if [[ -n "${summary}" ]]; then
      printf '%s@%s - %s\n' "${skill_id}" "${skill_version}" "${summary}"
    else
      printf '%s@%s\n' "${skill_id}" "${skill_version}"
    fi
  done < <(find "${skills_root}" -mindepth 2 -maxdepth 2 -type d -print0) | sort
}

normalize_key() {
  local key="$1"
  key="$(printf '%s' "${key}" | tr '[:lower:]-.' '[:upper:]__')"
  printf '%s' "${key}" | tr -cd 'A-Z0-9_'
}

trim() {
  local value="$1"
  value="${value#"${value%%[![:space:]]*}"}"
  value="${value%"${value##*[![:space:]]}"}"
  printf '%s' "${value}"
}

list_mode=0
skill_id=""
skill_version="latest"
files_modified_csv=""
target_repo="${repo_root}"
output_file=""
patch_file=""
print_output=0

set_keys=()
set_values=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --list)
      list_mode=1
      shift
      ;;
    --skill)
      if [[ $# -lt 2 ]]; then
        echo "Missing value for --skill." >&2
        exit 1
      fi
      skill_id="$2"
      shift 2
      ;;
    --version)
      if [[ $# -lt 2 ]]; then
        echo "Missing value for --version." >&2
        exit 1
      fi
      skill_version="$2"
      shift 2
      ;;
    --files-modified)
      if [[ $# -lt 2 ]]; then
        echo "Missing value for --files-modified." >&2
        exit 1
      fi
      files_modified_csv="$2"
      shift 2
      ;;
    --target-repo)
      if [[ $# -lt 2 ]]; then
        echo "Missing value for --target-repo." >&2
        exit 1
      fi
      target_repo="$2"
      shift 2
      ;;
    --set)
      if [[ $# -lt 2 ]]; then
        echo "Missing value for --set." >&2
        exit 1
      fi
      kv="$2"
      if [[ "${kv}" != *"="* ]]; then
        echo "Invalid --set value: ${kv}. Expected KEY=VALUE." >&2
        exit 1
      fi
      raw_key="${kv%%=*}"
      raw_value="${kv#*=}"
      norm_key="$(normalize_key "${raw_key}")"
      if [[ -z "${norm_key}" ]]; then
        echo "Invalid --set key: ${raw_key}" >&2
        exit 1
      fi
      set_keys+=("${norm_key}")
      set_values+=("${raw_value}")
      shift 2
      ;;
    --output)
      if [[ $# -lt 2 ]]; then
        echo "Missing value for --output." >&2
        exit 1
      fi
      output_file="$2"
      shift 2
      ;;
    --patch-file)
      if [[ $# -lt 2 ]]; then
        echo "Missing value for --patch-file." >&2
        exit 1
      fi
      patch_file="$2"
      shift 2
      ;;
    --print)
      print_output=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
done

if [[ ${list_mode} -eq 1 ]]; then
  list_skills
  exit 0
fi

if [[ -z "${skill_id}" ]]; then
  echo "--skill is required." >&2
  usage
  exit 1
fi

if [[ ! -d "${target_repo}" ]]; then
  echo "Target repo does not exist: ${target_repo}" >&2
  exit 1
fi

if ! target_repo="$(cd "${target_repo}" && pwd)"; then
  echo "Unable to resolve target repo: ${target_repo}" >&2
  exit 1
fi
if ! git -C "${target_repo}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "Target repo is not a git work tree: ${target_repo}" >&2
  exit 1
fi

skill_base_dir="${skills_root}/${skill_id}"
if [[ ! -d "${skill_base_dir}" ]]; then
  echo "Skill not found: ${skill_id}" >&2
  echo "Available skills:" >&2
  list_skills >&2
  exit 1
fi

if [[ "${skill_version}" == "latest" ]]; then
  skill_version="$(find "${skill_base_dir}" -mindepth 1 -maxdepth 1 -type d -exec basename {} \; | sort -V | tail -n 1)"
fi

if [[ -z "${skill_version}" ]]; then
  echo "No versions found for skill: ${skill_id}" >&2
  exit 1
fi

skill_dir="${skill_base_dir}/${skill_version}"
skill_doc="${skill_dir}/SKILL.md"
prompt_template="${skill_dir}/PROMPT.md.tmpl"

if [[ ! -f "${skill_doc}" || ! -f "${prompt_template}" ]]; then
  echo "Skill is missing required files (SKILL.md and PROMPT.md.tmpl): ${skill_dir}" >&2
  exit 1
fi

timestamp_utc="$(date -u +%Y%m%dT%H%M%SZ)"
run_dir="${target_repo}/.skills/runs"
mkdir -p "${run_dir}"

if [[ -z "${output_file}" ]]; then
  output_file="${run_dir}/${skill_id}-${skill_version}-${timestamp_utc}.md"
else
  mkdir -p "$(dirname "${output_file}")"
fi

formatted_files=""
if [[ -n "${files_modified_csv}" ]]; then
  IFS=',' read -r -a raw_files <<< "${files_modified_csv}"
  for item in "${raw_files[@]}"; do
    item="$(trim "${item}")"
    if [[ -n "${item}" ]]; then
      formatted_files+="- ${item}"$'\n'
    fi
  done
fi

if [[ -z "${formatted_files}" ]]; then
  formatted_files="- (not provided)"
else
  formatted_files="${formatted_files%$'\n'}"
fi

export SKILL_ID="${skill_id}"
export SKILL_VERSION="${skill_version}"
export TARGET_REPO="${target_repo}"
export FILES_MODIFIED="${formatted_files}"
export TIMESTAMP_UTC="${timestamp_utc}"

for i in "${!set_keys[@]}"; do
  key="${set_keys[$i]}"
  value="${set_values[$i]}"
  export "${key}=${value}"
done

perl -pe 's/\{\{([A-Z0-9_]+)\}\}/exists $ENV{$1} ? $ENV{$1} : $&/ge' "${prompt_template}" > "${output_file}"

meta_file="${output_file%.md}.meta"
{
  echo "skill_id=${skill_id}"
  echo "skill_version=${skill_version}"
  echo "target_repo=${target_repo}"
  echo "timestamp_utc=${timestamp_utc}"
  echo "prompt_file=${output_file}"
  if [[ -n "${files_modified_csv}" ]]; then
    echo "files_modified_csv=${files_modified_csv}"
  fi
} > "${meta_file}"

if [[ -n "${patch_file}" ]]; then
  if [[ ! -f "${patch_file}" ]]; then
    echo "Patch file not found: ${patch_file}" >&2
    exit 1
  fi
  git -C "${target_repo}" apply --check "${patch_file}"
  git -C "${target_repo}" apply "${patch_file}"
  echo "Applied patch: ${patch_file}"
fi

echo "Rendered prompt: ${output_file}"
echo "Run metadata: ${meta_file}"

if [[ ${print_output} -eq 1 ]]; then
  echo ""
  cat "${output_file}"
fi
