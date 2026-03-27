#/!bin/bash

year=$1
day_number=$2

function help_page {
  echo "
Usage: create_aoc_project.sh <year> <day_number>
  "
}


#SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Folder relative to script location
#SOURCE_DIR="$SCRIPT_DIR/my_folder"
#DEST_DIR="$SCRIPT_DIR/my_folder_copy"

# ------------------------------------------------------------------------------------------
# -Check the first argument-----------------------------------------------------------------
# Ensure it's not empty.
if [[ $year == "" ]]; then
  echo "Missing arguments!"
  help_page
  exit
fi

# Ensure it's an integer
if ! [[ "$year" =~ ^[0-9]+$ ]]; then
  echo "Error: year must be a number"
  exit 1
fi

# ------------------------------------------------------------------------------------------
# -Check and format the second argument-----------------------------------------------------
# Ensure it's not empty.
if [[ $day_number == "" ]]; then
  echo "Missing arguments!"
  help_page
  exit
fi

# Ensure it's an integer
if ! [[ "$day_number" =~ ^[0-9]+$ ]]; then
  echo "Error: day_number must be a number"
  exit 1
fi

# Ensure that it's a valid aoc day.
if (( $day_number < 1 || $day_number > 25 )) then
  echo "day_number must be a valid aoc day. Only 1..25 is allowed."
  help_page
  exit 
fi

# Update format to ensure it is made of two digits adding a leading zero if neccessary.
day_number=$(printf "%02d" "$day_number")

# ------------------------------------------------------------------------------------------
# -Prepare file paths-----------------------------------------------------------------------
script_directory=$(dirname "$BASH_SOURCE")
target_directory="$script_directory/$year"
project_path="$target_directory/day_$day_number"
template_directory="$script_directory/template"


# ------------------------------------------------------------------------------------------
# -Ensure the year folder exists------------------------------------------------------------
mkdir -p $target_directory


# ------------------------------------------------------------------------------------------
# -Ensure no project already exists at the target directory---------------------------------
if [ -d "$project_path" ]; then
  echo "Something already exists at $project_path!"
  echo "Please double check the year and day_number"
  help_page
  exit
fi


# ------------------------------------------------------------------------------------------
# -Copy template to the target_directory----------------------------------------------------
cp -r $template_directory $project_path
echo "Copied template to: $project_path"


# ------------------------------------------------------------------------------------------
# -Update the project name to match the day number------------------------------------------
project_name="Advent_of_code_${year}_${day_number}"
cargo_folder="$project_path/Cargo.toml"
sed -i -e "s|PROJECTNAME|$project_name|g" $cargo_folder
