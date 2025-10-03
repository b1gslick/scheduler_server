#!/usr/bin/env bash

################################################################################
# Variables                                                                         #
################################################################################

host=http://localhost:8080
sql_query_path=http://localhost:4545/query
version=v1
file_path=critical_path.hurl
envs_path=vars.env
verbosity="--very-verbose"
report="--report-html report/"

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
  echo "--url, -u            Set url."
  echo "--sqp, -s            Set url for sql query service."
  echo "--file, -f           Set test file path."
  echo "--env, -e            Set env vars file path."
  echo "--verbosity, -v      Set verbosity level."
  echo "--xml, -x            Set xml report type."
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
  -u | --url)
    host=${2}
    shift
    ;;
  -s | --sqp)
    sql_query_path=${2}
    shift
    ;;
  -f | --file)
    file_path=${2}
    shift
    ;;
  -e | --env)
    envs_path=${2}
    shift
    ;;
  -v | --verbosity)
    verbosity=${2}
    shift
    ;;
  -x | --xml)
    report="--report-junit report.xml"
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
>> URL:       $host
>> Test file: $file_path
>> Env vars : $envs_path

EOF

check() {
  if [[ $? -ne 0 ]]; then
    echo "$1 ended with error"
    exit $?
  fi
}

## set env vars to the file
echo host=$host >$envs_path
echo sql_query_path=$sql_query_path >>$envs_path
echo version=$version >>$envs_path
echo a_string=$(openssl rand -hex 12) >>$envs_path

echo email_1="$(openssl rand -hex 12)@$(openssl rand -hex 12).iv" >>$envs_path
echo email_2="$(openssl rand -hex 12)@$(openssl rand -hex 12).iv" >>$envs_path

echo title=$(openssl rand -hex 60) >>$envs_path
echo new_title=$(openssl rand -hex 60) >>$envs_path

echo content=$(openssl rand -hex 120) >>$envs_path
echo new_content=$(openssl rand -hex 120) >>$envs_path

time=$RANDOM
new_time=$RANDOM
echo time=$time >>$envs_path
echo new_time=$new_time >>$envs_path

echo expected=$((time * 60)) >>$envs_path
echo new_expected=$((new_time * 60)) >>$envs_path

echo limit=10000 >>$envs_path
echo offset=0 >>$envs_path

hurl --variables-file $envs_path \
  $verbosity \
  $report \
  --test $file_path
