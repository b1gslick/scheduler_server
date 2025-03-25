#!/usr/bin/env bash

################################################################################
# Variables                                                                         #
################################################################################

host=http://localhost:8080
slq_query_path=http://localhost:4545/query
version=v1
critical_path=critical_path.hurl
envs_path=vars.env

################################################################################
# Help                                                                         #
################################################################################

Help() {
  # Display Help
  echo "Test runner for hurl, "
  echo
  echo "Syntax: $1 --help"
  echo "options:"
  echo "--help, -h           Print this Help."
  echo
}

################################################################################
################################################################################
# Process the input options. Add options as needed.                            #
################################################################################
# Get the options

while [[ "$#" -gt 0 ]]; do
  case $1 in
  -h | --help)
    Help
    exit 0
    shift
    ;;
  *)
    echo "Unknown parameter passed: $1"
    exit 1
    ;;
  esac
  shift
done

# Main program                                                                 #
################################################################################
################################################################################

cat <<EOF
Run testing
EOF

check() {
  if [[ $? -ne 0 ]]; then
    echo "$1 ended with error"
    exit $?
  fi
}

## set env vars to the file
echo host=$host >$envs_path
echo slq_query_path=$slq_query_path >>$envs_path
echo version=$version >>$envs_path
echo a_string=$(openssl rand -hex 12) >>$envs_path
echo title=$(openssl rand -hex 60) >>$envs_path
echo content=$(openssl rand -hex 120) >>$envs_path
echo time=$RANDOM >>$envs_path

hurl --variables-file $envs_path --very-verbose --report-html report/ --test $critical_path
